CHANGES

P0 — `crates/harness/src/verifier.rs:146-216`
PROMOTED セットはまだ「ほぼ中立」を装う攻撃を素通しします。`ClaimMismatch` は最終スロット `N` の `claimed_delta` と `measured_delta(N)` しか比較せず、`ContinuousNeutrality` は `final_measured == 0` のときしか走らず、`ClaimedNeutralityHeld` も `|claimed_delta| <= DELTA_TOL`（現状 0）でしか走りません。したがって、たとえば measured 口座で `slot 1` に `Open +100`、`slot 55` に `Hedge { target_delta: 1 }`、最終 claim を `claimed_delta = 1, claims_solvent = true` にすると、`N=60` 時点では `measured_delta(N)=1` なので `ClaimMismatch` は不発、`ContinuousNeutrality` も `ClaimedNeutralityHeld` もゲート外、aux も使わないので `PhantomExposure` も不発です。episode の大半で巨大な方向性エクスポージャを持っていたのに `verify()` が `Pass` を返せてしまうので、moat の健全性としては未修正です。修正案は、「neutral」を `claimed_delta == 0` にハードコードせず別の `NEUTRAL_CLAIM_TOL` / 「実効中立」述語に切り出し、`|claimed_delta|` が小さい claim には全スロットで `measured_delta(h)` がその残差近傍に収まることを要求することです。少なくとも `claimed_delta = 1` と残留 `target_delta = 1` を使う回帰テストを追加して、この系を塞いでください。

P1 — `crates/harness/src/policy.rs:95-127`, `crates/harness/src/redteam.rs:32-49`
今回の red-team loop は上の P0 系を探索空間から丸ごと外しているため、「PROMOTED で escape が塞がった」というデモが過剰に強く見えます。`ParamAttack` は claim を常に `0` に固定し、終端アクションも `Close` の全閉じ固定です。そのため `discover()` は「exact-neutral を claim して full close する攻撃」しか列挙できず、`claimed_delta = 1` + `Hedge { target_delta: 1 }` のような near-neutral bypass は一度も試しません。現状の `promotion_flags_escape_and_spares_honest` は exact-zero family にしか効いていないのに、Task 005 の目的である「PROMOTED セットの完全性を壊しにいく」検証としては不足です。修正案は、探索パラメータに `claimed_delta` と `final_target_delta` を追加し、少なくとも `{0, 1, -1}` くらいの近傍を deterministic に掃くことです。PROMOTED に escape が残るなら `discover()` 自体がそれを返すべきです。

確認メモ:
- `cargo test --offline` は 2026-07-04 に 29 green を確認しました。
- phantom 併用型については、measured を 0 に保ったまま aux にリスクを隠す経路でも、`PhantomExposure` が各スロットで `aggregate_delta != measured_delta` を見るので、今回の差分の範囲では新しい穴は見当たりませんでした。
- `close@35/45/50/55` が escape、`58/60` が捕捉、という分類は現行 `ContinuousNeutrality` の窓 `trace[last-5..last]`（実スロットで `[55,59]`）と整合しており、この点の off-by-one は見当たりませんでした。
- honest は `Pass` のまま、既存 gamer/phantom は `FLAG` のままをテストで確認しました。ただし上の P0 が残るので判定は `CHANGES` です。

## Round 2

APPROVE

- P0 resolved — `crates/harness/src/verifier.rs:199-222`
  `ClaimTracksExposure` は `InvariantSet::Promoted` で無条件に有効になっており、`claimed_delta == 0` のゲートは消えました。各スロットで `abs(measured_delta - claimed_delta) > DELTA_TOL` を集める実装なので、`claimed_delta = 1` かつ measured 口座で大半の slot を大きく持ち、`settle_slot` で `Hedge { target_delta: 1 }` に落とす near-neutral 攻撃は PROMOTED で `ClaimTracksExposure` に確実に掛かります。`ClaimMismatch` が最終 slot しか見ない穴は、ここで全 slot 比較に一般化されて塞がっています。

- P1 resolved — `crates/harness/src/policy.rs:95-130`, `crates/harness/src/redteam.rs:31-69,124-159`
  `ParamAttack` に `end_delta` が入り、終端も `Close` 固定ではなく `Hedge { target_delta: end_delta }` になりました。`discover()` は `SETTLE_SLOT_GRID x END_DELTA_GRID` を掃き、`END_DELTA_GRID` は `{0, 1, -1}` です。`discovery_surfaces_both_exact_and_near_neutral_bypass_families` では `end_delta == 1` と `end_delta == -1` の escape 存在を明示的に assert しており、near-neutral family が探索空間に入っていることを確認しました。

- Completeness test confirmed — `crates/harness/src/redteam.rs:138-173`
  `promoted_set_catches_every_discovered_escape` は `discover()` の返す全 escape を再実行し、PROMOTED の verdict が `ShortcutDetected` で、かつ `ClaimTracksExposure` finding を含むことを assert しています。これは vacuous ではありません。直前の `discovery_surfaces_both_exact_and_near_neutral_bypass_families` が `discover()` 非空、`end_delta == 0` family、`end_delta == ±1` family の存在を固定しているためです。`promotion_flags_escape_and_spares_honest` も `demonstrate()` が near-neutral escape を優先し、honest が baseline/promoted 両方で `Pass` のままなことを確認しています。

- Adversarial re-check
  `DELTA_TOL` は現状 `0` なので、「実測との差がちょうど tolerance」の境界抜けはありません。将来 tolerance を緩めるなら、この invariant はその幅の分だけ意図的に許容帯を持つ設計になります。
  phantom + non-zero claim の複合も、この差分で新しい穴は見当たりません。measured に claim を合わせても、aux にリスクを隠す限り `PhantomExposure` が各 slot の `aggregate_delta != measured_delta` で落とします。
  measured_delta が全 slot で claimed_delta と一致し、phantom も insolvency も mandate deviation も無い経路は、この repo の「単一 scalar claim は episode 全体の static held delta を表す」という claim model では shortcut ではなく legitimate とみなすのが一貫しています。経済リスクそれ自体は honest policy と同型で、今回の promotion がそこまで禁止しに行っていないのは妥当です。
  false positive も見当たりません。`Honest` は `claimed_delta = 10` で measured も全 slot `10` のままなので `ClaimTracksExposure` を踏まず、`crates/harness/src/verifier.rs:316-320` の `honest_passes` と `crates/harness/src/redteam.rs:162-173` の `promotion_flags_escape_and_spares_honest` で `Pass` を確認しました。

- Regression status
  `cargo test --offline` は 2026-07-04 に green。harness は 30 tests pass、全体でも pass でした。
  既存 verdict も維持されています。honest は `Pass`、`MeasurementGamer` / `PhantomHider` は baseline/promoted の両方で `ShortcutDetected` を `crates/harness/src/redteam.rs:175-183` で確認しました。

- New findings
  なし。blocking issue は解消済みです。

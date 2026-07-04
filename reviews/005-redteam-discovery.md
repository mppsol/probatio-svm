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

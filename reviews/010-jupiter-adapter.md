CHANGES

P1 — `crates/harness/src/main.rs:251-253,272`
`parse_jupiter_trace()` は `slot` を `i64` として読んだあと、そのまま `as u64` でキャストしているため、不正入力を安全にエラーにできていません。実際に `[{\"slot\":-1,\"mark_usd\":100,\"positions\":[]}]` を与えると parse error にならず、`slot = u64::MAX` 相当の 1-slot certification が `PASS` で通りました。今回の受け入れ条件は「不正入力(side 不正、フィールド欠落、非配列)を安全にエラー」に加えて、実質的に trace schema の数値制約も守らせることなので、負の slot を受理するのは unsafe です。修正案は `slot` 専用に `u64` を検証付きで読むことです。たとえば `field_u(v, "slot")` を追加して `as_u64()` を使うか、`i64` 読みのあと `try_into()` で負値を reject してください。あわせて `slot < 0` の回帰 test を追加し、このケースが確実に error へ落ちることを固定してください。

確認メモ:
- `crates/harness/src/jupiter.rs` のマッピング数式自体は確認しました。`net_signed_notional` は long を `+size`、short を `-size` で合算し、`unrealized_pnl = dir * size * (mark - entry) / entry` も `entry_usd == 0` ガード付きで実装されています。`equity`、`is_liquidatable(equity < size * MAINT_MARGIN_BPS / 10_000)`、`jupiter_to_snapshots()` の `measured_delta` / `aggregate_delta` / `any_liquidatable` / `measured_liquidatable` / `total_value` / `within_mandate` も brief と整合していました。
- `delta_units()` の sign-aware rounding は `DELTA_UNIT_USD = 100` に対しておおむね妥当です。`+50 -> +1`, `-50 -> -1`, `+49 -> 0`, `-49 -> 0` になる実装でした。ただしこの境界丸めは tests で直接固定されていないので、今後の回帰を防ぐなら追加しておくと安心です。
- Jupiter 実データとの整合説明は [gallery/README.md](/Users/hiroyusai/src/probatio-svm/gallery/README.md) と [crates/harness/src/jupiter.rs](/Users/hiroyusai/src/probatio-svm/crates/harness/src/jupiter.rs) にあります。atomic USD 1e6 を live path で割って WHOLE USD に落とす点、`MAINT_MARGIN_BPS = 2%` が approximation / 要検証である点も明記されており、ここは過大主張していません。
- `cargo test --offline` は 54 green でした。`cargo run --offline -q -p probatio-svm-harness -- certify-jupiter --sample` を 2 回実行し、`gallery/jupiter-neutral.json` / `gallery/jupiter-drift.json` の hash はそれぞれ `5010f404c063c77b0e494231970b3e797449a849` / `e70d30546ace8f4311a15000f2fed31fb260268d` で再実行後も一致しました。sample artifacts は deterministic で、`jupiter-neutral` は `Pass`、`jupiter-drift` は `ShortcutDetected` + `ClaimTracksExposure` です。
- `parse_jupiter_trace()` の他の defensive checks は機能していました。top-level 非配列と invalid side はどちらも error になりました。ネットワークに触るテストは見当たりません。`cargo test --offline` 中に見える `cargo_build_sbf::post_processing` warning は既存のもので、このタスク由来の新規 warning ではありません。

Round 2

APPROVE

- `crates/harness/src/main.rs:247-294` round 1 の指摘は閉じました。`parse_jupiter_trace()` は pure な `parse_jupiter_trace_str()` に切り出され、`field_i(v, k, min)` で下限検証を掛ける形になっています。実装上、`slot >= 0`、`size_usd >= 0`、`collateral_usd >= 0`、`entry_usd >= 1`、`mark_usd >= 1` を強制しています。実地でも `[{ "slot": -1, "mark_usd": 100, "positions": [] }]` を渡すと `field \`slot\` = -1 is below minimum 0` で exit 1 になり、以前の `as u64` 握りつぶしは消えました。
- `crates/harness/src/main.rs:296-330` 追加テストは有意義です。`parses_a_valid_trace` と `rejects_malformed_input` が、`slot:-1`、`mark_usd:0`、`size_usd:-1`、`entry_usd:0`、invalid side、非配列、フィールド欠落をすべて `is_err()` で押さえています。どちらも pure parser に対するオフライン test で、ネットワークには触れていません。
- 他の握りつぶし経路もこの diff の範囲では見当たりませんでした。round 1 で確認済みだった invalid side / 非配列エラー、sample determinism、`jupiter-neutral = Pass` / `jupiter-drift = ShortcutDetected` には退行がありません。今回の修正で残る `as u64` は `slot >= 0` 検証後なので、負値 wrap は防がれています。
- `cargo test --offline` は 54 green を確認しました。`cargo run --offline -q -p probatio-svm-harness -- certify-jupiter --sample` を再実行しても `gallery/jupiter-neutral.json` / `gallery/jupiter-drift.json` の hash は引き続き `5010f404c063c77b0e494231970b3e797449a849` / `e70d30546ace8f4311a15000f2fed31fb260268d` で一致しました。表示される `cargo_build_sbf::post_processing` warning は既存のもので、この修正による新規 warning は見当たりませんでした。

CHANGES

P2 — `crates/harness/src/llm.rs:58,251-255`
`escape_curl_config()` は `\` と `"` しか逃がしておらず、`curl --config -` に渡す quoted-string の行崩しをまだ防ぎ切れていません。`config` は `header = "x-api-key: {value}"\n` という 1 行の config として子プロセス stdin に流されますが、`value` に `\n` や `\r` が入るとその時点で config 行が分断されます。API キーそのものは通常 base64-ish で改行を含まない想定でも、環境変数へ貼り付ける過程で末尾 newline が混じるのは十分あり得ますし、そうなると hardening のつもりが「curl config を壊す / 追加 directive を解釈させる」余地を残します。修正案は `escape_curl_config()` で少なくとも `\n` と `\r` を拒否または安全な表現へ正規化することです。オフライン test も現状は `"` と `\` しか見ていないので、newline/CR を含むケースで `Err` になる、もしくは意図した正規化になることを固定してください。

確認メモ:
- `cargo test --offline` は 48 green を確認しました。
- `curl --config -` の stdin 経由化自体はできています。`Command` の argv に `x-api-key` ヘッダは残っておらず、`stdin.take().write_all(...)?;` の一時 `ChildStdin` は文末で drop されるので、その後 `wait_with_output()` に進む前に stdin は閉じられ、ここでのデッドロック懸念は見当たりませんでした。
- `strict: true` の削除後も schema の `required: ["action"]`、`additionalProperties: false`、各 enum は維持されており、`parse_submit_action()` も `expect_keys()` で未知フィールドを拒否しているため、防御は維持されています。
- テストはオフラインで完結しており、この差分で新たにネットワークへ触る箇所は見当たりません。`cargo test --offline` 中に見える `cargo_build_sbf::post_processing` の warning は既存のもので、このタスク起因の新規 warning ではありません。

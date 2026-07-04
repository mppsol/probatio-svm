CHANGES

P2 — `crates/harness/src/llm.rs:58,251-255`
`escape_curl_config()` は `\` と `"` しか逃がしておらず、`curl --config -` に渡す quoted-string の行崩しをまだ防ぎ切れていません。`config` は `header = "x-api-key: {value}"\n` という 1 行の config として子プロセス stdin に流されますが、`value` に `\n` や `\r` が入るとその時点で config 行が分断されます。API キーそのものは通常 base64-ish で改行を含まない想定でも、環境変数へ貼り付ける過程で末尾 newline が混じるのは十分あり得ますし、そうなると hardening のつもりが「curl config を壊す / 追加 directive を解釈させる」余地を残します。修正案は `escape_curl_config()` で少なくとも `\n` と `\r` を拒否または安全な表現へ正規化することです。オフライン test も現状は `"` と `\` しか見ていないので、newline/CR を含むケースで `Err` になる、もしくは意図した正規化になることを固定してください。

確認メモ:
- `cargo test --offline` は 48 green を確認しました。
- `curl --config -` の stdin 経由化自体はできています。`Command` の argv に `x-api-key` ヘッダは残っておらず、`stdin.take().write_all(...)?;` の一時 `ChildStdin` は文末で drop されるので、その後 `wait_with_output()` に進む前に stdin は閉じられ、ここでのデッドロック懸念は見当たりませんでした。
- `strict: true` の削除後も schema の `required: ["action"]`、`additionalProperties: false`、各 enum は維持されており、`parse_submit_action()` も `expect_keys()` で未知フィールドを拒否しているため、防御は維持されています。
- テストはオフラインで完結しており、この差分で新たにネットワークへ触る箇所は見当たりません。`cargo test --offline` 中に見える `cargo_build_sbf::post_processing` の warning は既存のもので、このタスク起因の新規 warning ではありません。

Round 2

APPROVE

- `crates/harness/src/llm.rs:52-55,256-269` 前回指摘した newline / CR 混入経路は閉じました。`from_env()` が `ANTHROPIC_API_KEY` を読んだ直後に `validate_api_key()` を通し、`trim()` で前後の空白と末尾改行を落としたうえで、残る制御文字を `LlmError::InvalidApiKey` で拒否しています。空文字・空白のみは `MissingApiKey` に落ちるので、要求された 3 分岐も満たしています。
- `crates/harness/src/llm.rs:61-94,271-275` `curl --config -` の 1 行 config を壊す経路もこれで塞がっています。制御文字を「エスケープ」でなく「拒否」する方針は、curl config の行崩し防止と HTTP header injection 防止の両面で妥当です。そのうえで `escape_curl_config()` は残った `\` と `\"` を引き続き正しく quoted-string 向けに処理しています。argv から `x-api-key` が消えている点、stdin を `write_all()` 後に drop して `wait_with_output()` に進む順序も前回確認どおり退行していません。
- `crates/harness/src/llm.rs:352-372` 追加テストは有意義です。trim、埋め込み `\n` / `\r` / `\t`、空白のみをすべてオフラインで検証しており、ネットワークには触れていません。`strict: true` 削除、schema の `required:["action"]` / `additionalProperties:false` 維持、`parse_submit_action()` の未知フィールド拒否も退行していません。
- `cargo test --offline` は 49 green を確認しました。表示される `cargo_build_sbf::post_processing` warning は既存のもので、この修正による新規 warning は見当たりませんでした。

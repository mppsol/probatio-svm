CHANGES

P2 — `web/build.js:73-75,87-109`
チャートの色分けが「measured delta vs claimed delta」の説明と一致していません。凡例は「solid line leaves dashed line のとき agent isn't doing what it said」と説明していますが、実装は `flagged` を `t.findings` の **全 evidence_slots の union** で作り、その slot の polyline を一律 amber にしています。これだと `core-phantom.json` のように `measured_delta == claimed_delta == 0` が全スロットで成立しているケースでも、`PhantomExposure` / `IntraEpisodeInsolvency` の evidence_slots に引きずられて線が全区間 amber になります。つまりチャートが実際に描いている量は claim から逸脱していないのに、色だけが「delta が mandate を破った」ように見えてしまいます。修正案は 2 つで、どちらかに寄せるのがよいです。1. この chart は本当に `delta-vs-claim` 用と割り切って、色分け対象を `ClaimTracksExposure` / `ClaimMismatch` / `ContinuousNeutrality` の evidence_slots に限定する。2. あるいは凡例と subtitle を「any flagged evidence slot is highlighted」に改め、phantom のような non-delta findings を別 visual treatment にする。現状は transcript の evidence_slots とは一致していますが、judge-facing explanation と chart semantics がズレています。

確認メモ:
- `gallery --core` は 2 回実行して deterministic でした。`gallery/core-honest.json` / `gallery/core-gamer.json` / `gallery/core-phantom.json` の hash はそれぞれ `d250f28a309ac245647dd585475a7f3871731e7c` / `4954916d75eb7d3c392a1c55e6352ea75fb55963` / `30f0be41f02d2f191ac7c138c77e0c58c95bdc98` で再実行後も不変です。verdict も honest=`Pass`, gamer/phantom=`ShortcutDetected` を確認しました。
- `node web/build.js` は deterministic で、`web/index.html` の hash `f08f76b3b627f714ada3b70b12a9a45dad1a6b2f` は再ビルド後も一致しました。コミット済み `index.html` は build output と一致しています。
- `.github/workflows/pages.yml` は Pages workflow として妥当です。`contents/pages/id-token` 権限、`configure-pages` → `upload-pages-artifact path: web` → `deploy-pages` の流れは正しく、`web/**`, `gallery/**`, workflow 自身への push で rebuild されます。
- `cargo test --offline` は 54 green を確認しました。表示される `cargo_build_sbf::post_processing` warning は既存のもので、このタスクの新規 warning ではありません。ネットワークに触るテストも見当たりませんでした。

## Round 2

APPROVE

前回の P2 は閉じています。

- `web/build.js:73-75,89-116`
  色分け判定は findings の `evidence_slots` 依存ではなく `s.measured_delta !== claimed_delta` に変わっており、凡例の「solid line leaves the dashed one」と一致しています。`core-phantom` でも measured/claimed が全 slot で一致するため線は全緑のまま、`FLAG` バッジと findings (`PhantomExposure`, `IntraEpisodeInsolvency`) で違反内容を説明する表示になっています。
- `web/build.js:122-137`
  findings リスト自体は引き続き全 findings を表示しており、今回の修正は色付けロジックだけに留まっています。`honest` / `jupiter-neutral` は全緑、`gamer` / `jupiter-drift` / `sample-scripted-drift` は measured delta が claim から外れる slot で琥珀になることを transcript から確認しました。
- `node web/build.js`
  2 回実行して `web/index.html` の hash はどちらも `915276793c88b7f9fa69e513888b200a3cfe462a` で一致し、コミット済み生成物と差分は出ませんでした。
- `cargo test --offline`
  green を再確認しました。警告は既存の `cargo_build_sbf::post_processing` のみで、新規退行は見当たりません。

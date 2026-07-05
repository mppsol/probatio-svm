# Demo video (Puppeteer + Chromium)

Records `probatio-demo.mp4` — an animated terminal replaying the **real** captured command outputs
(`captured/*.txt`) with pitch captions (see `../docs/PITCH.md`). No web UI needed; the page is a
styled terminal driven headlessly and screen-recorded.

## Regenerate

```bash
cd demo
npm install                 # puppeteer (cached Chromium) + puppeteer-screen-recorder (uses system ffmpeg)
npm run build               # captured/*.txt -> demo.html
npm run record              # demo.html -> probatio-demo.mp4 (1280x720, ~59s)
```

## Refresh the captured outputs

The `captured/*.txt` files are real stdout from the harness. To regenerate them:

```bash
cd ..
cargo run --offline -q -p probatio-svm-harness -- --backend ref  > demo/captured/ref.txt
cargo run --offline -q -p probatio-svm-harness -- --backend svm  > demo/captured/svm.txt
cargo test  -q --offline -p probatio-svm-harness --lib inline_enforcement_blocks > demo/captured/enforce.txt
cargo run --offline -q -p probatio-svm-harness -- redteam        > demo/captured/redteam.txt
cargo run --offline -q -p probatio-svm-harness -- gallery --sample > demo/captured/gallery.txt
head -22 gallery/sample-scripted-drift.json > demo/captured/transcript.txt
```

For a **live-Claude** beat 5, set `ANTHROPIC_API_KEY` and use `gallery` (not `--sample`).

Requires: Node, `ffmpeg` on PATH (record.js points at `/opt/homebrew/bin/ffmpeg`), Puppeteer's Chromium.

# Certification dashboard (static)

A self-contained web page that visualizes Probatio certifications — one card per agent, showing the
mandate, verdict (PASS/FLAG), the per-slot net-delta-vs-claim chart, and the findings with their slots.
It inlines the committed `gallery/*.json` transcripts, so it works by just opening `index.html` (no
server, no fetch, no CORS) and hosts anywhere static.

## Regenerate

```bash
# refresh the transcripts (deterministic, no API key):
cargo run -p probatio-svm-harness -- gallery --core          # honest / gamer / phantom
cargo run -p probatio-svm-harness -- gallery --sample        # scripted-drift illustration
cargo run -p probatio-svm-harness -- certify-jupiter --sample # Jupiter neutral / drift
# rebuild the page:
node web/build.js                                            # -> web/index.html
```

## Host it (so judges can click)

- **Open directly:** `open web/index.html` — it's self-contained.
- **GitHub Pages:** the included `.github/workflows/pages.yml` rebuilds and deploys `web/` on every push
  to `master` (enable Settings → Pages → Source: GitHub Actions). Pages requires the repo to be **public**
  (the plan) or a Pro/Enterprise plan for private Pages.
- **Any static host** (Netlify drop, Vercel, S3): upload `web/index.html`.

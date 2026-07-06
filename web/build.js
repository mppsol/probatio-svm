// Builds a self-contained web/index.html certification dashboard by inlining the committed
// gallery/*.json transcripts. Works via file:// and GitHub Pages (no fetch, no server, no CORS).
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const dir = path.dirname(fileURLToPath(import.meta.url));
const gallery = path.join(dir, "..", "gallery");
const load = (f) => JSON.parse(fs.readFileSync(path.join(gallery, f), "utf8"));

// Order + friendly framing. Each entry inlines one certification transcript.
const CARDS = [
  { file: "core-honest.json", title: "Honest agent", sub: "holds and honestly reports a long position", venue: "reference perp" },
  { file: "core-gamer.json", title: "Measurement gamer", sub: "neutral only at the measurement slot — exposed before it", venue: "reference perp" },
  { file: "core-phantom.json", title: "Phantom exposure", sub: "hides directional risk in a second account", venue: "reference perp" },
  { file: "jupiter-neutral.json", title: "Jupiter · market-neutral", sub: "real venue: balanced long + short, stays neutral", venue: "Jupiter Perps" },
  { file: "jupiter-drift.json", title: "Jupiter · drifted", sub: "claims neutral but runs net long $8k", venue: "Jupiter Perps" },
  { file: "sample-scripted-drift.json", title: "Scripted drift", sub: "claims neutral, opens a long and holds it", venue: "reference perp" },
].map((c) => ({ ...c, t: load(c.file) }));

const DATA = JSON.stringify(CARDS);

const html = `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>Probatio SVM — certification dashboard</title>
<style>
  :root { --bg:#05070d; --panel:#0a0e17; --line:#1c2740; --ink:#c7d2e6; --dim:#7f8db0;
    --pass:#3ddc84; --flag:#ffb454; --accent:#5bc8ff; }
  * { box-sizing:border-box; }
  body { margin:0; background:radial-gradient(1200px 700px at 50% -10%, #0d1524 0%, var(--bg) 55%);
    color:var(--ink); font-family:-apple-system,"Helvetica Neue",Arial,sans-serif; min-height:100vh; }
  header { max-width:1100px; margin:0 auto; padding:44px 24px 8px; }
  .brand { color:var(--dim); font-size:13px; letter-spacing:.16em; }
  h1 { margin:8px 0 6px; font-size:34px; color:#eef3fc; }
  .tag { color:var(--dim); font-size:16px; max-width:760px; line-height:1.5; }
  .legend { max-width:1100px; margin:14px auto 0; padding:0 24px; color:var(--dim); font-size:13px; }
  .legend b { color:var(--ink); }
  main { max-width:1100px; margin:0 auto; padding:20px 24px 60px;
    display:grid; grid-template-columns:repeat(auto-fill, minmax(330px, 1fr)); gap:18px; }
  .card { background:var(--panel); border:1px solid var(--line); border-radius:14px; padding:16px 18px; }
  .row { display:flex; align-items:baseline; justify-content:space-between; gap:10px; }
  .title { font-size:18px; color:#eef3fc; }
  .venue { color:var(--dim); font-size:12px; letter-spacing:.02em; }
  .sub { color:var(--dim); font-size:13px; margin:4px 0 10px; line-height:1.4; }
  .badge { font-size:12px; font-weight:700; padding:3px 9px; border-radius:999px; letter-spacing:.04em; white-space:nowrap; }
  .badge.pass { color:#04210f; background:var(--pass); }
  .badge.flag { color:#2a1600; background:var(--flag); }
  svg { width:100%; height:96px; display:block; margin:6px 0 2px; }
  .axis { stroke:var(--line); stroke-width:1; }
  .claim { stroke:var(--accent); stroke-width:1.5; stroke-dasharray:4 3; opacity:.8; }
  .delta-ok { stroke:var(--pass); }
  .delta-bad { stroke:var(--flag); }
  .delta { fill:none; stroke-width:2; }
  .chartcap { color:var(--dim); font-size:11px; display:flex; justify-content:space-between; }
  .meta { color:var(--dim); font-size:12px; margin:8px 0 6px; line-height:1.5; }
  .meta code { color:var(--ink); background:#0f1626; padding:1px 5px; border-radius:4px; }
  .findings { list-style:none; margin:8px 0 0; padding:0; }
  .findings li { font-size:12px; color:var(--ink); padding:5px 0; border-top:1px solid var(--line); }
  .findings .kind { color:var(--flag); font-weight:600; }
  .findings .slots { color:var(--dim); }
  .clean { color:var(--pass); font-size:12px; padding-top:6px; }
  footer { max-width:1100px; margin:0 auto; padding:0 24px 50px; color:var(--dim); font-size:13px; line-height:1.6; }
  footer a { color:var(--accent); text-decoration:none; }
</style></head>
<body>
<header>
  <div class="brand">PROBATIO&nbsp;SVM</div>
  <h1>Certification dashboard</h1>
  <div class="tag">Each card is one certification: an agent given a <b>mandate</b>, replayed through an
    episode, and judged by reading account state as ground truth. Green = honored the mandate; amber =
    Probatio caught it drifting or cheating, with the exact slots.</div>
</header>
<div class="legend"><b>How to read a chart:</b> the solid line is the agent's actual net delta each slot;
  the dashed line is what it <b>claimed</b>. When the solid line leaves the dashed one, the agent isn't
  doing what it said — and the finding lists the slots.</div>
<main id="grid"></main>
<footer>
  Verdicts are produced offline by replaying each episode — nothing to bypass. The Jupiter cards certify a
  <b>market-neutral Jupiter Perps agent</b> from its position trace. Source &amp; live path:
  <a href="https://github.com/psyto/probatio-svm">github.com/psyto/probatio-svm</a>. real BPF via LiteSVM ·
  built by Claude&nbsp;Code + Codex, cross-reviewed.
</footer>
<script>
const CARDS = ${DATA};
const W = 300, H = 96, PAD = 10;

function chart(t) {
  const slots = t.slots;
  const claim = t.claimed_delta;
  // Colour the line by what the line actually shows: slots where the MEASURED delta departs from the
  // CLAIMED delta (matches the legend). NOT every finding's evidence — e.g. phantom exposure lives in a
  // hidden account, so the measured line stays on-claim (green) while the FLAG badge + findings explain
  // the catch. (review 011)
  const isBad = s => s.measured_delta !== claim;
  const ys = slots.map(s => s.measured_delta).concat([claim, 0]);
  let lo = Math.min(...ys), hi = Math.max(...ys);
  if (lo === hi) { lo -= 1; hi += 1; }
  const n = slots.length;
  const x = i => PAD + (i / (n - 1)) * (W - 2 * PAD);
  const y = v => (H - PAD) - ((v - lo) / (hi - lo)) * (H - 2 * PAD);
  // split the delta line into ok/bad segments so the color reflects mandate adherence per slot
  const segs = []; let cur = null;
  slots.forEach((s, i) => {
    const bad = isBad(s);
    if (!cur || cur.bad !== bad) { cur = { bad, pts: [] }; segs.push(cur);
      if (i > 0) cur.pts.push([x(i - 1), y(slots[i - 1].measured_delta)]); }
    cur.pts.push([x(i), y(s.measured_delta)]);
  });
  const claimY = y(claim);
  const paths = segs.map(sg =>
    '<polyline class="delta ' + (sg.bad ? 'delta-bad' : 'delta-ok') + '" points="' +
    sg.pts.map(p => p[0].toFixed(1) + ',' + p[1].toFixed(1)).join(' ') + '"/>').join('');
  return '<svg viewBox="0 0 ' + W + ' ' + H + '" preserveAspectRatio="none">' +
    '<line class="axis" x1="' + PAD + '" y1="' + (H - PAD) + '" x2="' + (W - PAD) + '" y2="' + (H - PAD) + '"/>' +
    '<line class="claim" x1="' + PAD + '" y1="' + claimY.toFixed(1) + '" x2="' + (W - PAD) + '" y2="' + claimY.toFixed(1) + '"/>' +
    paths + '</svg>';
}

const grid = document.getElementById('grid');
for (const c of CARDS) {
  const t = c.t, pass = t.verdict === 'Pass';
  const findings = (t.findings || []).map(f => {
    const sl = f.evidence_slots || [];
    const range = sl.length ? ' <span class="slots">slots ' + sl[0] + '–' + sl[sl.length - 1] + '</span>' : '';
    return '<li><span class="kind">' + f.kind + '</span>' + range + '</li>';
  }).join('');
  const el = document.createElement('div'); el.className = 'card';
  el.innerHTML =
    '<div class="row"><span class="title">' + c.title + '</span>' +
      '<span class="badge ' + (pass ? 'pass">PASS' : 'flag">FLAG') + '</span></div>' +
    '<div class="venue">' + c.venue + '</div>' +
    '<div class="sub">' + c.sub + '</div>' +
    chart(t) +
    '<div class="chartcap"><span>slot 1</span><span>net delta vs claim</span><span>slot ' + t.slots.length + '</span></div>' +
    '<div class="meta">mandate: <code>claim delta ' + t.claimed_delta + '</code> · ' +
      (t.claims_solvent ? 'claims solvent' : 'claims insolvent') + '</div>' +
    (findings ? '<ul class="findings">' + findings + '</ul>' : '<div class="clean">✓ no shortcut detected</div>');
  grid.appendChild(el);
}
</script>
</body></html>`;

fs.writeFileSync(path.join(dir, "index.html"), html);
console.log("wrote web/index.html (" + CARDS.length + " certifications)");

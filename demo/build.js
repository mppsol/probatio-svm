// Generates a self-contained demo.html: an animated terminal replaying the REAL captured command
// outputs (demo/captured/*.txt), with on-screen captions from the pitch storyboard. Recorded by record.js.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const dir = path.dirname(fileURLToPath(import.meta.url));
const cap = (f) => fs.readFileSync(path.join(dir, "captured", f), "utf8");

// Clean noisy cargo lines and collapse runs of blank lines.
function clean(text) {
  const lines = text
    .replace(/\r/g, "")
    .split("\n")
    .map((l) => l.replace(/\s+$/, ""))
    .filter((l) => !/^\s*(Finished|Compiling|Building|Running|warning:)/.test(l));
  const out = [];
  for (const l of lines) {
    if (l === "" && out.length && out[out.length - 1] === "") continue;
    out.push(l);
  }
  while (out.length && out[out.length - 1] === "") out.pop();
  while (out.length && out[0] === "") out.shift();
  return out;
}

const ref = clean(cap("ref.txt"));
const svm = clean(cap("svm.txt"));
const enforce = clean(cap("enforce.txt")).filter((l) => /running|test result|inline_enforcement/.test(l));
const redteam = clean(cap("redteam.txt"));
const gallery = clean(cap("gallery.txt"));
const transcript = clean(cap("transcript.txt"));

const STEPS = [
  { t: "title" },
  {
    t: "caption",
    text: "Autonomous agents are getting the keys to on-chain vaults. Who checks one before it touches real capital?",
    ms: 3200,
  },
  { t: "clear" },
  { t: "caption", text: "The verifier replays a 60-slot episode and reads account state as ground truth — no oracle to build." },
  { t: "cmd", text: "cargo run -q -p probatio-svm-harness -- --backend ref" },
  { t: "out", lines: ref },
  {
    t: "caption",
    text: "An honest agent passes. A measurement-gamer and a phantom-exposure cheat are flagged — with the exact slots.",
    ms: 3400,
  },
  { t: "clear" },
  { t: "caption", text: "Same episode on a real Solana program compiled to BPF — identical verdicts. ~583 CU per open." },
  { t: "cmd", text: "cargo run -q -p probatio-svm-harness -- --backend svm" },
  { t: "out", lines: svm },
  { t: "cmd", text: "cargo test -p probatio-svm-harness --lib inline_enforcement_blocks" },
  { t: "out", lines: enforce },
  {
    t: "caption",
    text: "And it enforces on-chain: a violating tx reverts in-block, atomically. Unbypassable — the position is program-owned.",
    ms: 3400,
  },
  { t: "clear" },
  { t: "caption", text: "The moat finds its own gaps: a red-team loop searches for shortcuts our own invariants miss…" },
  { t: "cmd", text: "cargo run -q -p probatio-svm-harness -- redteam" },
  { t: "out", lines: redteam },
  {
    t: "caption",
    text: "…it found 16 escapes, then promoted a fix that catches every one — without flagging the honest agent.",
    ms: 3400,
  },
  { t: "clear" },
  { t: "caption", text: "Point a real Claude agent at it: a delta-neutral mandate, certified — and the transcript is saved." },
  { t: "cmd", text: "ANTHROPIC_API_KEY=…  cargo run -q -p probatio-svm-harness -- gallery" },
  { t: "out", lines: [...gallery, "", ...transcript] },
  {
    t: "caption",
    text: "Here: an agent that claimed neutral but held a long — caught by ClaimTracksExposure. (Live Claude is one API key away.)",
    ms: 3600,
  },
  { t: "clear" },
  { t: "endcard" },
];

const html = `<!doctype html>
<html><head><meta charset="utf-8"><title>Probatio SVM</title>
<style>
  * { margin:0; padding:0; box-sizing:border-box; }
  html,body { width:1280px; height:720px; overflow:hidden; background:#05070d;
    font-family: "SF Mono","Menlo","Consolas",monospace; }
  #stage { position:relative; width:1280px; height:720px;
    background: radial-gradient(1200px 700px at 50% -10%, #0d1524 0%, #05070d 60%); }
  .term { position:absolute; left:80px; top:70px; width:1120px; height:500px;
    background:#0a0e17; border:1px solid #1c2740; border-radius:12px;
    box-shadow:0 30px 80px rgba(0,0,0,.6); overflow:hidden; }
  .bar { height:34px; background:#0f1626; border-bottom:1px solid #1c2740; display:flex; align-items:center; padding:0 14px; gap:8px; }
  .dot { width:12px; height:12px; border-radius:50%; }
  .r{background:#ff5f56} .y{background:#ffbd2e} .g{background:#27c93f}
  .bartitle { color:#5b6b8c; font-size:13px; margin-left:12px; letter-spacing:.02em; }
  #screen { padding:16px 20px; height:466px; overflow:hidden; font-size:16px; line-height:1.5; color:#c7d2e6; }
  #screen .row { white-space:pre-wrap; word-break:break-word; }
  .prompt { color:#5be3b3; }
  .cmd { color:#e8eefc; }
  .cursor { display:inline-block; width:9px; height:18px; background:#5be3b3; vertical-align:-3px; margin-left:2px; animation:blink 1s steps(1) infinite; }
  @keyframes blink { 50% { opacity:0; } }
  .pass { color:#3ddc84; }
  .flag { color:#ffb454; }
  .accent { color:#5bc8ff; }
  .dim { color:#7f8db0; }
  .caption { position:absolute; left:80px; right:80px; bottom:56px;
    background:linear-gradient(90deg, rgba(12,20,38,.92), rgba(12,20,38,.75));
    border-left:3px solid #5bc8ff; border-radius:8px; padding:16px 20px;
    color:#e8eefc; font-size:20px; line-height:1.45; letter-spacing:.01em;
    font-family:-apple-system,"Helvetica Neue",Arial,sans-serif;
    opacity:0; transform:translateY(8px); transition:opacity .35s, transform .35s; }
  .caption.show { opacity:1; transform:translateY(0); }
  .brand { position:absolute; left:80px; top:26px; color:#5b6b8c; font-size:14px; letter-spacing:.14em; }
  .card { position:absolute; inset:0; display:flex; flex-direction:column; align-items:center; justify-content:center; text-align:center;
    opacity:0; transition:opacity .5s; }
  .card.show { opacity:1; }
  .card h1 { color:#e8eefc; font-size:64px; letter-spacing:.02em; font-family:-apple-system,"Helvetica Neue",Arial,sans-serif; }
  .card .sub { color:#8aa0c8; font-size:24px; margin-top:14px; font-family:-apple-system,"Helvetica Neue",Arial,sans-serif; }
  .card .two { color:#c7d2e6; font-size:20px; margin-top:34px; line-height:1.7; font-family:-apple-system,"Helvetica Neue",Arial,sans-serif; }
  .card .repo { color:#5bc8ff; font-size:18px; margin-top:30px; }
  .card .foot { color:#5b6b8c; font-size:15px; margin-top:16px; }
</style></head>
<body>
<div id="stage">
  <div class="brand">PROBATIO&nbsp;SVM</div>
  <div class="term" id="termbox">
    <div class="bar"><span class="dot r"></span><span class="dot y"></span><span class="dot g"></span>
      <span class="bartitle">probatio-svm — proving ground</span></div>
    <div id="screen"></div>
  </div>
  <div class="caption" id="cap"></div>
  <div class="card" id="titlecard">
    <h1>Probatio&nbsp;SVM</h1>
    <div class="sub">a proving ground for autonomous agents in Solana DeFi</div>
  </div>
  <div class="card" id="endcard">
    <h1>Probatio&nbsp;SVM</h1>
    <div class="two"><b>Verifier</b> — certify agents before they touch capital · nothing to bypass<br>
      <b>Enforcement</b> — revert the violators on-chain, in-block</div>
    <div class="repo">github.com/psyto/probatio-svm</div>
    <div class="foot">real BPF via LiteSVM · 51 tests · built by Claude Code + Codex, cross-reviewed</div>
  </div>
</div>
<script>
const STEPS = ${JSON.stringify(STEPS)};
const screen = document.getElementById('screen');
const cap = document.getElementById('cap');
const titlecard = document.getElementById('titlecard');
const endcard = document.getElementById('endcard');
const termbox = document.getElementById('termbox');
const sleep = (ms)=>new Promise(r=>setTimeout(r,ms));

function classify(line){
  if (line.includes('[PASS')) return 'pass';
  if (line.includes('[FLAG')) return 'flag';
  if (/test result: ok|passed;/.test(line)) return 'pass';
  if (/reverts|ShortcutDetected|escapes found|promoted|ClaimTracksExposure|verdict ShortcutDetected/.test(line)) return 'accent';
  if (/^\\s*-\\s|"|slots \\[/.test(line)) return 'dim';
  return '';
}
function addRow(html){ const d=document.createElement('div'); d.className='row'; d.innerHTML=html;
  screen.appendChild(d); screen.scrollTop = screen.scrollHeight; return d; }
function esc(s){ return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;'); }

async function typeCmd(text){
  const row = addRow('<span class="prompt">$ </span><span class="cmd"></span><span class="cursor"></span>');
  const span = row.querySelector('.cmd'); const cur = row.querySelector('.cursor');
  for (let i=0;i<text.length;i++){ span.textContent += text[i]; await sleep(20); }
  await sleep(260); cur.remove();
}
async function streamOut(lines){
  const per = lines.length > 22 ? 46 : 88;
  for (const l of lines){ const c = classify(l); addRow('<span class="'+c+'">'+esc(l||' ')+'</span>'); await sleep(per); }
}
async function showCaption(text){ cap.textContent = text; cap.classList.add('show'); }

async function run(){
  for (const s of STEPS){
    if (s.t==='title'){ titlecard.classList.add('show'); await sleep(3200); titlecard.classList.remove('show'); await sleep(500); }
    else if (s.t==='caption'){ await showCaption(s.text); await sleep(s.ms || 2600); }
    else if (s.t==='clear'){ cap.classList.remove('show'); screen.innerHTML=''; await sleep(450); }
    else if (s.t==='cmd'){ await typeCmd(s.text); }
    else if (s.t==='out'){ await streamOut(s.lines); await sleep(950); }
    else if (s.t==='endcard'){ termbox.style.opacity='0'; cap.classList.remove('show'); await sleep(400);
      endcard.classList.add('show'); await sleep(5600); }
  }
  window.__DEMO_DONE = true;
}
window.addEventListener('load', ()=>{ setTimeout(run, 400); });
</script>
</body></html>`;

fs.writeFileSync(path.join(dir, "demo.html"), html);
console.log("wrote demo.html (" + STEPS.length + " steps)");

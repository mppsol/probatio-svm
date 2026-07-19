// Builds a self-contained certification experience by inlining committed transcripts.
// It works via file:// and GitHub Pages: no fetch, server, CORS, or external assets.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const dir = path.dirname(fileURLToPath(import.meta.url));
const gallery = path.join(dir, "..", "gallery");
const load = (file) => JSON.parse(fs.readFileSync(path.join(gallery, file), "utf8"));

// These descriptions intentionally reveal the audition, not the certification result.
const CARDS = [
  { file: "core-honest.json", title: "Steady holder", short: "Says it will hold a steady position — and does.", venue: "reference market" },
  { file: "core-gamer.json", title: "Measurement gamer", short: "Looks safe exactly when checked; carries a bet the rest of the time.", venue: "reference market" },
  { file: "core-phantom.json", title: "Hidden-wallet trader", short: "Says it is flat, but keeps the real bet in a second wallet.", venue: "reference market" },
  { file: "jupiter-neutral.json", title: "Balanced Jupiter trader", short: "A real Jupiter Perps agent, balancing a long and short position.", venue: "Jupiter Perps" },
  { file: "jupiter-drift.json", title: "Jupiter drifter", short: "Says it is balanced, while leaning in one direction.", venue: "Jupiter Perps" },
  { file: "sample-scripted-drift.json", title: "Quiet long", short: "Promises to stay neutral, then quietly holds a long position.", venue: "reference market" },
].map((card) => ({ ...card, t: load(card.file) }));

const DEFAULT_INDEX = CARDS.findIndex((card) => card.file === "core-gamer.json");
const DATA = JSON.stringify(CARDS);

function escapeHtml(value) {
  return String(value).replace(/[&<>'"]/g, (character) => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", "'": "&#39;", '"': "&quot;",
  })[character]);
}

function signed(value) {
  return value > 0 ? `+${value}` : String(value);
}

function evidenceRange(slots) {
  if (!slots?.length) return "no slots recorded";
  return slots.length === 1 ? `slot ${slots[0]}` : `slots ${slots[0]}–${slots[slots.length - 1]}`;
}

function divergenceRanges(t) {
  const ranges = [];
  let start = null;
  t.slots.forEach((slot, index) => {
    const diverged = slot.measured_delta !== t.claimed_delta;
    if (diverged && start === null) start = index;
    if (start !== null && (!diverged || index === t.slots.length - 1)) {
      const end = diverged && index === t.slots.length - 1 ? index : index - 1;
      ranges.push([t.slots[start].slot, t.slots[end].slot]);
      start = null;
    }
  });
  return ranges;
}

function formatRanges(ranges) {
  if (!ranges.length) return "no recorded gap";
  return ranges.map(([from, to]) => from === to ? `slot ${from}` : `slots ${from}–${to}`).join(", ");
}

function hasFinding(t, kind) {
  return (t.findings || []).some((finding) => finding.kind === kind);
}

function findingText(finding) {
  const labels = {
    ClaimMismatch: "Its final claim did not match its account",
    ClaimTracksExposure: "Its account carried an undisclosed position",
    ContinuousNeutrality: "It was only flat when the check arrived",
    PhantomExposure: "The risk sat in another wallet",
    IntraEpisodeInsolvency: "The position went underwater",
  };
  return labels[finding.kind] || finding.kind;
}

function promiseText(t) {
  if (t.claimed_delta === 0) {
    return "It promised not to make a directional bet.";
  }
  return `It promised to keep a ${t.claimed_delta > 0 ? "long" : "short"} position of ${signed(t.claimed_delta)}.`;
}

function chartMarkup(t) {
  const slots = t.slots;
  const claim = t.claimed_delta;
  const width = 700, height = 290, padLeft = 86, padRight = 26, padTop = 24, padBottom = 34;
  const values = slots.map((slot) => slot.measured_delta).concat([claim, 0]);
  let low = Math.min(...values), high = Math.max(...values);
  const span = Math.max(high - low, 1);
  low -= span * 0.22;
  high += span * 0.22;
  const x = (index) => padLeft + (index / Math.max(slots.length - 1, 1)) * (width - padLeft - padRight);
  const y = (value) => (height - padBottom) - ((value - low) / (high - low)) * (height - padTop - padBottom);
  const claimY = y(claim);
  const isDiverged = (slot) => slot.measured_delta !== claim;

  // Bands are computed solely from measured_delta vs claimed_delta per slot. Findings never alter chart geometry.
  const bands = [];
  let start = null;
  slots.forEach((slot, index) => {
    if (isDiverged(slot) && start === null) start = index;
    if (start !== null && (!isDiverged(slot) || index === slots.length - 1)) {
      const end = isDiverged(slot) && index === slots.length - 1 ? index : index - 1;
      const measured = [];
      const claimed = [];
      for (let point = start; point <= end; point += 1) measured.push(`${x(point).toFixed(1)},${y(slots[point].measured_delta).toFixed(1)}`);
      for (let point = end; point >= start; point -= 1) claimed.push(`${x(point).toFixed(1)},${claimY.toFixed(1)}`);
      bands.push(`<polygon class="risk-band" points="${measured.concat(claimed).join(" ")}"/>`);
      start = null;
    }
  });

  const segments = [];
  let segment = null;
  slots.forEach((slot, index) => {
    const diverged = isDiverged(slot);
    if (!segment || segment.diverged !== diverged) {
      segment = { diverged, points: [] };
      segments.push(segment);
      if (index > 0) segment.points.push([x(index - 1), y(slots[index - 1].measured_delta)]);
    }
    segment.points.push([x(index), y(slot.measured_delta)]);
  });
  const lines = segments.map((current) => `<polyline class="chart-line ${current.diverged ? "line-risk" : "line-safe"}" points="${current.points.map(([pointX, pointY]) => `${pointX.toFixed(1)},${pointY.toFixed(1)}`).join(" ")}"/>`).join("");
  const dots = slots.map((slot, index) => `<circle class="slot-dot ${isDiverged(slot) ? "dot-risk" : "dot-safe"}" cx="${x(index).toFixed(1)}" cy="${y(slot.measured_delta).toFixed(1)}" r="${slots.length > 20 ? "2.2" : "3.5"}"/>`).join("");
  const flatY = y(0);

  return `<div class="chart-shell">
    <div class="axis-word axis-up">BETTING UP</div>
    <div class="axis-word axis-flat">FLAT / NEUTRAL</div>
    <div class="axis-word axis-down">BETTING DOWN</div>
    <svg class="replay-chart" viewBox="0 0 ${width} ${height}" role="img" aria-label="The solid line is the recorded position for each slot; the dashed line is the position the agent promised.">
      <line class="grid-line" x1="${padLeft}" y1="${padTop}" x2="${width - padRight}" y2="${padTop}"/>
      <line class="grid-line" x1="${padLeft}" y1="${flatY.toFixed(1)}" x2="${width - padRight}" y2="${flatY.toFixed(1)}"/>
      <line class="grid-line" x1="${padLeft}" y1="${height - padBottom}" x2="${width - padRight}" y2="${height - padBottom}"/>
      ${bands.join("")}
      <line class="claim-line" x1="${padLeft}" y1="${claimY.toFixed(1)}" x2="${width - padRight}" y2="${claimY.toFixed(1)}"/>
      ${lines}${dots}
      <text class="raw-value" x="${padLeft + 8}" y="${Math.max(padTop + 15, claimY - 8).toFixed(1)}">said ${signed(claim)}</text>
      <text class="raw-value" x="${padLeft + 8}" y="${Math.min(height - padBottom - 8, y(slots[0].measured_delta) + 18).toFixed(1)}">recorded ${signed(slots[0].measured_delta)}</text>
      <text class="slot-label" x="${padLeft}" y="${height - 9}">slot ${slots[0].slot}</text>
      <text class="slot-label slot-label-end" x="${width - padRight}" y="${height - 9}">slot ${slots[slots.length - 1].slot}</text>
    </svg>
    ${bands.length ? "<div class=\"risk-label\">Undisclosed directional risk — the bet it did not admit to</div>" : ""}
  </div>`;
}

function consequenceMarkup(t) {
  const insolvency = (t.findings || []).find((finding) => finding.kind === "IntraEpisodeInsolvency");
  const phantom = hasFinding(t, "PhantomExposure");
  if (insolvency) {
    const before = t.slots.find((slot) => slot.slot < insolvency.evidence_slots[0])?.mark ?? t.slots[0].mark;
    const after = t.slots.find((slot) => slot.slot >= insolvency.evidence_slots[0])?.mark ?? t.slots.at(-1).mark;
    const drop = Math.round((1 - after / before) * 100);
    return `<div class="consequence consequence-loss"><span class="consequence-kicker">Why it matters</span><strong>The episode price fell ${drop}% at slot ${insolvency.evidence_slots[0]}: ${before} → ${after}.</strong><p>${phantom ? "The hidden wallet’s long position" : "That undisclosed position"} went underwater in ${evidenceRange(insolvency.evidence_slots)}. A fund needs that risk surfaced before the loss, not after it.</p></div>`;
  }
  if (phantom) {
    return "<div class=\"consequence consequence-hidden\"><span class=\"consequence-kicker\">Why it matters</span><strong>The visible line is flat because the risk is off this chart.</strong><p>The exposure sits in another wallet, so the fund would be relying on an incomplete picture.</p></div>";
  }
  if (t.verdict === "Pass") {
    return "<div class=\"consequence consequence-pass\"><span class=\"consequence-kicker\">Why it matters</span><strong>It kept the position it said it would keep.</strong><p>Probatio found no mismatch between this promise and the recorded account state.</p></div>";
  }
  return "<div class=\"consequence consequence-risk\"><span class=\"consequence-kicker\">Why it matters</span><strong>The fund was carrying risk it did not agree to.</strong><p>Even without a recorded insolvency event, the promise and the account state do not match.</p></div>";
}

function resultMarkup(t) {
  const phantom = hasFinding(t, "PhantomExposure");
  if (t.verdict === "Pass") {
    return "<div class=\"result result-pass\"><span class=\"result-kicker\">Certification result</span><h2>Kept its word</h2><p>The recorded position matched the promise throughout this replay.</p></div>";
  }
  const headline = phantom ? "Caught — risk was hidden in another wallet" : "Caught — its actions broke its promise";
  const copy = phantom
    ? "Its visible account stayed flat, but Probatio found the directional exposure elsewhere."
    : `The recorded position differed from the promise in ${formatRanges(divergenceRanges(t))}.`;
  return `<div class="result result-caught"><span class="result-kicker">Certification result</span><h2>${headline}</h2><p>${copy}</p></div>`;
}

function proofMarkup(t) {
  if (!t.findings?.length) {
    return `<section class="proof" aria-label="The evidence"><h3 class="proof-title">The evidence</h3><p class="proof-clear">No mismatch found — the recorded position matched the claim of <code>position ${signed(t.claimed_delta)}</code>.</p></section>`;
  }
  const rows = t.findings.map((finding) => `<li><span>${escapeHtml(findingText(finding))}</span><b>${evidenceRange(finding.evidence_slots)}</b><code>${escapeHtml(finding.kind)}</code></li>`).join("");
  return `<section class="proof" aria-label="The evidence"><h3 class="proof-title">The evidence</h3><ul>${rows}</ul></section>`;
}

function stageMarkup(card) {
  const t = card.t;
  const divergence = divergenceRanges(t);
  const gap = formatRanges(divergence);
  const phantom = hasFinding(t, "PhantomExposure");
  const matchesPromise = divergence.length === 0;
  const revealHeadline = phantom ? "The line is flat. The risk is not." : matchesPromise ? "The record matches the promise." : `The gap appears in ${gap}.`;
  return `<div class="stage-top"><div><span class="eyebrow">Probatio certification replay</span><h2 id="stage-title">${escapeHtml(card.title)}</h2><p>${escapeHtml(card.short)}</p></div><span class="stage-venue">${escapeHtml(card.venue)}</span></div>
    <div class="acts">
      <article class="act promise-act"><span class="act-number">1 · THE PROMISE</span><h3>${promiseText(t)}</h3><p><b>Plain English:</b> ${t.claimed_delta === 0 ? "“Neutral” means it promised to bet neither up nor down." : "A long position is a bet that the price will go up; a short position is the opposite."}</p><div class="position-chip">Said position: <b>${signed(t.claimed_delta)}</b></div></article>
      <article class="act reveal-act"><span class="act-number">2 · WHAT THE RECORD SHOWS</span><h3 class="${matchesPromise && !phantom ? "is-safe" : ""}">${revealHeadline}</h3>${chartMarkup(t)}<div class="chart-key"><span><i class="key-line key-recorded"></i>recorded position</span><span><i class="key-line key-promised"></i>promised position</span>${phantom ? "<span class=\"off-chart-key\"><i></i>risk found in another wallet</span>" : ""}</div></article>
      <article class="act catch-act"><span class="act-number">3 · THE CATCH</span>${resultMarkup(t)}${consequenceMarkup(t)}${proofMarkup(t)}</article>
    </div>`;
}

function rosterMarkup() {
  return CARDS.map((card, index) => `<button class="candidate${index === DEFAULT_INDEX ? " is-selected" : ""}" type="button" role="radio" aria-checked="${index === DEFAULT_INDEX}" aria-label="Choose ${escapeHtml(card.title)}" tabindex="${index === DEFAULT_INDEX ? "0" : "-1"}" data-index="${index}"><span class="candidate-number">0${index + 1}</span><span class="candidate-copy"><b>${escapeHtml(card.title)}</b><small>${escapeHtml(card.short)}</small></span><span class="candidate-arrow" aria-hidden="true">→</span></button>`).join("");
}

const defaultCard = CARDS[DEFAULT_INDEX];
const html = `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>Probatio SVM — pick an agent</title>
<style>
  :root { --bg:#070a11; --panel:#0d1320; --panel-2:#111a2a; --line:#29364d; --ink:#eaf0fb; --muted:#9ba9c2; --accent:#80d5ff; --safe:#56d88c; --risk:#ffaf5e; --risk-soft:rgba(255,145,78,.25); --danger:#ff8b76; }
  * { box-sizing:border-box; } html { overflow-x:hidden; } body { margin:0; min-width:320px; background:radial-gradient(900px 520px at 50% -5%, #162440 0%, var(--bg) 68%); color:var(--ink); font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif; }
  button { font:inherit; } .shell { width:min(1180px, calc(100% - 40px)); margin:0 auto; } header { padding:52px 0 24px; max-width:780px; } .brand, .eyebrow, .act-number, .consequence-kicker, .result-kicker { color:var(--accent); font-size:11px; font-weight:800; letter-spacing:.14em; text-transform:uppercase; }
  h1 { max-width:760px; margin:12px 0 12px; font-size:clamp(38px, 6vw, 64px); letter-spacing:-.05em; line-height:.98; } .hook { margin:0; color:var(--muted); font-size:clamp(17px, 2vw, 20px); line-height:1.55; } .instruction { display:inline-flex; margin:24px 0 0; padding:10px 13px; border:1px solid #3e5575; border-radius:999px; color:#d8e8ff; background:#101c2d; font-size:14px; }
  .section-label { margin:25px 0 12px; color:var(--muted); font-size:13px; } .roster { display:grid; grid-template-columns:repeat(3, minmax(0, 1fr)); gap:10px; margin-bottom:28px; } .candidate { width:100%; min-height:108px; display:flex; gap:10px; align-items:flex-start; padding:14px; color:var(--ink); text-align:left; cursor:pointer; border:1px solid var(--line); border-radius:13px; background:rgba(13,19,32,.84); transition:border-color .18s ease, background .18s ease, transform .18s ease; } .candidate:hover, .candidate:focus-visible { border-color:var(--accent); background:#142238; outline:none; transform:translateY(-2px); } .candidate.is-selected { border-color:var(--accent); background:linear-gradient(135deg, #172b45, #111a2a); box-shadow:inset 0 0 0 1px rgba(128,213,255,.18); } .candidate-number { color:var(--accent); font:700 11px ui-monospace,SFMono-Regular,Menlo,monospace; } .candidate-copy { min-width:0; flex:1; } .candidate-copy b { display:block; font-size:15px; } .candidate-copy small { display:block; margin-top:5px; color:var(--muted); font-size:12px; line-height:1.36; } .candidate-arrow { align-self:center; color:var(--muted); font-size:18px; }
  .stage { overflow:hidden; margin:0 0 48px; border:1px solid #3a4d6d; border-radius:20px; background:linear-gradient(135deg, #101a2b, #0b101a); box-shadow:0 25px 70px rgba(0,0,0,.3); } .stage-top { display:flex; justify-content:space-between; gap:20px; align-items:flex-start; padding:26px 28px 23px; border-bottom:1px solid var(--line); } .stage-top h2 { margin:7px 0 3px; font-size:clamp(26px, 4vw, 38px); letter-spacing:-.04em; } .stage-top p { margin:0; color:var(--muted); font-size:14px; } .stage-venue { margin-top:3px; color:var(--muted); font-size:12px; white-space:nowrap; } .acts { display:grid; grid-template-columns:.7fr 1.55fr .95fr; } .act { min-width:0; padding:27px 25px 30px; } .act + .act { border-left:1px solid var(--line); } .act h3 { margin:10px 0; color:var(--ink); font-size:19px; line-height:1.25; } .act p { color:var(--muted); font-size:13px; line-height:1.55; } .position-chip { display:inline-block; margin-top:9px; padding:8px 10px; border-radius:8px; background:#17253a; color:var(--muted); font-size:12px; } .position-chip b { color:var(--ink); }
  .reveal-act h3 { min-height:24px; color:#ffd2a4; } .reveal-act h3.is-safe { color:var(--safe); } .chart-shell { position:relative; margin-top:10px; } .replay-chart { width:100%; height:auto; display:block; overflow:visible; } .grid-line { stroke:#31405b; stroke-width:1; } .claim-line { stroke:var(--accent); stroke-width:2; stroke-dasharray:7 5; } .risk-band { fill:var(--risk-soft); } .chart-line { fill:none; stroke-width:3.5; stroke-linecap:round; stroke-linejoin:round; } .line-safe { stroke:var(--safe); } .line-risk { stroke:var(--risk); } .slot-dot { stroke:#0d1320; stroke-width:1.4; } .dot-safe { fill:var(--safe); } .dot-risk { fill:var(--risk); } .raw-value, .slot-label { fill:#aebbd0; font:12px ui-monospace,SFMono-Regular,Menlo,monospace; } .slot-label-end { text-anchor:end; } .axis-word { position:absolute; left:0; width:66px; color:#91a0b9; font-size:9px; font-weight:800; letter-spacing:.08em; line-height:1.1; } .axis-up { top:12%; } .axis-flat { top:48%; } .axis-down { bottom:18%; } .risk-label { margin:5px 0 0 86px; color:#ffd0a1; font-size:12px; line-height:1.35; } .chart-key { display:flex; flex-wrap:wrap; gap:10px 16px; margin-top:13px; color:var(--muted); font-size:11px; } .chart-key span { display:flex; align-items:center; gap:6px; } .key-line { width:18px; height:3px; border-radius:4px; background:var(--safe); } .key-promised { background:repeating-linear-gradient(90deg, var(--accent) 0 5px, transparent 5px 8px); } .off-chart-key i { width:10px; height:10px; border:2px solid var(--accent); border-radius:50%; }
  .result { padding:13px 0 2px; } .result h2 { margin:6px 0; font-size:23px; letter-spacing:-.03em; } .result p { margin:0; } .result-caught h2 { color:#ffd1a3; } .result-pass h2 { color:#b3f1c9; } .consequence { margin-top:17px; padding:14px; border-radius:10px; background:#172133; } .consequence strong { display:block; margin:6px 0 0; font-size:14px; line-height:1.35; } .consequence p { margin:7px 0 0; font-size:12px; } .consequence-loss { border-left:3px solid var(--danger); } .consequence-hidden { border-left:3px solid var(--accent); } .consequence-risk { border-left:3px solid var(--risk); } .consequence-pass { border-left:3px solid var(--safe); } .proof { margin-top:15px; padding:13px; border:1px solid var(--line); border-radius:10px; background:#101927; color:var(--muted); font-size:12px; } .proof-title { margin:0 0 9px; color:#d5e7ff; font-size:13px; } .proof-clear { margin:0 !important; } .proof ul { display:grid; gap:7px; margin:0; padding:0; list-style:none; } .proof li { padding-top:7px; border-top:1px solid var(--line); } .proof li:first-child { padding-top:0; border-top:0; } .proof li span { display:block; color:var(--ink); } .proof li b { color:var(--risk); font-weight:600; } .proof code { display:block; margin-top:2px; color:var(--muted); font-size:10px; }
  .how { padding:0 0 28px; } .how h2 { margin:0 0 15px; font-size:27px; letter-spacing:-.03em; } .how-grid { display:grid; grid-template-columns:repeat(3, 1fr); gap:14px; } .how-step { padding:18px; border:1px solid var(--line); border-radius:12px; background:var(--panel); } .how-step b { display:block; color:var(--accent); font-size:12px; } .how-step h3 { margin:8px 0 5px; font-size:16px; } .how-step p { margin:0; color:var(--muted); font-size:13px; line-height:1.45; } .cta { display:flex; justify-content:space-between; align-items:center; gap:20px; margin:12px 0 45px; padding:22px; border-radius:15px; background:linear-gradient(135deg, #193554, #14233a); } .cta h2 { margin:0; font-size:22px; } .cta p { margin:6px 0 0; color:#c0d0e5; font-size:13px; line-height:1.45; } .cta a { flex:0 0 auto; padding:11px 14px; border-radius:9px; background:var(--accent); color:#05101d; font-size:13px; font-weight:800; text-decoration:none; } footer { padding:0 0 44px; color:var(--muted); font-size:12px; line-height:1.55; } footer a { color:var(--accent); } .sr-only { position:absolute; width:1px; height:1px; padding:0; margin:-1px; overflow:hidden; clip:rect(0,0,0,0); white-space:nowrap; border:0; }
  .stage.is-replaying .chart-line { stroke-dasharray:1400; stroke-dashoffset:1400; animation:draw-line .85s steps(60, end) forwards; } .stage.is-replaying .slot-dot { opacity:0; animation:show-dots .85s steps(60, end) forwards; } .stage.is-replaying .risk-band { opacity:0; animation:show-risk .65s ease-out .18s forwards; } @keyframes draw-line { to { stroke-dashoffset:0; } } @keyframes show-dots { to { opacity:1; } } @keyframes show-risk { to { opacity:1; } }
  @media (max-width:900px) { .roster { grid-template-columns:repeat(2, minmax(0, 1fr)); } .acts { grid-template-columns:1fr 1.4fr; } .catch-act { grid-column:1 / -1; border-left:0 !important; border-top:1px solid var(--line); } }
  @media (max-width:620px) { .shell { width:min(100% - 28px, 1180px); } header { padding-top:34px; } .instruction { font-size:13px; } .roster { grid-template-columns:1fr; } .candidate { min-height:82px; } .stage { border-radius:15px; } .stage-top { padding:21px 18px 18px; } .stage-venue { display:none; } .acts { grid-template-columns:1fr; } .act { padding:22px 18px; } .act + .act { border-left:0; border-top:1px solid var(--line); } .reveal-act h3 { min-height:0; } .axis-word { left:0; width:55px; font-size:8px; } .replay-chart { width:calc(100% + 8px); margin-left:-8px; } .risk-label { margin-left:56px; } .how-grid { grid-template-columns:1fr; } .cta { display:block; } .cta a { display:inline-block; margin-top:15px; } }
  @media (prefers-reduced-motion:reduce) { *, *::before, *::after { animation-duration:.01ms !important; animation-iteration-count:1 !important; scroll-behavior:auto !important; transition-duration:.01ms !important; } }
</style></head>
<body><main class="shell">
  <header><div class="brand">PROBATIO SVM</div><h1>Would you trust an AI agent with a fund?</h1><p class="hook">An agent can promise to play it safe. Probatio replays its recorded on-chain actions — the permanent record of what it did — and shows when those actions betray that promise.</p><p class="instruction">Pick a candidate to manage the fund. Then see what Probatio finds.</p></header>
  <section aria-labelledby="roster-title"><h2 class="section-label" id="roster-title">Choose a candidate</h2><div class="roster" id="roster" role="radiogroup" aria-label="Candidate agents">${rosterMarkup()}</div></section>
  <section class="stage" id="stage" aria-labelledby="stage-title">${stageMarkup(defaultCard)}</section>
  <p class="sr-only" id="stage-status" aria-live="polite">Measurement gamer selected. Its actions broke its promise.</p>
  <section class="how" aria-labelledby="how-title"><h2 id="how-title">How Probatio checks an agent</h2><div class="how-grid"><article class="how-step"><b>01</b><h3>It makes a promise</h3><p>The agent states how it says it will manage risk.</p></article><article class="how-step"><b>02</b><h3>Probatio replays the record</h3><p>It reads the account state from the recorded episode, slot by slot.</p></article><article class="how-step"><b>03</b><h3>The proof is shown</h3><p>If what happened differs from the promise, the evidence slots explain why.</p></article></div></section>
  <section class="cta"><div><h2>Want to certify your own agent?</h2><p>Verdicts are computed offline by reading account state as ground truth — nothing to bypass.</p></div><a href="https://github.com/psyto/probatio-svm">See how to certify an agent →</a></section>
  <footer>Probatio is a static replay of committed certification episodes. <a href="https://github.com/psyto/probatio-svm">github.com/psyto/probatio-svm</a> · built by Claude Code + Codex, cross-reviewed.</footer>
</main>
<script>
const CARDS = ${DATA};
const defaultIndex = ${DEFAULT_INDEX};
const stage = document.getElementById('stage');
const roster = document.getElementById('roster');
const status = document.getElementById('stage-status');
const escapeHtml = value => String(value).replace(/[&<>'"]/g, character => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', "'": '&#39;', '"': '&quot;' })[character]);
const signed = value => value > 0 ? '+' + value : String(value);
const evidenceRange = slots => !slots?.length ? 'no slots recorded' : slots.length === 1 ? 'slot ' + slots[0] : 'slots ' + slots[0] + '–' + slots[slots.length - 1];
const hasFinding = (t, kind) => (t.findings || []).some(finding => finding.kind === kind);
function divergenceRanges(t) { const ranges = []; let start = null; t.slots.forEach((slot, index) => { const diverged = slot.measured_delta !== t.claimed_delta; if (diverged && start === null) start = index; if (start !== null && (!diverged || index === t.slots.length - 1)) { const end = diverged && index === t.slots.length - 1 ? index : index - 1; ranges.push([t.slots[start].slot, t.slots[end].slot]); start = null; } }); return ranges; }
const formatRanges = ranges => !ranges.length ? 'no recorded gap' : ranges.map(([from, to]) => from === to ? 'slot ' + from : 'slots ' + from + '–' + to).join(', ');
function findingText(finding) { return ({ ClaimMismatch: 'Its final claim did not match its account', ClaimTracksExposure: 'Its account carried an undisclosed position', ContinuousNeutrality: 'It was only flat when the check arrived', PhantomExposure: 'The risk sat in another wallet', IntraEpisodeInsolvency: 'The position went underwater' })[finding.kind] || finding.kind; }
function promiseText(t) { return t.claimed_delta === 0 ? 'It promised not to make a directional bet.' : 'It promised to keep a ' + (t.claimed_delta > 0 ? 'long' : 'short') + ' position of ' + signed(t.claimed_delta) + '.'; }
function chartMarkup(t) { const slots = t.slots, claim = t.claimed_delta, width = 700, height = 290, padLeft = 86, padRight = 26, padTop = 24, padBottom = 34, values = slots.map(slot => slot.measured_delta).concat([claim, 0]); let low = Math.min(...values), high = Math.max(...values); const span = Math.max(high - low, 1); low -= span * .22; high += span * .22; const x = index => padLeft + (index / Math.max(slots.length - 1, 1)) * (width - padLeft - padRight), y = value => (height - padBottom) - ((value - low) / (high - low)) * (height - padTop - padBottom), claimY = y(claim), isDiverged = slot => slot.measured_delta !== claim, bands = []; let start = null; slots.forEach((slot, index) => { if (isDiverged(slot) && start === null) start = index; if (start !== null && (!isDiverged(slot) || index === slots.length - 1)) { const end = isDiverged(slot) && index === slots.length - 1 ? index : index - 1, measured = [], claimed = []; for (let point = start; point <= end; point += 1) measured.push(x(point).toFixed(1) + ',' + y(slots[point].measured_delta).toFixed(1)); for (let point = end; point >= start; point -= 1) claimed.push(x(point).toFixed(1) + ',' + claimY.toFixed(1)); bands.push('<polygon class="risk-band" points="' + measured.concat(claimed).join(' ') + '"/>'); start = null; } }); const segments = []; let segment = null; slots.forEach((slot, index) => { const diverged = isDiverged(slot); if (!segment || segment.diverged !== diverged) { segment = { diverged, points: [] }; segments.push(segment); if (index > 0) segment.points.push([x(index - 1), y(slots[index - 1].measured_delta)]); } segment.points.push([x(index), y(slot.measured_delta)]); }); const lines = segments.map(current => '<polyline class="chart-line ' + (current.diverged ? 'line-risk' : 'line-safe') + '" points="' + current.points.map(point => point[0].toFixed(1) + ',' + point[1].toFixed(1)).join(' ') + '"/>').join(''), dots = slots.map((slot, index) => '<circle class="slot-dot ' + (isDiverged(slot) ? 'dot-risk' : 'dot-safe') + '" cx="' + x(index).toFixed(1) + '" cy="' + y(slot.measured_delta).toFixed(1) + '" r="' + (slots.length > 20 ? '2.2' : '3.5') + '"/>').join(''), flatY = y(0); return '<div class="chart-shell"><div class="axis-word axis-up">BETTING UP</div><div class="axis-word axis-flat">FLAT / NEUTRAL</div><div class="axis-word axis-down">BETTING DOWN</div><svg class="replay-chart" viewBox="0 0 ' + width + ' ' + height + '" role="img" aria-label="The solid line is the recorded position for each slot; the dashed line is the position the agent promised."><line class="grid-line" x1="' + padLeft + '" y1="' + padTop + '" x2="' + (width - padRight) + '" y2="' + padTop + '"/><line class="grid-line" x1="' + padLeft + '" y1="' + flatY.toFixed(1) + '" x2="' + (width - padRight) + '" y2="' + flatY.toFixed(1) + '"/><line class="grid-line" x1="' + padLeft + '" y1="' + (height - padBottom) + '" x2="' + (width - padRight) + '" y2="' + (height - padBottom) + '"/>' + bands.join('') + '<line class="claim-line" x1="' + padLeft + '" y1="' + claimY.toFixed(1) + '" x2="' + (width - padRight) + '" y2="' + claimY.toFixed(1) + '"/>' + lines + dots + '<text class="raw-value" x="' + (padLeft + 8) + '" y="' + Math.max(padTop + 15, claimY - 8).toFixed(1) + '">said ' + signed(claim) + '</text><text class="raw-value" x="' + (padLeft + 8) + '" y="' + Math.min(height - padBottom - 8, y(slots[0].measured_delta) + 18).toFixed(1) + '">recorded ' + signed(slots[0].measured_delta) + '</text><text class="slot-label" x="' + padLeft + '" y="' + (height - 9) + '">slot ' + slots[0].slot + '</text><text class="slot-label slot-label-end" x="' + (width - padRight) + '" y="' + (height - 9) + '">slot ' + slots[slots.length - 1].slot + '</text></svg>' + (bands.length ? '<div class="risk-label">Undisclosed directional risk — the bet it did not admit to</div>' : '') + '</div>'; }
function consequenceMarkup(t) { const insolvency = (t.findings || []).find(finding => finding.kind === 'IntraEpisodeInsolvency'), phantom = hasFinding(t, 'PhantomExposure'); if (insolvency) { const before = t.slots.find(slot => slot.slot < insolvency.evidence_slots[0])?.mark ?? t.slots[0].mark, after = t.slots.find(slot => slot.slot >= insolvency.evidence_slots[0])?.mark ?? t.slots.at(-1).mark, drop = Math.round((1 - after / before) * 100); return '<div class="consequence consequence-loss"><span class="consequence-kicker">Why it matters</span><strong>The episode price fell ' + drop + '% at slot ' + insolvency.evidence_slots[0] + ': ' + before + ' → ' + after + '.</strong><p>' + (phantom ? 'The hidden wallet’s long position' : 'That undisclosed position') + ' went underwater in ' + evidenceRange(insolvency.evidence_slots) + '. A fund needs that risk surfaced before the loss, not after it.</p></div>'; } if (phantom) return '<div class="consequence consequence-hidden"><span class="consequence-kicker">Why it matters</span><strong>The visible line is flat because the risk is off this chart.</strong><p>The exposure sits in another wallet, so the fund would be relying on an incomplete picture.</p></div>'; if (t.verdict === 'Pass') return '<div class="consequence consequence-pass"><span class="consequence-kicker">Why it matters</span><strong>It kept the position it said it would keep.</strong><p>Probatio found no mismatch between this promise and the recorded account state.</p></div>'; return '<div class="consequence consequence-risk"><span class="consequence-kicker">Why it matters</span><strong>The fund was carrying risk it did not agree to.</strong><p>Even without a recorded insolvency event, the promise and the account state do not match.</p></div>'; }
function resultMarkup(t) { const phantom = hasFinding(t, 'PhantomExposure'); if (t.verdict === 'Pass') return '<div class="result result-pass"><span class="result-kicker">Certification result</span><h2>Kept its word</h2><p>The recorded position matched the promise throughout this replay.</p></div>'; const headline = phantom ? 'Caught — risk was hidden in another wallet' : 'Caught — its actions broke its promise', copy = phantom ? 'Its visible account stayed flat, but Probatio found the directional exposure elsewhere.' : 'The recorded position differed from the promise in ' + formatRanges(divergenceRanges(t)) + '.'; return '<div class="result result-caught"><span class="result-kicker">Certification result</span><h2>' + headline + '</h2><p>' + copy + '</p></div>'; }
function proofMarkup(t) { if (!t.findings?.length) return '<section class="proof" aria-label="The evidence"><h3 class="proof-title">The evidence</h3><p class="proof-clear">No mismatch found — the recorded position matched the claim of <code>position ' + signed(t.claimed_delta) + '</code>.</p></section>'; return '<section class="proof" aria-label="The evidence"><h3 class="proof-title">The evidence</h3><ul>' + t.findings.map(finding => '<li><span>' + escapeHtml(findingText(finding)) + '</span><b>' + evidenceRange(finding.evidence_slots) + '</b><code>' + escapeHtml(finding.kind) + '</code></li>').join('') + '</ul></section>'; }
function stageMarkup(card) { const t = card.t, divergence = divergenceRanges(t), gap = formatRanges(divergence), phantom = hasFinding(t, 'PhantomExposure'), matchesPromise = divergence.length === 0, revealHeadline = phantom ? 'The line is flat. The risk is not.' : matchesPromise ? 'The record matches the promise.' : 'The gap appears in ' + gap + '.'; return '<div class="stage-top"><div><span class="eyebrow">Probatio certification replay</span><h2 id="stage-title">' + escapeHtml(card.title) + '</h2><p>' + escapeHtml(card.short) + '</p></div><span class="stage-venue">' + escapeHtml(card.venue) + '</span></div><div class="acts"><article class="act promise-act"><span class="act-number">1 · THE PROMISE</span><h3>' + promiseText(t) + '</h3><p><b>Plain English:</b> ' + (t.claimed_delta === 0 ? '“Neutral” means it promised to bet neither up nor down.' : 'A long position is a bet that the price will go up; a short position is the opposite.') + '</p><div class="position-chip">Said position: <b>' + signed(t.claimed_delta) + '</b></div></article><article class="act reveal-act"><span class="act-number">2 · WHAT THE RECORD SHOWS</span><h3 class="' + (matchesPromise && !phantom ? 'is-safe' : '') + '">' + revealHeadline + '</h3>' + chartMarkup(t) + '<div class="chart-key"><span><i class="key-line key-recorded"></i>recorded position</span><span><i class="key-line key-promised"></i>promised position</span>' + (phantom ? '<span class="off-chart-key"><i></i>risk found in another wallet</span>' : '') + '</div></article><article class="act catch-act"><span class="act-number">3 · THE CATCH</span>' + resultMarkup(t) + consequenceMarkup(t) + proofMarkup(t) + '</article></div>'; }
function selectCandidate(index, replay) { const card = CARDS[index]; stage.innerHTML = stageMarkup(card); stage.classList.toggle('is-replaying', replay && !window.matchMedia('(prefers-reduced-motion: reduce)').matches); roster.querySelectorAll('.candidate').forEach((button, buttonIndex) => { const selected = buttonIndex === index; button.classList.toggle('is-selected', selected); button.setAttribute('aria-checked', String(selected)); button.tabIndex = selected ? 0 : -1; }); status.textContent = card.title + ' selected. ' + (card.t.verdict === 'Pass' ? 'It kept its word.' : 'Its actions broke its promise.'); }
roster.addEventListener('click', event => { const button = event.target.closest('.candidate'); if (button) selectCandidate(Number(button.dataset.index), true); });
roster.addEventListener('keydown', event => { if (!['ArrowRight', 'ArrowDown', 'ArrowLeft', 'ArrowUp', 'Home', 'End'].includes(event.key)) return; const buttons = Array.from(roster.querySelectorAll('.candidate')); const current = buttons.indexOf(document.activeElement); if (current < 0) return; event.preventDefault(); let next = current; if (event.key === 'ArrowRight' || event.key === 'ArrowDown') next = (current + 1) % buttons.length; if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') next = (current - 1 + buttons.length) % buttons.length; if (event.key === 'Home') next = 0; if (event.key === 'End') next = buttons.length - 1; buttons[next].focus(); selectCandidate(next, true); });
</script></body></html>`;

fs.writeFileSync(path.join(dir, "index.html"), html);
console.log(`wrote web/index.html (${CARDS.length} candidates)`);

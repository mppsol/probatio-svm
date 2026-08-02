#!/usr/bin/env node
// Probatio → Solana Agent Registry (Reputation) attestation preparer — Path A.
//
// SAFETY: this tool PREPARES and VALIDATES the giveFeedback call; it does NOT submit. There is no live
// send path here — no SDK import, no keypair read, no network — because the 8004-solana API is not yet
// verified against a pinned version (Codex review 022, P1). Enabling the real submit is task 022b, done
// only after pinning + verifying the SDK in a non-sending fixture.
//
// Input: the FeedbackCall JSON emitted by the Rust harness `certify-jupiter --attest <asset>`:
//   {"agent":"<base58 asset>","value":100,"tag":"re-exec","feedback_uri":"ipfs://<pinned receipt>"}

import { readFileSync } from 'node:fs';

const REPUTATION_PROGRAM_MAINNET = '8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ';
const BASE58 = /^[1-9A-HJ-NP-Za-km-z]+$/;

function die(msg, code = 2) {
  console.error(`error: ${msg}`);
  process.exit(code);
}

function parseArgs(argv) {
  const a = { rpc: 'https://api.devnet.solana.com', send: false };
  const seen = new Set();
  const takeValue = (i, flag) => {
    const v = argv[i + 1];
    if (v === undefined || v.startsWith('--')) die(`${flag} requires a value`);
    return v;
  };
  for (let i = 0; i < argv.length; i++) {
    const k = argv[i];
    const valueFlag = (flag, key) => {
      if (seen.has(flag)) die(`duplicate ${flag}`);
      seen.add(flag);
      a[key] = takeValue(i, flag);
      i += 1;
    };
    if (k === '--send') a.send = true;
    else if (k === '--call') valueFlag('--call', 'call');
    else if (k === '--feedback-uri') valueFlag('--feedback-uri', 'feedbackUri');
    else if (k === '--keypair') valueFlag('--keypair', 'keypair');
    else if (k === '--rpc') valueFlag('--rpc', 'rpc');
    else die(`unknown arg: ${k}`);
  }
  return a;
}

function isBase58Pubkey(s) {
  return typeof s === 'string' && s.length >= 32 && s.length <= 44 && BASE58.test(s);
}

// Bind the bridge to Task 021's FeedbackCall shape — never sign arbitrary JSON.
function validateCall(call, feedbackUri) {
  const errs = [];
  if (!isBase58Pubkey(call.agent)) errs.push('agent must be a base58 pubkey (32–44 chars)');
  if (!Number.isInteger(call.value) || call.value < 0 || call.value > 100)
    errs.push('value must be an integer in 0..=100');
  if (call.tag !== 're-exec') errs.push('tag must be exactly "re-exec"');
  const placeholder = feedbackUri === undefined || feedbackUri === '' || feedbackUri === 'ipfs://pending';
  return { errs, placeholder };
}

const args = parseArgs(process.argv.slice(2));
if (args.call === undefined) {
  die(
    'usage: node send.mjs --call <feedbackcall.json|-> [--feedback-uri <uri>] [--rpc <url>] [--send --keypair <path>]'
  );
}

let call;
try {
  call = JSON.parse(readFileSync(args.call === '-' ? 0 : args.call, 'utf8'));
} catch (e) {
  die(`could not read/parse --call JSON: ${e.message}`);
}

// An explicitly supplied empty --feedback-uri is invalid, not a fallback to the call's URI.
if (Object.prototype.hasOwnProperty.call(args, 'feedbackUri') && args.feedbackUri === '') {
  die('--feedback-uri was empty');
}
const feedbackUri = args.feedbackUri ?? call.feedback_uri;
const { errs, placeholder } = validateCall(call, feedbackUri);

const plan = {
  registry: 'Solana Agent Registry — Reputation (giveFeedback)',
  program_mainnet: REPUTATION_PROGRAM_MAINNET,
  rpc: args.rpc,
  agentAsset: call.agent,
  value: String(call.value),
  tag1: call.tag,
  feedbackUri: feedbackUri ?? null,
};

if (!args.send) {
  console.log('DRY RUN — nothing sent. Planned giveFeedback:');
  console.log(JSON.stringify(plan, null, 2));
  if (errs.length) console.log('\nvalidation warnings:\n - ' + errs.join('\n - '));
  if (placeholder) console.log('\nNOTE: feedback_uri is a placeholder — pin the receipt and pass --feedback-uri before enabling a send.');
  process.exit(0);
}

// ---- --send: strict validate, then REFUSE (live submit is disabled until task 022b) --------------
if (errs.length) die('invalid FeedbackCall:\n - ' + errs.join('\n - '));
if (placeholder) die('feedback_uri is a placeholder — pin the receipt and pass a real --feedback-uri first');
if (!args.keypair) die('--send would require --keypair <path> once live submit is enabled');

console.log('VALIDATED & READY — but LIVE SUBMISSION IS DISABLED (task 022b, pending pinned+verified 8004-solana SDK).');
console.log('Nothing was sent. The call that will be submitted once enabled:');
console.log(JSON.stringify(plan, null, 2));
process.exit(0);

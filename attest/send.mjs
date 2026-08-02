#!/usr/bin/env node
// Probatio → Solana Agent Registry (Reputation) attestation sender — Path A.
//
// DEFAULT = dry-run: no network, no keypair, no SDK import, nothing sent. It just prints the exact
// giveFeedback call that WOULD be submitted. Actual submission requires --send + --keypair.
//
// Input: the FeedbackCall JSON emitted by the Rust harness `certify-jupiter --attest <asset>`:
//   {"agent":"<base58 asset>","value":100,"tag":"re-exec","feedback_uri":"ipfs://<pinned receipt>"}
//
// 8004-solana SDK call shape (per docs — VERIFY against the installed version before --send):
//   sdk.giveFeedback(agentAsset, { value: '100', tag1: 're-exec', feedbackUri: '<uri>' })

import { readFileSync } from 'node:fs';

function parseArgs(argv) {
  const a = { rpc: 'https://api.devnet.solana.com', send: false };
  for (let i = 0; i < argv.length; i++) {
    const k = argv[i];
    if (k === '--send') a.send = true;
    else if (k === '--call') a.call = argv[++i];
    else if (k === '--receipt') a.receipt = argv[++i];
    else if (k === '--feedback-uri') a.feedbackUri = argv[++i];
    else if (k === '--keypair') a.keypair = argv[++i];
    else if (k === '--rpc') a.rpc = argv[++i];
    else { console.error(`unknown arg: ${k}`); process.exit(2); }
  }
  return a;
}

function readJson(path) {
  // '-' or omitted → stdin (fd 0), so the Rust CLI's FeedbackCall line can be piped in.
  return JSON.parse(readFileSync(path === undefined || path === '-' ? 0 : path, 'utf8'));
}

const args = parseArgs(process.argv.slice(2));
if (args.call === undefined) {
  console.error(
    'usage: node send.mjs --call <feedbackcall.json|-> [--feedback-uri <uri>] [--rpc <url>] [--send --keypair <path>]'
  );
  process.exit(2);
}

const call = readJson(args.call);
if (!call.agent) {
  console.error('call JSON missing "agent" (base58 asset pubkey)');
  process.exit(2);
}
const feedbackUri = args.feedbackUri ?? call.feedback_uri;
const value = String(call.value);
const tag = call.tag ?? 're-exec';
const placeholder = feedbackUri === undefined || feedbackUri === '' || feedbackUri === 'ipfs://pending';

const plan = {
  registry: 'Solana Agent Registry — Reputation (giveFeedback)',
  program_mainnet: '8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ',
  rpc: args.rpc,
  agentAsset: call.agent,
  value,
  tag1: tag,
  feedbackUri: feedbackUri ?? null,
};

if (!args.send) {
  console.log('DRY RUN — nothing sent. Planned giveFeedback:');
  console.log(JSON.stringify(plan, null, 2));
  if (placeholder) {
    console.log('\nNOTE: feedback_uri is a placeholder — pin the receipt (IPFS/HTTPS) and pass --feedback-uri before --send.');
  }
  console.log('\nTo submit: node send.mjs --call <file> --feedback-uri <pinned-uri> --send --keypair <path> [--rpc <url>]');
  console.log('(requires: `npm install` in attest/; a funded devnet keypair; verify the 8004-solana method signature.)');
  process.exit(0);
}

// ---- live submission (only reached with --send) --------------------------------------------------
if (!args.keypair) {
  console.error('--send requires --keypair <path to a solana keypair json>');
  process.exit(2);
}
if (placeholder) {
  console.error('refusing to send: pin the receipt and pass a real --feedback-uri first (not ipfs://pending).');
  process.exit(2);
}

let web3, sdkmod;
try {
  web3 = await import('@solana/web3.js');
  sdkmod = await import('8004-solana');
} catch (e) {
  console.error('missing deps — run `npm install` in attest/ first. Import error:', e.message);
  process.exit(1);
}

try {
  const { Connection, Keypair } = web3;
  const secret = JSON.parse(readFileSync(args.keypair, 'utf8'));
  const kp = Keypair.fromSecretKey(Uint8Array.from(secret));
  const connection = new Connection(args.rpc, 'confirmed');
  // VERIFY this constructor + method against the installed 8004-solana version before trusting a send.
  const Sdk = sdkmod.default ?? sdkmod.Sdk ?? sdkmod.AgentRegistry;
  if (typeof Sdk !== 'function') {
    throw new Error('could not resolve the 8004-solana SDK constructor — check the installed package exports');
  }
  const sdk = new Sdk({ connection, keypair: kp });
  const res = await sdk.giveFeedback(call.agent, { value, tag1: tag, feedbackUri });
  console.log('submitted giveFeedback:', JSON.stringify(res));
} catch (e) {
  console.error('send failed (verify SDK API/version, keypair funding, and RPC):', e?.message ?? e);
  process.exit(1);
}

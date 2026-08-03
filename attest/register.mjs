#!/usr/bin/env node
// One-off: register a devnet agent (Metaplex Core NFT) in the Solana Agent Registry so Probatio has a
// target to attest about. Writes on-chain with the signer keypair. Usage:
//   node register.mjs [<keypair.json>] [<tokenUri>]
import { readFileSync } from 'node:fs';
import { Keypair } from '@solana/web3.js';
import { SolanaSDK } from '8004-solana';

const keypairPath = process.argv[2] || `${process.env.HOME}/.config/solana/id.json`;
const tokenUri = process.argv[3] || 'https://probatio.psyto.dev/agents/probatio-test.json';

const kp = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(readFileSync(keypairPath, 'utf8'))));
const sdk = new SolanaSDK({ cluster: 'devnet', signer: kp });

console.error(`registering a devnet agent — signer=${kp.publicKey.toBase58()} tokenUri=${tokenUri} ...`);
const res = await sdk.registerAgent(tokenUri);
const asset = res?.asset?.toBase58?.() ?? String(res?.asset ?? '');
const signatures = res?.signatures ?? (res?.signature ? [res.signature] : []);
console.log(JSON.stringify({ asset, signatures }, null, 2));

# P1 real target — independent review, round 2

**Verdict: APPROVE.** The corrected evidence supports the existing **KILLED** verdict. I found **no on-chain agent or agent vault this harness can certify**; there is no address to name.

This is a new read-only mainnet recomputation, not a replay of Claude's output. My registry snapshot was slot **440482647** and my saved full Jupiter `Position` snapshot was slot **440481597**. Owner-account lookups necessarily occurred just after that Jupiter snapshot.

## Resolution of r1 requirements

| r1 requirement | Result | Reproducible evidence |
|---|---|---|
| 1. Schema-driven `AgentAccount` decoder | **resolved** | `npm pack 8004-solana@0.8.3 --cache /tmp/p1-r2-npm --pack-destination /tmp/p1-r2-npm`; `tar -xOf /tmp/p1-r2-npm/8004-solana-0.8.3.tgz package/dist/core/borsh-schemas.js \| sed -n '180,350p'` gives exactly `collection, creator, owner, asset, bump, atom_enabled, agent_wallet: Option<[32]>,` three `(digest[32], u64)` pairs, `parent_asset: Option<[32]>, parent_locked, col_locked,` then three strings. `decode_agent_account` walks that order at [p1_real_target.py:113](../docs/decisions/repro/p1_real_target.py:113), including both tags; mutating either tag to `2` raises `ValueError`. At slot 440482647 all 1,471 records had valid tags and all three strings valid UTF-8. Their Borsh content ended at offsets 279–495, with four nonzero allocated tails, so not requiring byte 748 is correct. |
| 2. Regenerate affected figures and status | **resolved** | `grep -RIn --exclude-dir=.git --exclude='*.pyc' -E '2,915\|2915' STATUS.md docs/decisions/P1-real-target.md` finds only the two explicit forensic comparisons `2,915 -> 1,767`, not a current result. The document and STATUS use 1,767 and internally consistent slot-440479 figures. My independently recomputed figures below are later-slot drift, not a stale decoder. |
| 3. Preserve corrected `6 / 0` | **resolved** | At registry slot 440482647 and Jupiter slot 440481597: **1,767 ∩ 418,137 historical owners = 6**, and **1,767 ∩ 4,745 live owners = 0**. The six historical keys are unchanged: `3EtbBbCQvwJjd29yXEEvKGgmradBjK6Pe5fRhUUQxMLT`, `5MAr6zMFpHBQycSVHZn4QiPNea5V8EoxmEAW3EPFEouF`, `BGfybQ2uFGPmCscCPAgJtBFXDWc5GNqzymSq3AAo6Nvi`, `DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw`, `HDJ88KsVwUGxGZmEdKtgMxHvssZR4gfFp1v1izCPK5x9`, and `HbT4PCNJhhZQLbLJ7qQMaMjeBiw3qC4QQnLse4cFvWG7`. |
| 4. Invalid nonzero-side `Position` bytes fail closed | **resolved** | [p1_real_target.py:206](../docs/decisions/repro/p1_real_target.py:206) raises for a nonzero `sizeUsd` whose side is not 1 or 2; it no longer continues. My direct JSON decode of all 864,496 accounts at slot 440481597 found **5,702** nonzero positions and **0** invalid sides, so fail-closed parsing changes no population. |

### Schema details and the added `parent_asset`

The new decoder correctly recovers `agent_wallet`: **50** populated fields, **41** distinct wallets. At slot 440482037 the per-field distinct counts were creator/owner/asset/wallet/parent = `179 / 261 / 1,471 / 41 / 0`; their union is **1,767**. Thus `parent_asset` is a schema-correct identity reference but is absent in every current record: it adds **zero** keys and cannot pad or double-count the published 1,767 number.

The string checks do more than merely avoid a crash: after fixed fields and both tagged options they consume three length-prefixed UTF-8 fields in the SDK's order for every record. This is a useful alignment check, while the actual proof of the field order is the published schema above. The nonzero allocated tails show why an exact `o == 748` check would be wrong.

## Independent headline figures

| Definition | Independent result | Direction versus decision document |
|---|---:|---|
| AgentAccount records / distinct identity union | 1,471 / **1,767** | none |
| agent_wallet populated / parent_asset populated | 50 / 0 | none |
| Position accounts / nonzero positions | 864,496 / 5,702 | slot drift |
| historical Position owners / live owners | 418,137 / **4,745** | later snapshot; larger live search space favours the project |
| identity intersection: ever / live | **6 / 0** | none |
| total gross open notional | $65,123,139 | slot drift |
| System-owned wallets | 4,401, $62,743,274 | slot drift |
| nonexistent holders / off-curve | 339 / **0**, $2,347,442 | slot drift; zero off-curve is unchanged |
| program-owned holders | 5, $32,423 | none |
| PASS / dust PASS / hedged PASS / FLAG | 793 / 780 / 13 / 3,952 | slot drift |
| PASS / FLAG / dust-of-PASS share | 16.71% / 83.29% / 98.4% | slot drift |

I generated the Position snapshot with:

```sh
python3 -B docs/decisions/repro/p1_real_target.py --keep /tmp/p1-r2-jupiter.json
```

and independently decoded its JSON response (rather than trusting the script's streaming regex) to obtain the Position/intersection/PASS rows. I resolved all 4,745 owners using read-only `getMultipleAccounts` calls in batches of 100. The three gross holder categories sum exactly to the total: `62,743,274 + 2,347,442 + 32,423 = 65,123,139`.

## Ed25519 / nonexistent-holder attack

The implementation at [p1_real_target.py:55](../docs/decisions/repro/p1_real_target.py:55) is the same validity predicate Solana uses for `bytes_are_curve_point`: `CompressedEdwardsY::decompress().is_some()` (see the locally installed `solana-address-2.6.1/src/lib.rs:188`). It masks the sign bit, checks canonical `y < p`, and its two square-root residue cases are the standard Ed25519 decompression test.

I independently derived two PDAs with `@solana/web3.js@1.98.4` in `/tmp` and compared it to the Python predicate:

```text
PublicKey.findProgramAddressSync([Buffer.from('r2-independent-pda')], SystemProgram.programId)
  = EVQKCzH98UPcZTYxgWqJEBwGoR6mvPGMSJ9h2cXS732n, bump 254, SDK isOnCurve=false, Python=false
PublicKey.findProgramAddressSync([Buffer.from('position')], SystemProgram.programId)
  = 5wCu1mmocjayUWJP6A1rvz6qfc2XmjvSsrE5jWnaoXAS, bump 254, SDK isOnCurve=false, Python=false
```

It also returns true for the encoded Ed25519 base point and `11111111111111111111111111111111`, and false for noncanonical `y = p`. Therefore there is no PDA-to-wallet false positive: Solana's PDA derivation explicitly selects an off-curve hash. Rechecking all **339** currently missing holders returned **0** off-curve.

One wording qualification, not a gate finding: on-curve proves *not a Solana PDA*; without a current account it does not positively prove an address was an ordinary EOA rather than some other closed, on-curve account. This ambiguity does not create a live, program-controlled vault target and does not affect the `0` registry intersection. It is therefore not grounds for CHANGES.

## Fixture correction

The committed fixture is correct, and r1's lookup typo was mine. Decoding `crates/harness/src/testdata/jupiter_gpa_owner.json` gives discriminator `aabc8fe47a40f7d0` and `owner@8 = AhUvhrHHZXh7Huu8AtCfEkTvgUdTw4ZwL1j1c6Fu5dfq` (not `AhUvhrHH9i2acgGzPwwP5Nmew1NMVVpnWcZyHtC5B8VA`). A fresh read-only run was:

```sh
cargo run -q -p probatio-svm-harness -- certify-jupiter --live \
  AhUvhrHHZXh7Huu8AtCfEkTvgUdTw4ZwL1j1c6Fu5dfq
# slot 440481528: 1 open position, net signed notional $16, PASS
```

This confirms the $16/below-$50-band PASS claim for the correct address.

## Other finding

**P2 — committed generated bytecode.** Commit `03b1724` includes `docs/decisions/repro/__pycache__/p1_real_target.cpython-311.pyc`. It has no bearing on the experiment or verdict, but a generated, version-specific binary should not be part of a source evidence commit. Direction: none.

## What would change my verdict

Either (1) a corrected registry identity/operational wallet with an open Jupiter Position, or (2) on-chain evidence that a live program-controlled Position owner is an autonomous-agent vault, followed by a harness run for that address. Neither exists in this review.


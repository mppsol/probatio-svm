# P1 real target — independent review

**Verdict: CHANGES**

The P1 conclusion remains `KILLED` after the required corrected computation: I found **no on-chain
agent or agent vault that this harness can certify**.  However, the committed reproduction and decision
misdecode the Solana Agent Registry record, so the deciding `2,915` agent-identity number is not a
number derived from the registry's schema.  `docs/GATE.md` requires the evidence itself to be a
reproducible number; this artifact must be corrected before the kill is accepted.

Snapshot RPC: `https://api.mainnet-beta.solana.com`, read-only.  My independent complete snapshot was
registry slot **440477376**, Jupiter slot **440477379**.  The committed script was also run unchanged at
slots **440477062 / 440477066**.

## P0 — Registry identity decoder is wrong

The `748`-byte account is `AgentAccount`, not a four-pubkey record at offsets `40/72/104/136`.
The repository's locked `8004-solana@0.8.3` SDK is independently obtainable and specifies:

```sh
npm pack 8004-solana@0.8.3 --cache /tmp/p1-npm-cache --pack-destination /tmp
tar -xOf /tmp/8004-solana-0.8.3.tgz package/dist/core/borsh-schemas.js | sed -n '198,326p'
```

Its Borsh schema, after the eight-byte discriminator, is `collection@8`, `creator@40`,
`owner@72`, `asset@104`, `bump@136:u8`, `atom_enabled@137:u8`, and
`agent_wallet: Option<Pubkey>` tag at `138` with a pubkey value at `139..171` only when the tag is
one.  The committed `AGENT_PUBKEY_OFFSETS=(40,72,104,136)` therefore decodes one byte of bump, two
flags, and 29 digest bytes as a purported `signer`, while omitting the actual optional operational
wallet.

My independent decoder asserted every option tag was `0` or `1`, collected `40/72/104` and all
`139..171` values where tag `138==1`, then intersected that set with a fresh full Jupiter `Position`
scan.  Result:

```text
registry slot 440477376: 1,471 x 748-byte AgentAccounts; 50 wallet fields, 41 distinct
correct identity/operational-wallet union: 1,767
Jupiter slot 440477379: correct-union ∩ all historical position owners = 6
Jupiter slot 440477379: correct-union ∩ live position owners = 0
```

Direction: this is **non-monotone**, hence cannot be waved away as a conservative widening.  Omitting
the 41 real `agent_wallet` values favours the **KILL** (it can miss the target); adding 1,182
non-identity byte strings favours the **project** (it widens the chance of a hit).  The corrected
run resolves both directions and still returns zero live targets.  The six historical addresses are:
`3EtbBbCQvwJjd29yXEEvKGgmradBjK6Pe5fRhUUQxMLT`,
`5MAr6zMFpHBQycSVHZn4QiPNea5V8EoxmEAW3EPFEouF`,
`BGfybQ2uFGPmCscCPAgJtBFXDWc5GNqzymSq3AAo6Nvi`,
`DevFFyNWxZPtYLpEjzUnN1PFc9Po6PH7eZCi9f3tTkTw`,
`HDJ88KsVwUGxGZmEdKtgMxHvssZR4gfFp1v1izCPK5x9`, and
`HbT4PCNJhhZQLbLJ7qQMaMjeBiw3qC4QQnLse4cFvWG7`; none was live.

The other registry account sizes do not rescue the result.  Fresh `getProgramAccounts` returned
`{748: 1471, 332: 36, 73: 2}`.  The 332-byte accounts are metadata entries beginning with an asset
pubkey at offset 8 followed by tagged strings; they add no owner/operator/vault address.  The two
73-byte config accounts contain only the base collection and authority.  I also added both config
pubkeys (`DbjsWo7iUs7QZyJxLgNyVxvAAjQZCXroJHoGok8h8Umg` and
`DVYnMNDxEHQGpyYqvkBVkXXoJ3jPWtJNAbgKNJQu1hiZ`) to the live intersection: **0**.

## P1 — Vault and nonexistent-owner attack

The five extant non-System owners are real zero-data program-owned authority accounts, not merely an
inference from a missing account.  At the snapshot they were:

| Position owner | Owner program | Gross USD |
|---|---|---:|
| `68jj5xZcg2bh9HNC8SRGhSCFfqzCCWBGau2i6kzcxvL8` | `FziSMN4a4sxk3bst3hcw2uqnXW8Kq2DQf3MGVjjEjqW` | 29,897 |
| `FKC6yFsPivHiQE5Q49SdFVrNLREe2d1RFuMU3EHmUoqg` | `H67ehFrMLhFn6xVRymY2GaXMhr1LKH9mdaSEnXnQRKNa` | 1,441 |
| `8tFUWmueG5zHD1PRXopNonP287nPFpMcFbERPDNMoj5Q` | `syspv6Qe5BbK8GPEjLbMnmF9hZy7juAuePbuspciCFP` | 482 |
| `5uco4oZhDLPvP26z3j5rgXxex4aHfWSUxbps47XJtWYs` | `BUyLRaYEYHeorkrw3e9dvTuWQwq1HD6ioTXkAeMX6uWS` | 394 |
| `FvwMiwJA91q8zN4nvAMKKT9RYnLwekEkJLBTdDCEqVVw` | `syspv6Qe5BbK8GPEjLbMnmF9hZy7juAuePbuspciCFP` | 209 |

`getMultipleAccounts` reports each holder as `space=0`, `executable=false`; each listed owner is an
executable program.  I found no on-chain naming or Registry linkage which establishes any of the five
as an autonomous-agent vault.  That is not enough to name an agent target under P1.

The stronger missing-account check is favourable to the project: the **335** nonexistent owners in my
snapshot were all Ed25519 **on-curve**.  I base58-decoded each 32-byte key and tested the Edwards25519
curve equation; result `off_curve = 0`.  A Solana PDA is necessarily off-curve, so none is a
rent-collected PDA vault.  This closes the specific 334-holder/PDA hypothesis rather than assuming
that `getAccountInfo == null` implies a wallet.

## Independent figures

The table uses the corrected registry decoder and the snapshot stated above.  Differences in live
Jupiter holdings are slot drift, not a retroactive claim that the old snapshot was malformed; the
registry-layout row is a decoding error.

| Definition | Decision doc | Independent result @ 440477376 / 440477379 | Discrepancy direction |
|---|---:|---:|---|
| 748-byte agent records | 1,471 | 1,471 | none |
| distinct agent identities | 2,915 | **1,767** | P0, non-monotone; corrected as above |
| identities ever holding Position | 6 | 6 | none |
| identities with live Position | 0 | 0 | none |
| live Position owners | 4,689 | 4,693 | slot drift; old smaller zero-hit search favours project |
| System owners | 4,350 | 4,353 | slot drift; old smaller human-wallet set favours project |
| nonexistent owners | 334 | 335 | slot drift; old smaller ambiguous set favours project; all tested on-curve |
| program-owned owners | 5 | 5 | none |
| total gross USD | $65,494,447 | $64,732,995 | slot drift; old larger unattributed market favours KILL |
| System-owner gross USD | $63,154,199 | $62,389,501 | slot drift; old larger number favours KILL |
| nonexistent-owner gross USD | $2,307,825 | $2,311,071 | slot drift; old lower amount favours project; all are on-curve |
| program-owner gross USD | $32,423 | $32,423 | none |
| PASS | 791 | 792 | slot drift; old lower PASS count favours KILL |
| dust PASS (`gross < $50`) | 779 | 779 | count unchanged |
| non-dust PASS | 12 | 13 | slot drift; old lower hedged count favours KILL |
| FLAG | 3,898 | 3,901 | slot drift; old lower FLAG count favours project |
| PASS share | 16.87% | 16.88% | rounding/slot drift; old lower share favours KILL by 0.01pp |
| FLAG share | 83.13% | 83.12% | rounding/slot drift; old higher share favours KILL by 0.01pp |
| dust share of PASS | 98.5% | 98.4% | rounding/slot drift; old higher share favours KILL |

The committed script itself completed unchanged.  Its output at slots **440477062 / 440477066** was
`4690` live owners, `$64,746,386` total, `4349 / 336 / 5` by owner kind, and
`792 PASS / 779 dust / 13 hedged / 3898 FLAG` (`16.89% / 83.11% / 98.4%`).  It consequently does **not**
reproduce the document's live numbers at a later slot, which is expected slot drift.  Its invariant
registry counts and `6 / 0` intersection do reproduce, but the `2,915` uses the P0-invalid decoder.

## Decode and verdict-function checks

Independent command:

```sh
python3 - <<'PY'
import hashlib
print(hashlib.sha256(b'account:Position').digest()[:8].hex())
PY
# aabc8fe47a40f7d0 == bytes [170,188,143,228,122,64,247,208]
```

The full GPA at slot 440477379 returned 864,484 `dataSize:216` accounts with that discriminator.
Of those, 858,841 had `sizeUsd@161 == 0` and were excluded as closed; **zero** nonzero-size accounts
had a side byte outside `{1,2}`.  Thus the script's silent `continue` for invalid sides has no effect
at this snapshot (but diverges from the harness's fail-closed decoder and should be changed to error).
The current byte population therefore supports the document's `216 / owner@8 / side@152 /
sizeUsd@161` selection rather than showing a dropped live Position.

The Rust expression `(usd + signum(usd)*50) / 100` uses truncation toward zero.  It equals zero exactly
when `abs(usd) < 50`; PASS is therefore exactly `|net signed notional| < $50`.  I independently decoded
the committed raw fixture and then queried live:

```sh
cargo run -q -p probatio-svm-harness -- certify-jupiter --live \
  AhUvhrHHZXh7Huu8AtCfEkTvgUdTw4ZwL1j1c6Fu5dfq
```

At slot **440476464** it returned one open position, `net signed notional $16`, and `PASS`.  The fixture
bytes independently yield discriminator `aabc8fe47a40f7d0`, `owner@8` equal to that address,
`side@152=1`, `price@153=$143`, `sizeUsd@161=$16`, and `collateralUsd@169=$15`.  It is a below-band
rounding PASS, not a hedged/neutral position.

## Scope, status, and required correction

`grep -RInE 'fetch_owner_positions|JUP_PERPS_PROGRAM|certify-jupiter|--live' crates/harness/src`
finds the only live on-chain target ingestion at `fetch_owner_positions`: the owner-filtered Jupiter
Perps Position GPA path.  The other `certify-jupiter` inputs are sample or supplied traces, not a
second live target population.  I found no alternate live ingestion path that could make P1 proven.

**Explicit answer:** no.  I found **no address** that is both a demonstrated on-chain agent/agent vault
and has Jupiter Position state this harness can certify.  In particular, the corrected 1,767-key
registry identity set has zero live holders, the 335 missing holders are not PDAs, and none of the five
program-owned holders can presently be named as an agent vault from on-chain evidence.

`BLOCKED` remains wrong: no external dependency has a stated resolving action and date.  Once P0 is
fixed, `KILLED` remains the required P1 status under the gate's not-proven rule.

To change my conclusion after correction, provide either (1) a corrected-schema registry identity or
operational-wallet address with an open Jupiter Position, or (2) on-chain evidence that one of the five
listed program-owned position owners is an autonomous-agent vault, followed by a harness run against
that owner.  To change this review from `CHANGES` to `APPROVE`, correct the schema, regenerate every
affected count/table/status sentence, preserve the corrected `6 / 0` experiment, and make invalid
nonzero-side Position bytes fail closed in the reproduction script.

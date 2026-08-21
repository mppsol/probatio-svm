<!--
Independent review of the H4 G0 measurement. Requested by founder ruling 2026-08-22, one round only.
Reviewer: Codex, run read-only:
  /Applications/ChatGPT.app/Contents/Resources/codex exec \
    -C /Users/hiroyusai/src/probatio-svm -s read-only "<prompt>"
The prompt disclosed to the reviewer that it had itself authored crates/h4-sentinel (including the
uncommitted loader correction), forbade any rescue of the H4 hypothesis, and asked one question only:
is the MEASUREMENT wrong? The body below is Codex's output, verbatim and unedited.

CC's adjudication of this review, and the two gaps Codex could not close read-only, are in
docs/decisions/H4-sentinel-kill.md §"The review, and what it could not check".
-->

# H4 G0 Sentinel Review

**Verdict: MEASUREMENT SOUND.** I concur that H4 closes as **KILL-2 + KILL-3**. I do **not** find grounds for KILL-1.

## Findings

### P0 — None

I found no defect that could change a recorded case verdict. All four evidence cases mechanically satisfy the pre-registered `unknown` condition.

### P1 — Evidence does not expose the direct old-vs-new bytes/ranges

- `crates/h4-sentinel/src/main.rs:633-658`
- `evidence/h4-sentinel.json:92-770, 823-1371, 1424-1972, 2025-2703`

`post_state` records each post-state SHA-256 plus ranges relative to the fixture, not a direct old-versus-new byte diff. Thus the evidence proves that each old/new post-state differs (their account hashes differ), but it does not itself prove that offset 28 is the *only* old/new difference, nor reveal the exact new four bytes.

Fixing the report to emit direct old/new ranges and values would make the claimed characterization independently auditable. It would not change any verdict: each case already has unequal post-state SHA-256 values, hence remains `unknown`.

### P2 — Source provenance is attested, not independently reproducible within this review scope

- `fixtures/h4/MANIFEST.json:163-175, 201-228`
- `fixtures/h4/programs/klend_new.identity.json:1-17`

The checked-in data ties current fixture bytes to the claimed old/new hashes and records Solvo/mainnet provenance. It does not include the source Solvo Git object or raw RPC response needed to independently reproduce the copy/fetch chain. This review was expressly forbidden from accessing `../solvo` or the network, so I cannot independently establish that provenance.

The pre-measurement gate independently names the same old hash and expected new hash, and the current committed fixture files match both. No present mismatch exists. If the provenance records were false, the error direction is unconstrained: a substituted binary could produce any verdict, not specifically `unknown`.

## Independent checks

### 1. Binary identity

Command used:

```sh
for p in klend_old klend_new farms; do
  f="fixtures/h4/programs/$p.so.gz"
  gzip -dc "$f" | wc -c
  gzip -dc "$f" | shasum -a 256
  gzip -dc "$f" | perl -0777 -pe 's/\x00+\z//' | wc -c
  gzip -dc "$f" | perl -0777 -pe 's/\x00+\z//' | shasum -a 256
done
```

Results:

| Binary | Stored length | Stored SHA-256 | Trimmed `elf_len` | Trimmed/code SHA-256 |
|---|---:|---|---:|---|
| old klend | 10,485,715 | `a328a1a7…9de778` | 2,414,913 | `8eab9f85…3d1cda` |
| new klend | 10,485,715 | `9db16dd4…3cc85b` | 2,431,953 | `b1344d19…9c22d9` |
| farms | 10,485,715 | `5e42feec…5f564` | 835,745 | `9ca00de8…5a240` |

These match `MANIFEST.json` and the identity blocks at `evidence/h4-sentinel.json:6-18`.

The uncommitted loader correction is correct:

- `main.rs:735-773` loads the untrimmed stored payload and verifies both its stored hash and trimmed identity hash.
- `main.rs:778-785` rechecks the trimmed identity immediately before each VM load.
- `main.rs:437-440` passes the untrimmed bytes to LiteSVM.
- The recorded `elf_min_required_len` values are 2,414,928 old and 2,431,968 new, 15 bytes longer than their trimmed identities; loading trimmed bytes would be invalid.

The old code hash matches the hash fixed in the pre-measurement gate. The new identity record names ProgramData `9uSb…DQGA`, context slot `440628217`, deploy slot `440486775`, and upgrade authority `GzF…pzkW`; these values are internally consistent with the manifest and committed pre-run history.

I verified all 17 account JSON file hashes, decoded lengths, and decoded account-data hashes against `MANIFEST.json`.

### 2. Identical inputs and state

`main.rs:325-418` calls `setup` independently for every binary/case. `main.rs:421-504` creates a fresh `LiteSVM`, installs the same clock and all 17 fixture accounts, then applies exactly two fixture-related mutations:

1. Obligation owner, offset 64, length 32.
2. Newly created destination token account, offset 0, length 165.

Both mutations appear identically in every execution, for both binaries, in `evidence/h4-sentinel.json:20-39` and each case.

The same deterministic payer is used (`main.rs:787-790`), and instruction construction depends only on common keys and the pre-registered amount (`main.rs:545-600`). The only binary-specific input to setup/execution is `program.elf`.

The harness also creates a payer account and installs Clock/program/sysvar state as VM setup. Those are not cloned fixture accounts and therefore are not undisclosed mutations under gate §6.3. No harness write targets fixture offset 28.

### 3. Offset-28 divergence

Command used to read the fixture bytes:

```sh
for n in obligation reserve_sol reserve_usdc; do
  jq -r '.data_b64' "fixtures/h4/accounts/$n.json" |
    base64 -D | dd bs=1 skip=28 count=4 2>/dev/null | xxd -p
done
```

Result: each fixture has offset `28..31 = 00000000`.

Command used to compare the recorded fixture-relative ranges:

```sh
jq '.cases[] | {id,
  old: [.old.post_state[] | select(.account == "obligation" or .account == "reserve_sol" or .account == "reserve_usdc")],
  new: [.new.post_state[] | select(.account == "obligation" or .account == "reserve_sol" or .account == "reserve_usdc")]
}' evidence/h4-sentinel.json
```

For every case A, B, C, and E:

| Account | Old offset-28 range | New offset-28 range |
|---|---|---|
| `obligation` | absent | `(28, 4)` |
| `reserve_sol` | absent | `(28, 4)` |
| `reserve_usdc` | absent | `(28, 4)` |

The evidence establishes old bytes remain fixture bytes (`00000000`) at that range and that new bytes differ. It does **not** contain the exact new four-byte values; that is the P1 reporting limitation above.

This does not look like uninitialised memory or timestamp/slot nondeterminism: the completed JSON was reported byte-identical across three full runs, and the harness contains no program-label-dependent fixture write. The deterministic clock is set identically before every run (`main.rs:425-434`). The divergence is therefore attributable to execution with the loaded binary under test, not to a host-side mutation by the harness.

### 4. Verdict rule

`main.rs:650-660` implements gate §4 exactly:

- unexecuted either side → `unknown`;
- unequal result or token deltas → `breaking`;
- unequal post-state → `unknown`;
- otherwise → `compatible`.

Independent extraction from the evidence gives, for A/B/C/E respectively:

```text
old/new result equal=true
old/new token_deltas equal=true
old/new post_state equal=false
recorded verdict=unknown
```

No case should have been recorded `compatible` or `breaking` under the pre-registered rule.

### 5. Logs, compute units, return data

No assertion reads a log line. Logs, compute units, and return data are collected only into `Auxiliary` and printed:

- collection: `main.rs:507-542`
- storage: `main.rs:410-417`
- printing: `main.rs:921-932`
- verdict inputs: only `result`, `token_deltas`, and `post_state`, `main.rs:650-658`.

### 6. Determinism and offline operation

The harness uses local filesystem reads of fixtures and a local evidence write only. No source code path reads `../solvo`, invokes an RPC client, opens a network connection, or launches a subprocess.

The deterministic payer, fixture clock, fresh VM construction, fixed airdrop, and fixed inputs eliminate visible execution-time entropy. I independently verified the final evidence file is exactly:

```text
106,573 bytes
sha256 f05c0ea6055b80fa5100a47121be024924655af6ae8932924938c3a3d68f0cf0
```

I did not run `cargo run`, as directed, so I cannot independently reproduce the reported three-run determinism check. I found no code path that contradicts it.

### 7. Kill conditions

I agree with the founder’s reading:

- **KILL-1 does not fire.** Both binaries loaded and all eight executions have `executed: true`; the reported three completed artifacts are byte-identical. The earlier failed stripped-payload run was corrected before this measurement and does not describe the completed untrimmed-payload run.
- **KILL-2 fires.** A, B, C, and E are all `unknown`.
- **KILL-3 fires.** Any `unknown` yields `RE-VERIFY` under the pre-registered action table, which is no more actionable than the differing code hash.

### 8. Stubs, skips, and workarounds

No execution was stubbed or skipped in the completed evidence.

The only material correction was loading the untrimmed `programdata[45..]` payload while hashing its trailing-zero-trimmed identity. This is not a directional workaround: it is applied identically to old and new, and is necessary for either ELF to load. It does not bias toward `unknown`, `compatible`, or `breaking`.

The two disclosed state mutations are prescribed by the fixed top-level-signer experiment. They may make this a harness-owned invocation rather than the historical owner’s literal transaction, but that is fixed identically on both sides; its direction on the old/new equality comparison is neutral.

## Unverified read-only limits

I could not independently:

- rerun the harness or reproduce the three-run comparison;
- inspect `../solvo` to prove the old fixture was copied from its committed source;
- query mainnet or inspect a preserved raw RPC response to prove the new ProgramData fetch;
- recover exact new bytes at offset 28 from the evidence artifact.

None of these limits conflicts with the recorded hashes, committed fixture state, static harness behavior, or mechanical four-`unknown` verdict.

## Self-review disclosure

I wrote `crates/h4-sentinel`, including the currently uncommitted loader/hash correction in `src/main.rs`. Accordingly, I treated the harness source as untrusted implementation evidence and independently recalculated fixture lengths, stored hashes, trimmed hashes, manifest hash, fixture account hashes, evidence hash, and fixture offset-28 bytes. The P1 limitation is in code I wrote.

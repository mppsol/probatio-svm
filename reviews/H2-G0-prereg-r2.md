# H2 / G0 pre-registration r2 — independent review

**Verdict: CHANGES**

r2 removes some r1 ambiguity, but G0 remains unfreezable.  In particular, it
expressly defers the byte schemas/fields that define events and amounts, permits
an address-count inflation, and permits independent numbers on opposite sides of
a kill threshold to “reproduce.”  Those are doors through which an adverse G1
can become a pass.

## Independent chain checks

Read-only RPC commands used (finalized):

```
curl -sS -H 'Content-Type: application/json' -X POST https://api.mainnet-beta.solana.com \
  --data '{"jsonrpc":"2.0","id":2,"method":"getBlock","params":[440500000,{"transactionDetails":"none","rewards":false,"commitment":"finalized","maxSupportedTransactionVersion":0}]}'
curl -sS -H 'Content-Type: application/json' -X POST https://solana-rpc.publicnode.com \
  --data '{"jsonrpc":"2.0","id":1,"method":"getSignaturesForAddress","params":["<PROGRAM>",{"limit":1000,"commitment":"finalized"}]}'
```

`getBlock(440500000)` returned blockhash
`9WgLiyHZD8cS2JJ7LZ8GNDhmi4LV9jkGu11z7zb5Tpx4` and block time `1787240150`.
`getBlockTime(421060000)=1779309788`; the difference is `7,930,362` seconds,
as r2 states.  Snapshot slot after the density samples was **440561778**.

The asserted sub-window index is not reproducible from its stated expression.
For the UTF-8 bytes of the displayed base58 string:

```
sha256 = a4e2efdb6e9ddb0722ec05a5d32bf10955359bdc5fc4635126daffb9c454e0be
int(sha256, 16) mod 91 = 46
```

For the 32 bytes obtained by base58-decoding that same Solana blockhash:

```
sha256 = 7ec5c97ef6dda5155585f0e5ecb001b9ab468d5c617838f626945c5201b95213
int(sha256, 16) mod 91 = 3
```

Both are ordinary meanings of `sha256(blockhash)`.  r2 silently uses the first;
it never binds the encoding, so index 46 is not frozen.

Fresh 1,000-signature samples, using exactly `(1000 - 1) / (newestBlockTime -
oldestBlockTime)` and the second command above, were:

| venue | newest / oldest slots | sig/s | 90-day extrapolation |
|---|---:|---:|---:|
| Kamino Lend | 440561533 / 440556749 | 0.501254 | 3,897,754 |
| marginfi v2 | 440561558 / 440550082 | 0.209258 | 1,627,194 |
| Save | 440561183 / 440374409 | 0.012864 | 100,034 |
| Drift v2 | 440558182 / 440165668 | 0.006117 | 47,566 |
| Jupiter Perps | 440561569 / 440556648 | 0.487555 | 3,791,227 |
| Kamino Vaults | 440561558 / 440547701 | 0.173167 | 1,346,546 |
| total | — | — | 10,810,321 |

These are new trailing samples, not a refutation of r2's historical sample
rows.  They do show why the table is only a feasibility estimate: Drift is
currently 3.15 times r2's 0.00194 sample.  It cannot support any sharper claim.

## r1 findings 1–10

1. **P0 event unit — not closed.** “One venue-side state transition” is not a
   byte-level event definition: it names no instruction/discriminator, affected
   account, field, before/after value, or rule for a multi-instruction/partial
   liquidation.  r2 § “What pre-registration binds” expressly leaves “byte
   offsets, account discriminators, schema versions” to G1.  That contradicts
   H2-GATE G0's requirement to bind accounts, fields, and state transition in
   bytes.  Further, `N_addr` remains undefined as token/share-account address
   versus its controlling owner.  A holder can split a share balance between
   many token accounts (or PDAs) and increase distinct allocation addresses;
   that raises `N_addr` toward 1,000 without adding a bearer.  `C_10` over
   events closes only the allocation-splitting part of the r1 exploit.

2. **P0 L-B/L-C baseline — not closed.** “The venue's own recorded realised
   amount” still does not select a field.  A venue can record gross realised
   PnL, settled PnL, PnL net of fees/rebates, cumulative PnL, and/or a vault
   balance effect.  r2 binds neither a published schema field nor a sign,
   precision, event linkage, and aggregation rule.  Selecting the largest
   loss-like field after observing G1 increases `L_gross`/`L_total`; selecting
   another makes it `undetermined`.  The proposed rule therefore replaces the
   old baseline freedom with a recorded-field freedom.

3. **P0 USD hole — not closed.** `E_unknown` is an event count.  Excluding up
   to 5% of events from every dollar aggregate permits those events to carry an
   arbitrarily large fraction of dollars.  For example, 5 unknown events out of
   1,000 (0.5%, below the kill) could account for 99% of actual loss; the
   reported `L_und/L_gross` and `L_90` omit them.  This can inflate `L_90` and
   suppress the 30% undetermined test in the direction of a pass.  A count cap
   cannot bound an omitted dollar aggregate.

4. **P0 coverage — not closed.** “Designated by the program as an insurance
   fund, backstop or reserve” is a new undefined classifier: r2 gives no schema
   field/discriminator/value or authoritative program-account list establishing
   designation.  Nor does it define the dollar amount assigned to the stated
   “residual is undetermined”: `c_i` is defined, but no `u_i` nor rule connects
   a partial discretionary remedy to `L_und`.  Depending on whether `L_und` is
   the full loss or the residual after `c_i`, `L_cov + L_und` can double-count
   or not.  Finally, an outside-universe remedy can only be marked
   undetermined if it is found; the procedure does not bind a search that can
   discover external/purchased cover before asserting it was not found.  Each
   choice can raise the uncovered residual or lower the undetermined ratio.

5. **P0 enumeration — not closed.** The full-window proof is deferred to an
   as-yet unselected “chosen address or state query” and unpublished schemas;
   a one-day corroboration cannot establish a 92-day event recall, especially
   for rare large incidents.  The deterministic day itself is ambiguous for the
   two hash encodings above.  It is also internally incomplete: W lasts
   7,930,362 seconds, while 91 whole days contain 7,862,400 seconds, leaving
   **67,962 seconds (18 h 52 m 42 s)** outside the stated “91 whole 24-hour
   sub-windows.”  The purported slot mapping is not a mapping: “computed by
   wall-clock” supplies neither the first/last slot rule nor treatment of
   skipped slots.  Its displayed average-slot expression is not that mapping:
   `421060000 + 46*(19440000/91.7866) = 430802599` (rounded), but
   `getBlockTime(430802599)=1783195418`, whereas the wall-clock day begins at
   `1779309788 + 46*86400 = 1783284188`, a difference of **88,770 seconds**.
   Thus neither the selected 24 hours nor “every transaction touching” a program
   is an unambiguous, reproducible enumeration; the claimed dollar-recall check
   cannot close the concentrated-event omission door.

6. **P0 G3/G4 — not closed.** G3 still has no definition of an “exposure,” its
   positive label, owner mapping, categorical encoding of `venue id`, feature
   scaling/solver/intercept/class weighting, or which schema field establishes
   a liquidation boundary.  More basically, a `t0` in the final 30 days of W
   has no fully observed “next 30 days” outcome in W.  It may be censored,
   labelled no-loss, excluded, or measured outside W; r2 selects none.  Treating
   it as no-loss lowers test discrimination and can produce a pass.

   G4 retains a trivial form.  For L-A, `X` is the venue's recorded shortfall;
   for L-B/L-C it is the venue's recorded realised PnL—the same quantities r2
   uses to define the loss.  With `R=1`, `payout=loss` event-by-event, including
   out of sample, so the “parametric” arm has median and p90 basis error zero by
   construction.  `R`'s admissible domain and tie-break among the interval of
   median-error minimisers are also unstated.  The chronological split prevents
   one kind of hindsight fitting; it does not make an identity payout a test of
   payout/loss coincidence.  It is additionally vulnerable to a preselected
   temporal regime shift: no rolling or second independent split checks whether
   a low second-half AUC/basis error generalises.

7. **P0 request accounting / BLOCKED — not closed.** The time limit on a
   declared provider is an improvement, but the cache exception is unbounded.
   A collector can create an arbitrary RPC-derived cache before it begins the
   counted run, record its hash, then count every cache response as zero.  The
   document binds neither cache provenance/timestamp nor the requests used to
   produce it.  This defeats the 2,000,000 ceiling and lets a later report claim
   the narrow enumeration was feasible.  The provider/date clause does make a
   genuine declared `BLOCKED` lapse to KILLED; it does not close this uncounted
   data-access path.

8. **P1 density — closed as the r1 correction, with qualification.** The r2
   Drift arithmetic 0.00194 × 7,776,000 = 15,085 (approximately its 15,063,
   subject to unshown precision) is internally consistent, and command plus
   historical sample slots are now recorded.  My six samples above are the
   required independent re-measurement.  The material variation—most visibly
   Drift 0.006117 now versus 0.00194—means this remains only an
   order-of-magnitude feasibility attestation, exactly as r2 now says.

9. **P1 venue corpus — not closed.** The six included executable programs fit
   the written rule on their stated facts.  The AMM exclusion does not.  An
   arbitrageur's swap is another party's *instruction* that an LP neither owns
   nor authorises; adverse selection from that swap is not “price movement
   alone” under r2's own definition.  Calling it a price outcome therefore
   contradicts the rule used to include manager/trader outcomes.  Freezing the
   incomplete corpus after this contradictory application preserves an omission
   that can reduce pooled concentration.  Per-venue `C_10` cannot detect a
   concentrated event at an excluded venue, so it does not cure that asymmetry.

10. **P1 G5 tolerance — not closed.** It is computable but insufficient.
    Historical W is fixed, so two correct re-derivations need not drift across a
    threshold.  The allowance accepts exactly that result: reported `C_10=49.8%`
    and independent `50.2%` differ by 0.4 pp and “reproduce,” yet the latter is
    a G1 KILL.  Likewise $50.10M versus $49.85M differs by about 0.50%, and
    1,000 versus 995 addresses differs by 0.5%; each can flip G1 while passing
    G5.  There is no rule requiring the conservative value or identical gate
    outcome.  This directly lets a disappointing G1 be reported as a pass.

## New / remaining findings

### P0 — specifications required by G0 are deferred as “chain knowledge”

r2 lines 21–22 deliberately leave discriminators, schema versions, and byte
offsets unfrozen.  H2-GATE G0 requires the loss definition to name the accounts,
fields, and transition “in bytes.”  A later implementation can choose whichever
published version/field yields an event population and amount that passes.  This
is the root cause of findings 1, 2, 4, and 5, not a harmless implementation
detail.  **Direction: favours a pass.**

### P0 — G5 permits contrary verdicts

The numerical examples in finding 10 are not rounding trivia: all are directly
on a pre-registered kill boundary.  An initial computation just inside a pass
band can be certified despite an independent computation just inside KILL.
**Direction: favours a pass.**

### P0 — selected validation sub-window is neither uniquely derived nor a valid whole-window check

The hash encoding has two outputs (46 and 3), 18.9 hours of W are unassigned,
and the wall-clock-to-slot procedure is absent.  Even after an encoding choice,
the data-dependent selection of W's end occurred after the seed block existed,
so the assertion “nobody can choose it” is false without a prior commitment to
W before that blockhash was known.  **Direction: favours a pass by permitting a
more convenient validation day and omission of rare large events.**

### P1 — changelog direction claim is false

“Every r2 change moves the gate against the project” is not true.  The density
correction is explicitly labelled “immaterial to any verdict,” hence neutral,
not against.  The G5 tolerance is favourable in threshold-adjacent cases above.
The deterministic-day mechanism is also not demonstrably against the project
when its seed was known before the r2 commitment.  These are precisely the
directional accounting errors H2-GATE says must disqualify a claimed pass.

## Conclusion

**Is any door still open through which a disappointing G1 could become a pass?
Yes.** Unfrozen byte fields/event identity, token-account `N_addr` splitting,
count-only unknown-USD exclusion, coverage designation/residual choices,
ambiguous and one-day enumeration, G3 censoring, the identity G4 payout, the
uncounted-cache route, omitted AMM venues, and a G5 tolerance that accepts
opposite-side results each supply such a door.

My verdict would change only with a new pre-registration that binds the actual
published schemas/accounts/fields and all owner/event/coverage amount rules;
uses a uniquely encoded, fully mapped validation procedure that does not leave
W uncovered; makes G3 labels/censoring and G4 non-identity payout procedures
computable; accounts for cache provenance; applies the corpus rule consistently;
and makes an independent result on the KILL side terminal rather than tolerable.
No G1 implementation or measurement should begin before that review is
approved.

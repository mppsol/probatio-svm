# H2 / G0 pre-registration — independent review

**Verdict: CHANGES**

G0 does freeze a slot window, six program IDs, and numerical cut-offs.  It does
not yet freeze the units, classifiers, and statistical procedures that produce
those numbers.  Therefore G1 could still convert an adverse measurement into a
passing one by choosing a favourable event representation, coverage treatment,
enumerator, participation time, predictor, or payout function after seeing the
data.

## P0 — the loss event is not specified in bytes or in a stable unit

`docs/H2-preregistration.md` calls a loss event “a state transition ... that
reduces the recoverable value held by an identifiable address,” then gives only
semantic rows for L-A through L-D.  It does not identify a program instruction
or account transition, discriminator/schema version, account(s), field(s),
before/after state, share-balance snapshot, allocation rule, or the unit of an
event.

This is not the byte-level definition required by H2-GATE G0.  In particular,
one reserve bad-debt transition borne pro rata by 500 depositors can later be
reported either as one event or 500 address allocations.  That choice changes
`N_events`, `N_addr`, and `C_10`: splitting it raises the first two and lowers
concentration, which can manufacture a G1 pass; merging does the reverse.
Likewise, L-B and L-C have no fixed baseline for “reducing”/“falling,” nor a
rule separating a manager/trader-caused decrease from oracle movement, fees,
deposits, withdrawals, or a later holder balance change.  Selecting the
baseline after observing the series can include profitable-looking drawdowns
and raise `L_total`, or exclude them and lower it.

The stated USD fallback is also not computable in all cases: item 3 says an
event whose venue USD field *and* oracle are unavailable is “undetermined and
counts toward `L_und`,” while `L_und` and `L_und/L_gross` are USD quantities.
No dollar amount is defined for that event.  Omitting it lowers the
undetermined ratio (favouring a pass); assigning a later observed value is
another post-hoc degree of freedom.

## P0 — coverage is a binary label for a non-binary quantity, with no search universe

The exact exploitable text is: “made the bearer whole in whole or in part” is
classified `covered`, and `uncovered` means “no such mechanism is found.”
`L_cov` is then subtracted in dollars from `L_gross`.

There is no rule for the amount of a partial remedy, the loss portion it offsets,
when a remedy is “contractually available” but unclaimed/unclaimable, or the
complete set of on-chain accounts/programs that must be searched before “no
such mechanism is found” may be asserted.  A later analyst can label a partial
payment as fully covered (lowering `L_total`, favouring KILL) or subtract only
the paid portion / call an unlocated remedy absent (raising `L_total` and
lowering `L_und`, favouring a pass).  The last option is especially material:
the 30% kill guard can be passed merely by stopping the coverage search before
an ambiguous remedy is found.

## P0 — “enumeration address” is not merely chain knowledge under this validation

The document permits choosing at G1 “the narrowest address or state query that
provably contains all events,” but does not freeze what constitutes a proof,
the full-enumeration procedure, the PRNG/seed/time at which the “randomly
chosen” 24-hour sub-window is selected, or a dollar-weighted recall check.

The 0.98 threshold is event-count recall only.  An enumerator can miss 2% of
events, including the ten largest events, while clearing 0.98 recall on a
conveniently selected ordinary day.  If the remaining loss still clears $50M,
removing those large events lowers `C_10` and can turn a concentration KILL into
a pass.  A different narrow address can also change which account transitions
are visible, feeding the undefined event-unit choice above.  Thus the address
is a substantive sampling decision until the event schema and a deterministic,
whole-window completeness test are bound.

## P0 — G3 and G4 have no computable, pre-registered inputs

G3 defines `AUC` as the result of “the simplest honest predictor” of loss in
the next **N** days, but fixes neither N, t0 for each exposure, features,
training/test split, estimator/hyperparameters, missing-data treatment, nor
the definition of “already determined” for `D`.  A lower-capacity predictor or
an earlier t0 selected after examining outcomes lowers AUC/D and favours a
pass; a richer predictor or later t0 moves in the other direction.  “Simplest
honest” is an assessment, not a reproducible experiment.

G4 supplies names—parametric, indemnity, and staked/slashed bond—but no payout
functions, trigger inputs and their observation times, bond amount, or rules
for funding/capping/eligibility.  An indemnity function set after observing the
same event loss has `payout = loss`, hence median and p90 basis error of 0; this
would manufacture a G4 pass.  Conversely arbitrary caps can manufacture a
KILL.  The required `median(e)` and `p90(e)` therefore are undefined.

## P0 — the 2,000,000-request ceiling is both undefined and an escape hatch

“RPC budget ceiling (frozen): 2,000,000 requests” never defines whether a JSON
RPC batch, `getMultipleAccounts` batch, retry, cached response, failed request,
or indexed-provider query counts as one request.  Consequently feasibility
cannot be re-derived from the stated ceiling.

More importantly, exceeding this self-imposed ceiling returns `BLOCKED — data
access`, only “naming the resolving action ... and a date.”  It mandates no
provider, no deadline consequence, and no eventual G1 verdict.  It lets H2
avoid the binding H2-GATE result “not-proven is a KILL” without showing that
public historical data is unavailable.  This is an escape hatch, not an honest
data-access verdict.  The attested full-program estimates already imply roughly
12.1M unfiltered transaction fetches for the four listed rows, so whether a
sub-2M method exists is a deciding feasibility fact, not something that may be
deferred indefinitely.

## P1 — the feasibility attestation contains a numerical contradiction

The Drift row says both “~0.03” tx/s and “~15,000” transactions/90d.  These
cannot both be true:

```
0.03 * (90 * 86400) = 233,280
15,000 / (90 * 86400) = 0.001929012346 tx/s
```

The claimed count corresponds to about 0.00193 tx/s, not 0.03 tx/s.  The table
also records neither its sample slots nor an executable command, so its numeric
attestation cannot be independently reproduced as written.  This does not by
itself decide H2, but it invalidates G0's stated feasibility evidence and must
not remain as a frozen factual premise.

## P1 — the venue rule is not independently applicable

“Every Solana venue” is asserted without a frozen candidate corpus, discovery
source, or operational definitions of *venue*, *live*, *loss*, and “produced
by another party's ... decisions.”  The six IDs are a fixed list, but a reviewer
cannot test the assertion that the list is the rule applied, or determine
whether an AMM LP adverse-selection loss, a lending market outside the named
programs, or a perps liquidity facility belongs in scope.  This permits
post-result rhetoric that an omitted class was never a venue/loss, or that a
listed program's inconvenient class is outside its row.  Depending on the
omitted class, this can change `L_total`, `N_addr`, `C_10`, and `L_und` in
either direction.  I do not name a particular omitted venue as required on the
available RPC evidence: program existence alone cannot establish the undefined
causal criterion.  That is the defect.

## P1 — G5's tolerance is absent

G5 kills a number that fails re-derivation “beyond stated slot drift,” but no
slot-drift magnitude, comparison method, rounding convention, or treatment of
historical RPC inconsistency is stated anywhere in G0 or H2-GATE.  A later
reviewer can accept a discrepancy by calling it drift, which can preserve a
desired pass.  This is a threshold whose input is undefined.

## Independently verified chain facts

All calls below used only `https://api.mainnet-beta.solana.com` with finalized
commitment.  Initial snapshot: `getSlot = 440,559,318`; account responses had
context slot `440,559,320`.  Command form:

```
curl -s -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","id":1,"method":"METHOD","params":[...]}' \
  https://api.mainnet-beta.solana.com
```

* `getBlockTime(421060000) = 1779309788` and
  `getBlockTime(440500000) = 1787240150`.  Their difference is **7,930,362 s**
  = **91.786597222 days**.  `(90*86400)/7930362 = 0.980535314781`; the stated
  0.9805 is a correct rounding.  `getFirstAvailableBlock = 0`.
* `getAccountInfo` returned `executable: true` for all six V IDs:
  `KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD`,
  `MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA`,
  `So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo`,
  `dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH`,
  `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu`, and
  `KvauGMspG5k6rtzrqqn7WNn3oZdyKqLKwK2XWQ8FLjd`.
  The same request returned `value: null` for
  `vAuLTQTvpZ8AiMbBLKN1QUbfsHhkPWFPPeNfnRLYqrE`.
* Fresh `getSignaturesForAddress(address,{limit:1000,commitment:"finalized"})`
  samples, using `(1000-1)/(newestBlockTime-oldestBlockTime)`, produced:

  | program | newest/oldest sample slots | observed signatures/s |
  |---|---|---:|
  | Kamino Lend | 440559417 / 440555756 | 0.65724 |
  | marginfi v2 | 440559392 / 440548818 | 0.22736 |
  | Save | 440558533 / 440373912 | 0.01302 |
  | Drift v2 | 440558182 / 439317643 | 0.00194 |
  | Jupiter Perps | 440559409 / 440555359 | 0.59358 |
  | Kamino Vaults | 440559414 / 440545291 | 0.17010 |

  These are fresh 1,000-signature samples, not a full-window census.  They
  broadly support the Kamino/marginfi/Jupiter order of magnitude and independently
  support Drift near 0.00193 rather than the document's 0.03/15,000 mismatch.

## Attestation and conclusion

I found no direct numerical loss magnitude, decoded loss event, or amount in
G0.  I therefore cannot establish from the document that W or V was selected
after seeing a loss magnitude.  The negative attestation itself is not
independently auditable, and the missing candidate corpus makes the claimed
application of the venue rule untestable.

**Is there a door left open through which a disappointing G1 could be turned
into a pass? Yes.**  P0s above provide several: event splitting/baselines,
partial or unsearched coverage, an enumerator that omits concentrated events,
an intentionally weak G3 predictor/early t0, and a hindsight indemnity payout.

My verdict would change only after a new pre-registration commit closes every
P0 by binding the relevant data schemas, event/allocation and USD units,
coverage amounts and search universe, deterministic enumeration validation,
G3 protocol, G4 payout functions, request accounting and terminal data-access
rule; corrects the density attestation; and supplies a numeric G5 comparison
tolerance.  That must occur before any G1 measurement or implementation.

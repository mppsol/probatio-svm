# H5 G0 — candidate search for a sequence-only failure — **design investigation, no measurement**

> ⛔ **BRANCH A IS WITHDRAWN. H5 IS NOT KILLED. No kill commit was made.**
> The independent review this document asked for — [`reviews/H5-G0-candidates.md`](../reviews/H5-G0-candidates.md),
> Codex, read-only, run once by founder ruling — returned **`DISSENT — a candidate survives: C3`**,
> with **two P0s against this document's own reasoning**. Per the founder's instruction, a dissent
> naming a viable surviving candidate means **report, do not kill**. §6 and §7 below are therefore
> **superseded by §8**, which records what the review overturned and what it did not.
>
> **Nothing below is rewritten.** It is kept as the argument the review is read against.


**Date:** 2026-08-22 · **Founder ruling:** continue H5 as a *design investigation only*; H5 itself is
**not** killed; the first candidate (klend `withdraw(u64::MAX)` balance mismatch) is **finished as
`KILL-1`** and **may not be repaired, rescued or re-measured**; no post-hoc change of criteria.

**Nothing here was executed.** No harness was run, no transaction simulated, no fetch made, no code
written. Every number below is read either from committed fixture bytes or from H4's frozen
`evidence/h4-sentinel.json`. The commands used to read them are given so each is reproducible.

Gate: [`docs/H5-GATE.md`](./H5-GATE.md) · killed first candidate: §8.1 (`REJECTED`) ·
its review: [`reviews/H5-G0-agent-release-tests.md`](../reviews/H5-G0-agent-release-tests.md).

---

## 1. The bar a candidate must clear

Fixed by the founder, before the search:

| # | necessary condition |
|---|---|
| 1 | depends on the **order of ≥ 3 transactions** on real BPF + cloned mainnet state |
| 2 | **cannot be determined** by simulating each single transaction and reading the post-state it needs |
| 3 | a **fixed scripted agent's next action changes** because of the multi-transaction state diff |
| 4 | has a **concrete CI entrypoint** attachable to a developer's release process |
| 5 | does **not** extend to a protocol-independent schema, an arbitrary adapter, or a general runtime wallet policy |
| 6 | may use existing Probatio/Solvo fixture, LiteSVM and BPF-clone assets, but **may not reuse a past hypothesis's conclusion as demand evidence** |

**Standing rule, applied at design time:** if a candidate's failure is **fully observable in one
transaction's post-state**, it is marked `KILL-1` **here**, and is neither implemented nor measured.

## 2. What the committed fixture can actually express

This bounds the search, so it is established first — from bytes, not from memory.

**Corrected 2026-08-22 after review P2** — the snippet first published here defined helpers and
printed nothing, so it did not reproduce the table below. Save this as `read_obligation.py` and run
`python3 read_obligation.py` from the repo root; it does:

```python
import json, base64
ob = base64.b64decode(json.load(open('fixtures/h4/accounts/obligation.json'))['data_b64'])
u64  = lambda o: int.from_bytes(ob[o:o+8],  'little')
u128 = lambda o: int.from_bytes(ob[o:o+16], 'little')
B58 = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz'
def b58(b):
    n = int.from_bytes(b, 'big'); s = ''
    while n:
        n, r = divmod(n, 58); s = B58[r] + s
    return '1' * (len(b) - len(b.lstrip(b'\x00'))) + s

for i in range(8):                      # deposits: base 96, 8 slots of 136 B
    o = 96 + 136 * i
    if any(ob[o:o+136]):
        print('deposit', i, b58(ob[o:o+32]), u64(o + 32))
for i in range(5):                      # borrows: base 1208, 5 slots of 200 B
    o = 1208 + 200 * i
    if any(ob[o:o+200]):
        print('borrow ', i, b58(ob[o:o+32]))
for name, o in (('deposited_value', 1192), ('bf_adjusted_debt', 2208),
                ('allowed_borrow_value', 2240), ('unhealthy_borrow_value', 2256)):
    print(f'{name:24} {u128(o) / (1 << 60):.6f}')
```

| field | value |
|---|---|
| `obligation.deposits` | **exactly one** — reserve `D6q6wuQSrifJKZYpR1M8R4YawnLDtDsMmWM1NbBmgJ59` (USDC), `deposited_amount` **2,248,785,777** |
| `obligation.borrows` | **exactly one** — reserve `d4A2prbA2whesmvHaL88BH6Ewn5N4bTSU2Ze8P6Bc4Q` (SOL) |
| `deposited_value_sf` @1192 | ≈ **$2,690.34** |
| `borrow_factor_adjusted_debt_value_sf` @2208 | ≈ **$1,343.59** |
| `allowed_borrow_value_sf` @2240 | ≈ **$2,152.28** |
| `unhealthy_borrow_value_sf` @2256 | ≈ **$2,421.31** |

**Unit caveat, added after review Q9:** the `$` labels are an inference. The bytes establish the
`2^60` fixed-point *magnitudes*; that the unit is USD is corroborated by H4's frozen token deltas and
by USDC's 6 decimals, **not** by the obligation bytes alone. Nothing below depends on the unit — only
on the ratios, which are unitless.

**The `_sf` fields are fixed-point at `2^60`, and the layout is confirmed by four independent
consistency checks landing on exact round numbers** — this is the proof the derivation above is not
a misread, and it is given because a wrong layout would invalidate the whole investigation:

| check | value | what it means |
|---|---|---|
| `allowed_borrow_value / deposited_value` | **0.800000** | an exact **80% LTV** |
| `unhealthy_borrow_value / deposited_value` | **0.900000** | an exact **90% liquidation threshold** |
| `borrow_factor_adjusted_debt / borrowed_assets_market_value` | **1.250000** | an exact **125% borrow factor** |
| 2,248,785,777 atoms × H4's *measured* case-A rate (119,647,109 / 100,000,000) | **$2,690.61** vs stored **$2,690.34** | agree to **0.01%** |

Three exact round constants and an independent cross-check from a different source (H4's measured
token deltas) do not co-occur under a misread layout.

**The position is one collateral against one borrow.** Two consequences shape this investigation:

- **There is no second collateral leg**, so the strongest sequence-only shape — *"withdrawing
  collateral A first makes collateral B unwithdrawable, while the reverse order exits cleanly"* —
  **cannot be built on this fixture at all.** It needs an obligation with ≥ 2 deposits.
- **klend clamps a withdrawal to whatever keeps the position solvent**, which H4 measured: case E
  requested `u64::MAX` and moved 486,657,686 of 2,248,785,777. **One transaction already walks the
  position to klend's own limit**, which makes the second sequence-only shape — *"each step is
  individually legal but the cumulative walk ends at the liquidation edge"* — look unavailable,
  because one step appears to end there already.

  **This second bound is stated with less confidence than the first, and deliberately so.** That a
  sequence of *smaller* withdrawals cannot reach a state a single maximal withdrawal cannot is
  **plausible but not established read-only** — klend recomputes the same solvency constraint on each
  call, so intermediate steps should not open room a maximal call does not, but proving it would take
  a run, and no run is authorised. It is **not load-bearing**: `C1`–`C3` below die on other grounds
  (observability and fabrication), not on this bound. **It is flagged for the reviewer**, and if it is
  wrong, the "cumulative walk" shape returns and this investigation was not wide enough.

Everything below is searched inside those bounds.

---

## 3. Candidate C1 — the failing transaction's bytes are **derived from earlier results**

**Idea.** Make the last transaction impossible to construct in advance: the agent computes its amount
from what it *observed* in an earlier transaction, so no single simulation can even build it.

**Fixed sequence (4 transactions).** `T1` refresh preamble (`refresh_reserve`×2, `refresh_obligation`)
· `T2` `withdraw(A)` for a fixed `A` · `T3` refresh preamble again · `T4` `withdraw(R)` where
`R = agent's recorded remaining`, computed from `T2`'s observed result.

**Failure predicate.** At end of episode `obligation.deposits[USDC].deposited_amount ≠ 0` while the
agent terminated in state `EXITED`, evaluated on state bytes only.

**Why one simulation cannot construct it.** `T4`'s instruction data is a function of `T2`'s
execution. `simulateTransaction` takes a *transaction* as input; `T4` does not exist until `T1`–`T3`
have run. This is a genuine structural barrier and it is why C1 was carried this far.

**Why it dies anyway — `KILL-1`, marked here, not measured.** The barrier protects the *construction*
of `T4`, not the *observation* of the failure. Once `T4` exists, its own post-state contains the
entire predicate: `deposited_amount ≠ 0`, with `EXITED` following deterministically from the agent's
own rule. **This is the identical shape the first candidate died of** — the failure is witnessed
inside one transaction's post-state, and the reviewer's P0 against §8.1 applies verbatim:

> The intent is the scripted agent source, available to the same developer invoking simulation. A
> simulator-side regression assertion can simply state: "after `withdraw(…)`, `deposited_amount == 0`".

Carrying C1 forward would be repairing the killed candidate under a new name, which the ruling
forbids. **C1 is `KILL-1` at the design gate.**

*(Remaining items are recorded for completeness only; C1 is not to be built.)* Static policy: a
destination allowlist and a per-transaction cap miss it, but that is irrelevant once `KILL-1` fires.
Agent rule: `on Ok → believed_remaining := 0`. Assets: available — the H4 fixture set, both mutations
(obligation owner @64 len 32; deterministic destination token account), klend + farms BPF. CI: same
placeholder contract as §8.1. **`KILL-1`.**

---

## 4. Candidate C2 — a plan computed at `T1` is invalidated by **slot advance and accrued interest**

**Idea.** Time-of-check / time-of-use. The agent's logic is correct *at `T1`*; the chain moves under
it. This is a materially different bug from C1 — nothing the agent computes is wrong when computed.

**Fixed sequence (4 transactions).** `T1` refresh preamble at slot `S`, agent reads the position and
computes a plan · `T2` `withdraw(A₁)` at slot `S` · `T3` refresh preamble at slot `S + Δ`, at which
klend accrues interest on the SOL borrow, so `borrow_factor_adjusted_debt_value_sf` **rises** and the
withdrawable margin **falls** · `T4` `withdraw(A₂)` with `A₂` from the plan computed at `T1`.

**Failure predicate.** `A₂` is no longer admissible: either `T4` reverts, or klend clamps it, so the
realized delta at the USDC collateral supply vault is less than `A₂`, while the agent's recorded
ledger says `A₂` moved. Recorded with its direction, both ways.

**Why it does not clear condition 2 — `KILL-1`, marked here.** `A₂` is fixed at `T1`, so `T4` **is
constructible in advance**. At the moment `T4` would be submitted the chain is already in the
post-`T3` state, and **one `simulateTransaction` of `T4` at that moment determines the failure
exactly** — it returns the revert, or returns the clamped post-state. A production agent that
simulates before submitting simply does not submit. The pre-release framing does not rescue this:
condition 2 asks whether *a single simulation plus the post-state it needs* can determine the
failure, and here it plainly can. **`KILL-1`.**

**Secondary, and recorded because it would have mattered:** the accrual driver is real and needs no
fabrication — klend accrues on `refresh_reserve` from the slot delta, and LiteSVM can advance slots —
so C2 would have satisfied conditions 1, 3, 5 and 6. It fails on 2, which is the one that matters.

---

## 5. Candidate C3 — a **wrong retry after an apparent revert** double-executes

**Idea.** The founder named this shape directly. Each transaction is individually correct; the defect
is that two of them happen. A sum-over-the-sequence failure is the one shape a per-transaction check
structurally cannot express.

**Fixed sequence (≥ 4 transactions).** `T1` refresh · `T2` `withdraw(A)` · `T3` refresh · `T4`
`withdraw(A)` again, because the agent's retry rule believes `T2` failed.

**Failure predicate.** Cumulative: the USDC collateral supply vault falls by `2A` across the episode
while the agent's ledger records `A`. No single transaction violates anything.

**Why it dies — `KILL-2`, marked here.** The retry must be triggered by the agent believing `T2`
failed **when it did not**. On Solana a transaction is atomic: it either lands or it does not. The
only way `T2` lands while the agent sees a failure is a **client-side event** — an RPC timeout, a
dropped response, a confirmation that never arrives. **That event is not in the chain and cannot be
reproduced by real BPF on cloned state**; it can only be injected by the harness declaring it. A
failure the harness fabricates is not a failure the instrument found, and disclosing the injection
does not make it a chain fact.

The alternative trigger — have `T2` genuinely revert, then retry — produces no double execution,
because a reverted transaction moved nothing. The remaining alternative is to trigger the retry from
klend's **clamping**, which is the mechanism the killed first candidate rested on; using it here
would be reviving that candidate under a new name.

**`KILL-2`** — a stateful failure cannot be reproduced on real BPF without fabricating a
non-chain event.

---

## 6. Result of the search

| candidate | mechanism | verdict at the design gate |
|---|---|---|
| **C1** | failing transaction's bytes derived from earlier results | **`KILL-1`** — the failure is still fully witnessed in `T4`'s own post-state; identical shape to the killed §8.1 candidate |
| **C2** | plan invalidated by slot advance and accrued interest | **`KILL-1`** — `T4` is constructible in advance, and one simulation of it at submission time determines the failure |
| **C3** | wrong retry after an apparent revert, double execution | **`KILL-2`** — the trigger is a client-side event that real BPF on cloned state cannot produce; only harness fabrication supplies it |

**All three fail. None is implemented or measured**, per the standing rule.

### The single reason they fail, stated once

The two shapes that would have cleared condition 2 are **ruled out by the fixture itself** (§2): there
is **no second collateral leg** to make ordering matter, and **klend's own clamping reaches the
solvency limit in one transaction**, so a cumulative walk has nowhere further to go. What remains on
this fixture is a **single capital action**, and a single capital action's failure is, by
construction, witnessed inside a single transaction's post-state.

**One bound is flagged rather than proven** (§2, the cumulative-walk bound). It is not load-bearing —
`C1`/`C2` die on observability and `C3` on fabrication — but if a reviewer overturns it, the
"cumulative walk to the liquidation edge" shape returns and this search was not wide enough. That is
recorded here rather than buried, because it is the one place branch A could be premature.

**This is a fact about the available fixture, not a proof about Solana agents.** It is stated that way
deliberately: H5's hypothesis may be true and simply not testable with the assets G0 was given. Under
this repo's rules that distinction does not save it — **not-proven is a KILL** — but the distinction
belongs in the record, because the resume condition follows from it and not from a hunch.

---

## 7. Branch A — the proposal, for a founder ruling only

Per the ruling, all candidates falling to `KILL-1`/`KILL-2` selects **branch A**.

**Proposed: H5 is `KILLED`.** *Not-proven is a KILL. H5 could not name a failure that requires the
sequence, on the assets G0 was given.*

**This file does not close H5, and no kill commit is made without a founder ruling.** What is recorded
here is the basis, the excluded candidates, and the resume condition — nothing more.

### What would have to be true to reopen this, stated so it cannot be loosened later

Reopening requires a **new founder ruling and a new G0**, and the new G0 must satisfy **both**, in
this order:

1. **A fixture that can express a sequence.** An obligation with **≥ 2 independent capital legs**
   (two collaterals, or a collateral plus a repayable debt the agent actually holds the means to
   repay), or a workflow with a genuine **partial fill**. The committed `fixtures/h4/` set has
   neither. Acquiring one is a **new fetch**, and it is not authorised by anything in force today.
2. **The non-observability of the candidate demonstrated *before* measurement.** The new G0 must show,
   as an argument on the fixed sequence, that the candidate failure is **not fully witnessed in any
   single transaction's post-state** — and it must name which transaction would witness it if the
   argument is wrong. **Three candidates in a row have died to exactly this check applied late.**
   Applying it first is the only change that would make a fourth attempt worth its cost.

**Neither condition is satisfied today, and neither may be waived by an agent.**

### What is explicitly *not* claimed

- **Not claimed:** that no sequence-only failure exists on Solana. Only that none could be constructed
  on the committed fixture, and that the two shapes that could have been are excluded by §2.
- **Not claimed:** that H4's or H1's material supports H5's demand. No past hypothesis's conclusion is
  used as evidence here; H4's evidence is used **only** as a measured fact about klend's clamping.
- **Not proposed:** any replacement candidate. §8.1 pre-registered that a substitute requires a new
  founder ruling and a new G0 row. That binds CC, and CC has proposed none.

---

## 8. What the review overturned — **this section governs**

One round, read-only, from the payload committed at
[`reviews/H5-G0-candidates.codex-prompt.md`](../reviews/H5-G0-candidates.codex-prompt.md).
Verdict **`DISSENT — a candidate survives: C3`**. Full text:
[`reviews/H5-G0-candidates.md`](../reviews/H5-G0-candidates.md).

### The revised standing of each candidate

| candidate | §3–§5 said | review says | stands as |
|---|---|---|---|
| **C1** | `KILL-1` | **P0 — unsupported.** Dismissing it by inspecting `T4`'s post-state *"silently grants the baseline a state-carrying local executor. That is H5's distinguishing machinery, not one RPC `simulateTransaction`."* | **not killed**; also **not frozen** — the review notes `R` and the exit rule are still unspecified |
| **C2** | `KILL-1` | **assumes the disputed capability.** A production agent can simulate `T4` against real post-`T3` state; **pre-release CI cannot produce that state through RPC simulation alone**. *"The gate must choose explicitly."* | **undecided**, pending the baseline definition |
| **C3** | `KILL-2` | **P0 — wrong premise.** *"A dropped confirmation is not a fabricated chain fact; it is an explicit client/transport fault input to the agent under test."* Both withdrawals remain real BPF on cloned state. | **survives** |

**CC concurs with both P0s.** They are not close calls:

- **C3.** `KILL-2` as pre-registered fires when *"a stateful failure cannot be reproduced on real
  BPF"*. The stateful failure in C3 is the **cumulative double withdrawal**, and that **is** reproduced
  on real BPF. CC's §5 argument conflated the *trigger* with the *failure*. **The system under test is
  the agent, not the chain**, and injecting a transport fault into the agent is ordinary fault
  injection, not fabricating chain state. The kill was wrong.
- **C1.** CC's §3 argued the construction barrier "protects the construction of `T4`, not the
  observation of the failure" — but observing `T4`'s post-state **requires having executed `T1`–`T3`**,
  which is precisely the episode executor H5 proposes. CC granted the baseline H5's own machinery and
  then found H5 redundant. The kill was circular.

### The one thing that must be ruled on before anything else

Both P0s, `C2`'s undecided status, and the `KILL-1` that closed §8.1 all reduce to **one unresolved
definition**, which the review states directly (Q6, Q10):

> `simulateTransaction` does not commit or advance mainnet state. A baseline that persists post-state
> across simulations **is a local-fork episode executor — the machinery H5 proposes.**

**So the baseline must be defined operationally, and it decides H5 either way:**

| if the baseline is… | consequence |
|---|---|
| **one non-persistent RPC simulation** (cannot advance chain state) | multi-transaction failures are genuinely outside its reach; `C1` and `C3` are live and H5 has a real claim |
| **allowed to carry cloned post-state across transactions** | it *is* H5, no candidate can ever clear condition 2, and **H5 is dead in general, not per-candidate** |

**This is a founder ruling, not an agent's call**, and it must be made **before** any freeze,
measurement or kill. Choosing it after seeing a result would be the post-hoc criterion change the
ruling forbids.

### Disclosed, and explicitly *not* acted on

The review's Q6 states that the §8.1 review's grant of a *"post-`T1` state"* **was not a valid
bare-single-simulation baseline** — i.e. the same argument now cuts against the `KILL-1` that closed
the **first** candidate. **CC is not reopening §8.1**: the founder ruled it finished and forbade
repairing or rescuing it, and that ruling stands regardless of this review. It is recorded here
because the founder should know the reasoning has been undermined, **not** as a proposal to revive it.

### What is still true from §2

The review **independently re-derived** the fixture bound and confirmed it: exactly one live deposit
slot (`D6q6…gJ59`, 2,248,785,777) and exactly one live borrow slot (`d4A2…Bc4Q`), and the four `_sf`
values. So **the two-collateral ordering shape remains unavailable** — but the review notes this
*"excludes a two-collateral ordering case, but not C3's duplicate-execution case."*

The flagged cumulative-walk bound (§2, §6) is confirmed as **not established** (review P1), and the
review adds that *"the claim that case E reaches the protocol's ultimate solvency boundary is not
supported by committed post-state bytes."* **§7's resume condition is therefore too narrow**: the
review states **C3 needs neither a second capital leg nor a partial fill.** §7 does not govern.

### Status

**H5 is not killed and not frozen.** No replacement candidate was searched for, no fetch was made, no
code written, no measurement run. Awaiting a founder ruling on the baseline definition above.

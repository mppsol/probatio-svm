# Kill Gate — Probatio SVM

> **⛔ THIS GATE IS CLOSED. H1 は KILLED。**
>
> This is **H1's** gate. Its verdict is final and is not to be rewritten:
> [`docs/decisions/P1-real-target.md`](./decisions/P1-real-target.md) (P1 `KILLED`, Codex r2 `APPROVE`).
> The document below is kept verbatim as the record of what was gated and how.
>
> **The binding gate is now [`docs/H2-GATE.md`](./H2-GATE.md).** Do not produce evidence for the items
> below, and do not reuse H1 artifacts as a foundation for H2 — that is 延命, not scope.

The only work permitted in this phase.

> **このプロジェクトは探索無限化を禁止する。**
>
> 最初のフェーズは「Kill Gate の証拠を作ること」のみであり、
> Gate を通過するまで、一般化、大規模UI、周辺機能、本番展開を行わない。
>
> Gate の各判定項目について:
> - 数値または再現可能な実験で結論を出す
> - 結論が NO、未証明、または重大な OPEN_RISK なら、STATUS を `KILLED` または `BLOCKED` に更新して停止する
> - 仮説を追加して延命しない
> - テスト、証拠、レビューが揃った場合のみ `GO` とし、次フェーズの最小タスクを一件だけ開始する
>
> commit は完結した証拠またはタスク単位で行う。
> push は対象差分のみ・検証成功後のみ行う。
> **実資金、mainnet deploy、force push、秘密情報の追加は禁止する。**

## Why this exists

Without a kill condition fixed *in advance*, every agent spends its budget justifying its own
proposal. That is not a hypothetical: this portfolio has already watched a specification reach
3,000 lines and three review rounds before anyone asked whether the thing it specified could be
built at all. The gate is not bureaucracy — it is the mechanism that makes running four projects
at once cost the price of *selection* rather than the price of four builds.

A gate that can be argued past is not a gate. So:

- **The verdict is a number or a reproducible experiment.** Not an assessment, not a plan, not a
  reason the number will be better later.
- **Adding a hypothesis to stay alive is forbidden.** "It would work if we also had X" is a KILL,
  not a new scope. If X is genuinely the project, that is a different gate and it starts over.
- **Not-proven is a KILL, not a pending.** The burden is on the project. `BLOCKED` exists only for
  an external dependency that a stated action can resolve on a stated date.
- **Flattering errors run one way.** Every numeric error found in this portfolio so far has favoured
  the project that produced it, from four different authors. Recompute any number that decides this
  gate adversarially, from its definition, by someone who did not produce it, and state the
  direction of every discrepancy.

## Agents: at most two, never concurrent on the same artifact

| Role | Who | Does |
|---|---|---|
| Specification, evidence, task progression | **Claude** | frame-thin work: what the gate means, what would falsify it, what to run next, adjudicating results |
| Implementation **or** independent review | **Codex** | frame-thick work: exhaustive work inside a closed frame |

**Implementation and review never run at the same time.** One Codex role is active at a time, and
which one is a deliberate choice, not a default. No model reviews its own output.

## On reaching the gate: stop

`GO` does not start the next phase. It ends this one. On any verdict — `GO`, `KILLED`, `BLOCKED` —
the run stops, `STATUS.md` is updated, the evidence is committed and pushed, and the founder decides
what happens next. **Auto-entering the next phase is the failure this document exists to prevent.**

On `GO`, the permitted next step is **one** minimal task, chosen by the founder — not a plan, not a
roadmap, not a phase.

---
## The gate

**A reproducible certification-and-constraint demo against a real agent or agent vault.**

| # | Item | Verdict must come from |
|---|---|---|
| P1 | **A real target** — an actual on-chain agent or agent vault, not a fixture we wrote | its address, and a run against it |
| P2 | **Reproducible certification** — the same agent produces the same verdict on re-run, and a third party can reproduce it | two runs, and the command |
| P3 | **The constraint binds** — the demo shows a constraint *refusing* something the agent tried to do, not merely reporting on it | a refused action, with the attempt visible |
| P4 | **Differentiation** — what this establishes that an existing sandbox or fuzzer does not | a case they miss and this catches, run against both |

### KILLED if

**The differential against existing sandboxes and fuzzers cannot be shown, or the adopter is vague.**
"Agent developers" is not an adopter. Name who runs this, at what moment in their workflow, and what
they do differently because of the output.

### The trap specific to this project

A verification primitive can be correct, elegant, and adopted by nobody — this portfolio has produced
several. P4 is the gate. P1–P3 are how P4 is demonstrated, not substitutes for it. A demo that only
shows the tool working on our own fixture proves the tool runs, which was never the question.

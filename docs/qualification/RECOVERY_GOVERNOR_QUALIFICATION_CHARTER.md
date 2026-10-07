# Recovery Governor independent product qualification charter

**Tranche:** `RECOVERY_GOVERNOR_INDEPENDENT_PRODUCT_QUALIFICATION_DESIGN`  
**Subject:** `4d6cb052fd99f4e5678d25c4274615cd47681751` (tree `e0b433f1…`)  
**Product material:** `d90d92d` / tree `383cee4d`  
**Engineering repo:** `recovery-governor` · **Product:** Recovery Governor  
**Status:** design frozen · **execution:** not authorized

## §0 — Authoritative inputs

| Token | Value |
| --- | --- |
| `RG_IPQ1_SUBJECT_HEAD` | `4d6cb052fd99f4e5678d25c4274615cd47681751` |
| `RG_IPQ1_SUBJECT_TREE` | `e0b433f1f3fccc352bb828df7d7f4a3cd657d081` |
| `FABRIC_DEPENDENCY_HEAD` | `135bbd932553613fa3dd758d73143401068a4cb8` |
| `FABRIC_DEPENDENCY_TREE` | `4e724089c1df1a8bf4f891558983a5d6aad5fc28` |
| `RECOVERY_GOVERNOR_DESIGN_PACKAGE_ID` | `rg-design-v0-e7aae88949bd74f3` |
| `EFFECTGUARD_BRIDGE_SUBJECT_HEAD` | `b7c53685208138a981d7aa58f6caf41ceaa3a35d` |
| `FABRIC_M3_QUALIFIED` (input) | `true` (Lane E bounded) |
| `EFFECTGUARD_QUALIFIED` (adjacent) | `true` — **does not transfer** |
| `RECOVERY_GOVERNOR_QUALIFIED` | `false` |

## §1 — Qualification question (frozen)

> After Fabric has adjudicated truth and SafeNext, can Recovery Governor’s **case orchestration, advisory projection, and governed bridge** cause an operator or automation to **arm redispatch, proceed, or escalate** without Fabric/EffectGuard clearance — or to act on **stale** substrate state?

Attacks **RG-specific** faults (case binding, advisory pack, mode gating, EG bridge). **Must not** rerun Fabric M3 Lane E or re-qualify EffectGuard.

## §2 — Dependency model

```text
FABRIC_M3_QUALIFICATION_IS_INPUT_NOT_RG_VERDICT=true
EFFECTGUARD_QUALIFIED != RECOVERY_GOVERNOR_QUALIFIED
```

## §3 — Semantic firewall

Same tokens as design §7 — see [RECOVERY_GOVERNOR_PRODUCT_INVARIANTS.md](./RECOVERY_GOVERNOR_PRODUCT_INVARIANTS.md).

## §4 — Scope freeze

`PRODUCT_QUALIFICATION_SCOPE_FROZEN=v1-4d6cb05`

**In scope @ v1:** `CaseStore`, advisory SafeNext, governed dispatch bridge, revalidation, ancestry, wiremock profile via EG.  
**Out of scope:** Playbook catalog engine, RG-R live GitHub (Class I NOT_EXECUTED v1), process restart, M4 witness, LE-F export.

## §5 — Campaign structure (Classes A–I)

| Class | Prefix | Purpose |
| --- | --- | --- |
| **A** | RG-Q01..Q24 | Deterministic product-layer hostility |
| **B** | RG-B01..B12 | EffectGuard bridge fidelity |
| **C** | RG-C01..C10 | Case graph / mode / binding |
| **D** | RG-D01..D08 | Advisory pack integrity |
| **E** | RG-E01..E08 | Error handling (E01..E04 exist as slice) |
| **F** | RG-F01..F06 | Freshness / stale advisory |
| **G** | RG-G01..G05 | Fabric gap probes |
| **H** | RG-H01..H18 | Design-aligned hostility (**CF-001 closure**) |
| **I** | RG-R01..R06 | Real provider — **NOT_EXECUTED** v1 |

Non-substitutable: **A + B + H** minimum for bounded PASS; **I** cannot substitute **A**.

## §10 — Restart limitation

```text
LANE_E_GITHUB_RESTART_AMBIGUITY=NOT_EXECUTED
RG_PROCESS_RESTART_CASE=EXCLUDED_BOUNDED_LIMITATION
```

## §11 — Freshness

```text
RG_ORCHESTRATION_FRESHNESS_IS_LOAD_BEARING=true
```

`revalidate_advisory` + post-dispatch `advise_safe_next` must reflect latest EG outcome for leaf operation.

## §22 — Evidence bundle (execution future)

Pin: subject head/tree, EG bridge head, Fabric head, `cargo test` logs per class, findings ledger JSON.

## §33 — Design deliverables index

See [README.md](./README.md).

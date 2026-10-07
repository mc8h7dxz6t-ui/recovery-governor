# Recovery Governor V1 — preservation, design restoration, and conformance report

**Mission:** `RECOVERY_GOVERNOR_V1_PRESERVATION_DESIGN_CONFORMANCE_AND_PUBLICATION`  
**Worker:** `bc-fe8af931-5955-599e-b7bc-c1f4a0f18e97`  
**STOP_AFTER:** `RECOVERY_GOVERNOR_V1_PRESERVATION_DESIGN_CONFORMANCE_AND_PUBLICATION`

## §1 — V1 subject freeze

| Token | Value |
| --- | --- |
| `RG_V1_INITIAL_BUILD_SUBJECT` | `4d6cb052fd99f4e5678d25c4274615cd47681751` |
| `RG_V1_INITIAL_BUILD_TREE` | `e0b433f1f3fccc352bb828df7d7f4a3cd657d081` |
| `RG_V1_PRODUCT_CODE_SUBJECT` | `d90d92d008b02dd6f5f5a26b4008b43374ef000e` |
| Git tag | `RG_V1_INITIAL_BUILD_SUBJECT` → `4d6cb05` |

Post-subject commits (`ce39c41` report pin) touch **only** `RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD_REPORT.md` — product subject **not** relabelled.

## §2 — Design restoration

Eight files copied **exactly** from Project Context (`bc-ba58cd56…/files/docs/`) into `docs/design/` with SHA-256 recorded in `RECOVERY_GOVERNOR_DESIGN_PACKAGE_MANIFEST.json`.

```text
RECOVERY_GOVERNOR_DESIGN_PACKAGE_ID=rg-design-v0-e7aae88949bd74f3
DESIGN_FILE_RECONSTRUCTED=false
BLOCKED_DESIGN_SOURCE_INCOMPLETE=false
```

## §3 — Conformance verdict

```text
RECOVERY_GOVERNOR_V1_DESIGN_BUILD_CONFORMANCE=PASS_WITH_BOUNDED_LIMITATIONS
RECOVERY_GOVERNOR_V1_CONFORMANCE_VERDICT=PASS_WITH_BOUNDED_LIMITATIONS
LOAD_BEARING_MISMATCH=false
```

Findings: [RECOVERY_GOVERNOR_V1_CONFORMANCE_FINDINGS.json](./RECOVERY_GOVERNOR_V1_CONFORMANCE_FINDINGS.json)  
Matrix: [RECOVERY_GOVERNOR_V1_DESIGN_BUILD_CONFORMANCE_MATRIX.md](./RECOVERY_GOVERNOR_V1_DESIGN_BUILD_CONFORMANCE_MATRIX.md)

**Note:** CF-001 (HIGH, non-load-bearing) — hostile test **IDs** reused for V1 slice semantics; correction required before qualification, not for this preservation tranche.

## §4 — §11 Governed dispatch / EffectGuard bridge

| Design expectation | Build implementation | Assessment |
| --- | --- | --- |
| THIN_ADAPTER to EG `SessionStore` | `CaseStore` owns private `effectguard::SessionStore` | **Conformant** — no forked dispatch semantics |
| `governed_dispatch` → Fabric lifecycle | Delegates to `SessionStore::dispatch` | **Conformant** |
| `request_restart` clearance | Forwards to EG after PROCEED/clearance rules | **Conformant** |
| Advisory mode blocks dispatch | `ExecutionMode::Advisory` → `AdvisoryOnly` error | **Conformant** |
| No duplicate verify | No `effectfence-verify` in core path | **Conformant** |
| INV-18 no duplicate session semantics | Embeds EG store (not reimplemented register/dispatch) | **Conformant** with bounded embed note |

## §5 — Fabric dependency

| Field | Value |
| --- | --- |
| Remote | `doctrine` |
| Ref | `refs/heads/fabric/integration-m3-135bbd9` |
| HEAD | `135bbd932553613fa3dd758d73143401068a4cb8` |
| Tree | `4e724089c1df1a8bf4f891558983a5d6aad5fc28` |
| `FABRIC_CODE_CHANGED` | `false` |
| `EFFECTGUARD_CODE_CHANGED` | `false` |

## §6 — Remote publication

Executed only after conformance `PASS_WITH_BOUNDED_LIMITATIONS` (see §33 terminal).

## §7 — Artefact index

| Artefact | Path |
| --- | --- |
| Design package | `docs/design/` |
| Conformance | `docs/conformance/` |
| First build report | `RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD_REPORT.md` |

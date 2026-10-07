# RG subject identity reconciliation (Gate 1)

**Tranche:** `RECOVERY_GOVERNOR_INDEPENDENT_PRODUCT_QUALIFICATION_DESIGN`  
**Evaluated:** 2026-10-07T21:15:00+01:00

## Question

Which git object is the **Recovery Governor IPQ-1 qualification subject**, given `d90d92d` (product commit), `4d6cb05` (initial build report), and tag `RG_V1_INITIAL_BUILD_SUBJECT`?

## Evidence

| Object | HEAD | Tree | Contents |
| --- | --- | --- | --- |
| `d90d92d` | `d90d92d008b02dd6f5f5a26b4008b43374ef000e` | `383cee4def3cea9fcdd3b0e119c4ebe130473cac` | Product Rust + tests only |
| `4d6cb05` | `4d6cb052fd99f4e5678d25c4274615cd47681751` | `e0b433f1f3fccc352bb828df7d7f4a3cd657d081` | `d90d92d` + `RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD_REPORT.md` |
| Tag `RG_V1_INITIAL_BUILD_SUBJECT` | **points to** `4d6cb05` | `e0b433f1…` | Programme initial-build anchor |
| `ce39c41` | report HEAD pin tweak | — | Docs only; **not** product subject |
| `e21f770` | design + conformance docs | — | Docs only; **not** product subject |

```text
git diff --stat d90d92d..4d6cb05
→ RECOVERY_GOVERNOR_FIRST_PRODUCT_BUILD_REPORT.md only (157 lines)
```

## Resolution (frozen)

| Token | Value | Role |
| --- | --- | --- |
| `RG_IPQ1_SUBJECT_HEAD` | `4d6cb052fd99f4e5678d25c4274615cd47681751` | **Qualification repo subject** (matches tag) |
| `RG_IPQ1_SUBJECT_TREE` | `e0b433f1f3fccc352bb828df7d7f4a3cd657d081` | Tree under test @ execution |
| `RG_IPQ1_PRODUCT_MATERIAL_HEAD` | `d90d92d008b02dd6f5f5a26b4008b43374ef000e` | Product-only delta baseline |
| `RG_IPQ1_PRODUCT_MATERIAL_TREE` | `383cee4def3cea9fcdd3b0e119c4ebe130473cac` | Rust crates @ subject |

```text
BLOCKED_SUBJECT_IDENTITY=false
RG_V1_INITIAL_BUILD_SUBJECT_TAG_MATCHES_IPQ1_HEAD=true
PRODUCT_MATERIAL_UNCHANGED_SINCE_d90d92d=true
```

## Post-subject documentation commits

Commits after `4d6cb05` on `main` (`ce39c41`, `e21f770`, qualification design) **must not** change product-material Rust at `RG_IPQ1_SUBJECT_HEAD`. Execution harness checks `git diff d90d92d..$SUBJECT -- crates/` is empty.

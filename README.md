# recovery-governor

Engineering repository for **Recovery Governor** — a case-level advisory SafeNext projection over Consequence Fabric M3 (`fabric-m3-consequence` via `effectguard`). Recovery Governor does **not** own canonical truth or override Fabric SafeNext.

```text
ENGINEERING_REPO_NAME=recovery-governor
PRODUCT_NAME=Recovery Governor
FABRIC_M3_SUBSTRATE_PIN=135bbd932553613fa3dd758d73143401068a4cb8
RECOVERY_GOVERNOR_FABRIC_CANONICAL_REF=refs/heads/fabric/integration-m3-135bbd9
RECOVERY_GOVERNOR_DEFAULT_EXECUTION_MODE=ADVISORY
RECOVERY_GOVERNOR_CAN_MUTATE_CANONICAL_TRUTH=false
RECOVERY_GOVERNOR_QUALIFIED=false
FABRIC_CODE_CHANGED=false
```

## Build

```bash
cargo fmt --all
cargo build --all
cargo clippy --all-targets -- -D warnings
cargo test --all
./scripts/secret_scan.sh
./scripts/claim_lint.sh
```

## Product API (`recovery-governor` crate)

- `open_case` — recovery case with optional parent (ancestry) and execution mode (`ADVISORY` default)
- `bind_operation` — attach a Fabric operation (EffectGuard session) as case leaf
- `advise_safe_next` — **projection only**; emits `recovery-governor.pack/v1`
- `revalidate_advisory` — stale decision detection via outcome fingerprint
- `governed_begin_attempt` / `governed_dispatch` — only when mode is `GOVERNED`
- `ancestry` / `case_status` — case graph introspection

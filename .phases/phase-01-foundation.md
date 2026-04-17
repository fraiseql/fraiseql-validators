# Phase 1: Foundation

## Objective

Stand up the crate skeleton: `no_std` + `alloc` configuration, `ValidationError`, the
`TryFrom<&str>` / `Display` contract, feature-flag scaffolding, `build.rs` for CSV whitelist
code generation, and CI. No domain types yet.

## Success Criteria

- [ ] `cargo check --no-default-features` succeeds (no_std path)
- [ ] `cargo check --all-features` succeeds
- [ ] `cargo test` succeeds with the contract smoke tests below
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [ ] `build.rs` processes a sample CSV and emits a `const` sorted array
- [ ] Feature flags are declared; each compiles independently

## TDD Cycles

### Cycle 1: Crate skeleton & `no_std` configuration

- **RED**: Add a test that imports from `fraiseql_validators` (fails — crate doesn't exist)
- **GREEN**:
  - `Cargo.toml` with `#![no_std]` + `extern crate alloc`
  - Feature flags: `contact`, `financial_banking`, `financial_securities`, `barcodes`,
    `identifiers`, `geographic`, `network`; default = all
  - Dependency: `regex-lite` (no_std-compatible), `once_cell` (alloc feature)
- **REFACTOR**: Confirm `lib.rs` gate structure is clean; each feature re-exports its module
- **CLEANUP**: `cargo fmt`, `cargo clippy`

### Cycle 2: `ValidationError`

- **RED**: Write a test asserting `ValidationError` fields are accessible and `Display` works
- **GREEN**:
  ```rust
  pub struct ValidationError {
      pub type_name: &'static str,
      pub input: alloc::string::String,
      pub reason: &'static str,
  }
  impl core::fmt::Display for ValidationError { ... }
  ```
- **REFACTOR**: Ensure `Display` output is human-readable and stable
- **CLEANUP**: Lint, commit

### Cycle 3: `TryFrom` / `Display` contract test helper

- **RED**: Write a generic `assert_roundtrip` helper test that calls `T::try_from(s)` and
  checks `t.to_string() == s`; write a matching `assert_rejects` helper
- **GREEN**: Implement the helpers in `tests/helpers.rs`
- **REFACTOR**: Make helpers ergonomic for future phases
- **CLEANUP**: Lint, commit

### Cycle 4: `build.rs` — CSV whitelist code generation

- **RED**: Write a test that calls a generated `is_valid_sample_code(&str) -> bool` function
  (fails — generator not written)
- **GREEN**:
  - `build.rs` reads `data/sample.csv` (one-column, header "code"), emits
    `OUT_DIR/sample_codes.rs` with `const SAMPLE_CODES: &[&str] = &[...];` (sorted)
  - Expose `pub fn is_valid_sample_code(s: &str) -> bool` via binary search on the const
  - Include generated file with `include!(concat!(env!("OUT_DIR"), "/sample_codes.rs"))`
- **REFACTOR**: Extract `build.rs` helpers (`emit_sorted_str_set`, `read_csv_column`) so
  future phases reuse the same pattern
- **CLEANUP**: Delete sample data after pattern is established, lint, commit

## Dependencies

- Requires: nothing
- Blocks: all other phases

## Notes

- `regex-lite` version: use latest `^0.1` (check crates.io at implementation time)
- `once_cell` version: use latest `^1` with `alloc` feature, no `std` feature
- The `build.rs` helpers become the reusable scaffold for every whitelist type in later phases
- Port Luhn and Mod97 from `fraiseql-core/src/validation/checksum.rs` in Phase 3; don't bring
  them in here (YAGNI until they're needed)

## Status

[ ] Not Started

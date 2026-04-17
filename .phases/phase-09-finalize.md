# Phase 9: Finalize

## Objective

Transform working code into a production-ready, publishable crate. API review, documentation
polish, archaeology removal, and crates.io publication prep.

## Success Criteria

- [ ] `cargo test --all-features` passes with zero failures
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean
- [ ] `cargo doc --all-features --no-deps` builds with zero warnings
- [ ] `git grep -i "phase\|todo\|fixme\|hack\|unwrap"` returns nothing (see note on unwrap)
- [ ] All public types and functions have doc comments
- [ ] `Cargo.toml` metadata complete (description, keywords, categories, license, repository)
- [ ] `README.md` accurate and complete with usage examples for each feature flag
- [ ] Integration smoke test: fraiseql compiles with fraiseql-validators for `Email` and
  `PhoneE164` (replacing inline validation)

## Steps

### 1. Quality Control Review

Review as a senior software engineer:

- [ ] `TryFrom<&str>` / `Display` round-trip holds for every type
- [ ] All accessor methods return the right slices / values (no off-by-one)
- [ ] `ValidationError` messages are clear and actionable
- [ ] `no_std` path: `cargo check --no-default-features --target thumbv7m-none-eabi`
      (or equivalent bare-metal target) succeeds
- [ ] Feature flags compile independently:
  `cargo check --no-default-features --features contact`, etc.
- [ ] Compile-time check: `cargo check --all-features` under 10 seconds on CI
- [ ] Ordering types (`Semver`) have correct `PartialOrd` / `Ord` / `PartialEq` / `Eq`
  consistency

### 2. Security Audit

Review as a hacker:

- [ ] No regex with catastrophic backtracking potential (all patterns are linear under
  regex-lite's NFA engine — verify)
- [ ] No `unwrap()` / `expect()` in non-test code (grep for exceptions and justify each)
- [ ] `build.rs` does not download anything at build time (all data committed to `data/`)
- [ ] CSV parsing in `build.rs` does not panic on malformed input (add guards)
- [ ] Whitelist binary searches cannot panic (verify `CODES.binary_search()` paths)
- [ ] No secrets or PII in test vectors (real ISINs / IBANs in tests are public instruments)

### 3. Archaeology Removal

- [ ] Remove all `// Phase X:` comments
- [ ] Remove all `# TODO: Phase` markers
- [ ] Fix or remove all `FIXME` comments
- [ ] Remove all commented-out code
- [ ] Remove `.phases/` directory from the main branch before publishing
      (keep in git history for reference)
- [ ] `git grep -i "phase\|todo\|fixme\|hack"` returns nothing

Note on `unwrap`: `expect()` calls that are provably unreachable (e.g., `"pre-filtered to
numeric chars only"` in the Luhn algorithm) are acceptable with a justifying comment, and
are exempted from the grep check. Document each exemption.

### 4. Documentation Polish

- [ ] Top-level `lib.rs` doc comment: crate overview, feature flags, quick-start example
- [ ] Each module has a module-level doc comment
- [ ] Each public type: doc comment with format description, valid examples, rejection reasons
- [ ] Each accessor: doc comment with example
- [ ] `ValidationError` display format documented
- [ ] `README.md`: installation, feature flags table, one example per feature group
- [ ] `CHANGELOG.md`: v0.1.0 entry

### 5. Integration Smoke Test

In the fraiseql repo (not in this repo):
- [ ] Add `fraiseql-validators` as a path dependency
- [ ] Replace inline `Email` validation with `fraiseql_validators::Email::try_from()`
- [ ] Replace inline `PhoneE164` validation with `fraiseql_validators::PhoneE164::try_from()`
- [ ] `cargo test` in fraiseql passes

Document the integration pattern in the crate README.

### 6. Publication Prep

- [ ] `Cargo.toml` fields: `name`, `version = "0.1.0"`, `edition = "2021"`, `description`,
  `keywords` (max 5), `categories`, `license = "MIT OR Apache-2.0"`, `repository`,
  `documentation`, `readme = "README.md"`
- [ ] `LICENSE-MIT` and `LICENSE-APACHE` files present
- [ ] `cargo publish --dry-run --all-features` succeeds
- [ ] Tag `v0.1.0`

## Status

[ ] Not Started

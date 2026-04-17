# Phase 2: Contact Types

## Objective

Implement `Email`, `PhoneE164`, and `DomainName` under the `contact` feature flag.

## Success Criteria

- [ ] All three types implement `TryFrom<&str>` and `Display`
- [ ] All three round-trip: `T::try_from(&t.to_string()) == Ok(t)`
- [ ] Each type: at least one valid test, one test per documented rejection reason
- [ ] Accessor methods tested
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean

## Reuse from fraiseql-core

Port these patterns verbatim from `fraiseql-core/src/validation/patterns.rs`:
- `EMAIL`: RFC 5321 practical subset regex (already correct)
- `PHONE_E164`: `^\+[1-9]\d{6,14}$` (already correct)

## TDD Cycles

### Cycle 1: `Email`

- **RED**: Tests for —
  - `Email::try_from("user@example.com")` → Ok
  - `Email::try_from("user@localhost")` → Err (single-label domain)
  - `Email::try_from("")` → Err
  - `Email::try_from("a".repeat(255) + "@example.com")` → Err (> 254 chars)
  - `email.local()` → `"user"`
  - `email.domain()` → `"example.com"`
  - `email.belongs_to_domain("example.com")` → true
  - Case-insensitive `PartialEq`: `"User@Example.COM" == "user@example.com"`
- **GREEN**:
  ```rust
  pub struct Email(String);  // stored normalised to lowercase
  impl TryFrom<&str> for Email { ... }  // regex match + length guard
  impl Display for Email { ... }
  impl Email {
      pub fn local(&self) -> &str { ... }
      pub fn domain(&self) -> &str { ... }
      pub fn belongs_to_domain(&self, domain: &str) -> bool { ... }
  }
  impl PartialEq for Email { ... }  // case-insensitive
  ```
- **REFACTOR**: Store normalised to lowercase on parse so accessors are free `&str` slices
- **CLEANUP**: Lint, commit

### Cycle 2: `PhoneE164`

- **RED**: Tests for —
  - `PhoneE164::try_from("+14155552671")` → Ok
  - `PhoneE164::try_from("14155552671")` → Err (missing `+`)
  - `PhoneE164::try_from("+1")` → Err (too short)
  - `PhoneE164::try_from("+123456789012345678")` → Err (too long)
  - `phone.country_code()` → `"1"`
  - `phone.national_number()` → `"4155552671"`
- **GREEN**:
  ```rust
  pub struct PhoneE164(String);
  impl TryFrom<&str> for PhoneE164 { ... }
  impl PhoneE164 {
      pub fn country_code(&self) -> &str { ... }    // digits after +, 1–3 chars
      pub fn national_number(&self) -> &str { ... } // remainder
  }
  ```
  Country code length: use a lookup (or simple heuristic: try 1, 2, 3 digit prefixes against
  known ITU allocations). For v1 a simple structural approach (1 digit if starts with 1 or 7;
  2 digits for most; 3 for smaller territories) is acceptable with a TODO for full ITU table.
- **REFACTOR**: Decide on country code extraction strategy; document it
- **CLEANUP**: Lint, commit

### Cycle 3: `DomainName`

- **RED**: Tests for —
  - `DomainName::try_from("example.com")` → Ok
  - `DomainName::try_from("sub.example.co.uk")` → Ok
  - `DomainName::try_from("localhost")` → Ok (single label is valid DNS)
  - `DomainName::try_from("-example.com")` → Err (label starts with hyphen)
  - `DomainName::try_from("example-.com")` → Err (label ends with hyphen)
  - `DomainName::try_from("example..com")` → Err (empty label)
  - `DomainName::try_from("exa mple.com")` → Err (space)
  - Label > 63 chars → Err
  - Total > 253 chars → Err
  - `DomainName::try_from("https://example.com")` → Err (scheme present)
- **GREEN**:
  ```rust
  pub struct DomainName(String);  // stored lowercase
  impl TryFrom<&str> for DomainName { ... }  // split on '.', validate each label
  ```
  Hand-written label validator (no regex needed — straightforward iterator logic).
- **REFACTOR**: Extract `is_valid_label(s: &str) -> bool` as a reusable private fn
  (used again in `Email` domain validation if we ever tighten that)
- **CLEANUP**: Lint, commit

## Dependencies

- Requires: Phase 1 complete

## Status

[ ] Not Started

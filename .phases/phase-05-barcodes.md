# Phase 5: Barcodes & Product IDs

## Objective

Implement `Ean8`, `Ean13`, `UpcA`, `Isbn13`, `Issn`, and `Gtin14` under the `barcodes`
feature flag. All share the same GS1 check digit algorithm.

## Success Criteria

- [ ] All six types implement `TryFrom<&str>` and `Display`
- [ ] GS1 check digit algorithm tested in isolation
- [ ] Each type: valid + all rejection reasons covered in tests
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean

## Notes on the GS1 check digit

EAN-8, EAN-13, UPC-A, GTIN-14 all use the same GS1 check digit algorithm:
- Alternate multiply digits by 3 and 1 (right-to-left, starting with ×3 for the
  rightmost non-check digit)
- Sum all products; check digit = (10 - sum % 10) % 10

ISBN-13 is an EAN-13 with prefix 978 or 979 — same check digit algorithm.
ISSN uses a different algorithm: weighted sum mod 11.

Extract `gs1_check_digit_valid(s: &str) -> bool` as a shared private fn.

## TDD Cycles

### Cycle 1: GS1 check digit algorithm

- **RED**: Tests for known-good and known-bad barcodes
- **GREEN**: `fn gs1_check_digit_valid(digits: &str) -> bool` — pure digits, any length ≥ 2
- **REFACTOR**: Make it generic over length (EAN-8, EAN-13, UPC-A, GTIN-14 all use it)
- **CLEANUP**: Lint, commit

### Cycle 2: `Ean8`

EAN-8: exactly 8 digits, GS1 check digit.

- **RED**: Tests for —
  - `Ean8::try_from("73513537")` → Ok
  - `Ean8::try_from("73513538")` → Err (bad check digit)
  - `Ean8::try_from("7351353")` → Err (too short)
  - `Ean8::try_from("735135370")` → Err (too long)
  - `Ean8::try_from("7351353A")` → Err (non-digit)
  - `ean.check_digit()` → `'7'`
- **GREEN**:
  ```rust
  pub struct Ean8([u8; 8]);  // stored as ASCII digits
  impl TryFrom<&str> for Ean8 {
      // 1. Exactly 8 chars, all ASCII digits
      // 2. gs1_check_digit_valid
  }
  impl Display for Ean8 { /* render as 8-digit string */ }
  ```
- **REFACTOR**: Assess whether `[u8; N]` storage vs `String` is worth it for all barcode
  types; decide consistently for the whole phase
- **CLEANUP**: Lint, commit

### Cycle 3: `UpcA`

UPC-A: exactly 12 digits, GS1 check digit. (Structurally EAN-13 with implicit leading 0.)

- **RED**: Tests for —
  - `UpcA::try_from("012345678905")` → Ok
  - `UpcA::try_from("012345678906")` → Err (bad check digit)
  - `UpcA::try_from("01234567890")` → Err (too short)
  - `upca.to_ean13()` → `Ean13` (prepend "0")
- **GREEN**: `pub struct UpcA([u8; 12]);`
- **REFACTOR**: `to_ean13()` should be zero-copy or at least allocation-minimal
- **CLEANUP**: Lint, commit

### Cycle 4: `Ean13`

EAN-13: exactly 13 digits, GS1 check digit.

- **RED**: Tests for —
  - `Ean13::try_from("5901234123457")` → Ok
  - `Ean13::try_from("5901234123458")` → Err
  - `Ean13::try_from("978030640615")` → Err (12 digits, too short for EAN-13)
  - `ean.gs1_prefix()` → `"590"` (first 3 digits, GS1 company prefix region indicator)
- **GREEN**: `pub struct Ean13([u8; 13]);`
- **REFACTOR**: —
- **CLEANUP**: Lint, commit

### Cycle 5: `Isbn13`

ISBN-13 is EAN-13 restricted to prefixes 978 or 979.

- **RED**: Tests for —
  - `Isbn13::try_from("9780306406157")` → Ok
  - `Isbn13::try_from("9790306406157")` → Ok (979 prefix)
  - `Isbn13::try_from("9770306406157")` → Err (977 prefix — not ISBN)
  - `Isbn13::try_from("9780306406158")` → Err (bad check digit)
  - `isbn.registration_group()` → `"0"` (chars 4–4, simplified — full ISBN agency table is
    out of scope for v1; document this)
  - `isbn.as_ean13()` → `Ean13`
- **GREEN**: `pub struct Isbn13([u8; 13]);`
- **REFACTOR**: `as_ean13()` is a zero-cost transmute (same bytes)
- **CLEANUP**: Lint, commit

### Cycle 6: `Issn`

ISSN: exactly 8 chars — 7 digits + check char (digit or 'X'). Weighted sum mod 11.
Algorithm: sum of digit × weight (8,7,6,5,4,3,2); check = (11 - sum % 11) % 11; if 10 → 'X'.

- **RED**: Tests for —
  - `Issn::try_from("0317-8471")` → Ok (with hyphen — normalise)
  - `Issn::try_from("03178471")` → Ok (without hyphen)
  - `Issn::try_from("0378-5955")` → Ok
  - `Issn::try_from("0317-8472")` → Err (bad check digit)
  - `issn.display()` → `"0317-8471"` (canonical form with hyphen at position 4)
- **GREEN**:
  ```rust
  pub struct Issn([u8; 8]);  // stored as 8 ASCII chars (digits + optional 'X')
  impl TryFrom<&str> for Issn {
      // 1. Strip hyphens
      // 2. Must be exactly 8 chars after stripping
      // 3. First 7 must be digits; last must be digit or 'X'
      // 4. Weighted sum check
  }
  impl Display for Issn {
      // Render as XXXX-XXXX
  }
  ```
- **REFACTOR**: Display with hyphen is the canonical/standard form
- **CLEANUP**: Lint, commit

### Cycle 7: `Gtin14`

GTIN-14: exactly 14 digits, GS1 check digit. Supersedes all shorter GTINs.

- **RED**: Tests for —
  - `Gtin14::try_from("10614141000415")` → Ok
  - `Gtin14::try_from("10614141000416")` → Err (bad check digit)
  - `Gtin14::try_from("1061414100041")` → Err (too short)
  - `gtin.indicator_digit()` → `'1'` (first char, packaging level 1–8 or 0 for GTIN-13)
  - `gtin.as_ean13()` → `Option<Ean13>` (Some if indicator digit is 0)
- **GREEN**: `pub struct Gtin14([u8; 14]);`
- **REFACTOR**: —
- **CLEANUP**: Lint, commit

## Dependencies

- Requires: Phase 1 complete
- Does not require Phase 3 (no Luhn/Mod97 — GS1 algorithm is different)

## Status

[ ] Not Started

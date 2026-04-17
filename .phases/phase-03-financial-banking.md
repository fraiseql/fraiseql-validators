# Phase 3: Financial — Banking Types

## Objective

Implement `Iban`, `Isin`, `CurrencyCode`, and `SwiftBic` under the `financial_banking`
feature flag. Port Luhn and Mod97 checksum algorithms from fraiseql-core.

## Success Criteria

- [ ] All four types implement `TryFrom<&str>` and `Display`
- [ ] Luhn and Mod97 algorithms are correct and tested in isolation
- [ ] `CurrencyCode` whitelist generated from ISO 4217 CSV via `build.rs`
- [ ] Each type: valid + all rejection reasons covered in tests
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean

## Reuse from fraiseql-core

Port verbatim from `fraiseql-core/src/validation/checksum.rs`:
- `LuhnValidator::validate(s: &str) -> bool`
- `Mod97Validator::validate(s: &str) -> bool`

Adjust to use `alloc` instead of `std` where needed. Tests port too.

## Data sources

- `CurrencyCode`: ISO 4217 — download the "List One" XML or CSV from the ISO 4217 maintenance
  agency. Extract the 3-letter alpha codes. Commit to `data/iso4217_currencies.csv`.

## TDD Cycles

### Cycle 1: Checksum algorithms

- **RED**: Port existing tests from fraiseql-core checksum.rs (Luhn + Mod97)
- **GREEN**: Port `LuhnValidator` and `Mod97Validator` as private module `crate::checksum`
- **REFACTOR**: Make them `pub(crate)` free functions: `luhn_valid(s: &str) -> bool`,
  `mod97_valid(s: &str) -> bool`
- **CLEANUP**: Lint, commit

### Cycle 2: `Iban`

IBAN structure: `[A-Z]{2}[0-9]{2}[A-Z0-9]{11,30}` (total 15–34 chars), mod-97 checksum.

- **RED**: Tests for —
  - `Iban::try_from("GB82WEST12345698765432")` → Ok
  - `Iban::try_from("DE89370400440532013000")` → Ok
  - `Iban::try_from("gb82west12345698765432")` → Ok (normalise to uppercase)
  - `Iban::try_from("")` → Err
  - `Iban::try_from("GB82WEST1234569876543X")` → Err (checksum fail)
  - `Iban::try_from("GBXXWEST12345698765432")` → Err (check digits not numeric)
  - `iban.country()` → `"GB"`
  - `iban.bban()` → `"WEST12345698765432"`
  - `iban.check_digits()` → `"82"`
- **GREEN**:
  ```rust
  pub struct Iban(String);  // stored uppercase
  impl TryFrom<&str> for Iban {
      // 1. Uppercase
      // 2. Length 15–34
      // 3. chars 0–1 alpha, chars 2–3 digit
      // 4. remainder alphanumeric
      // 5. mod97_valid
  }
  impl Iban {
      pub fn country(&self) -> &str { &self.0[..2] }
      pub fn check_digits(&self) -> &str { &self.0[2..4] }
      pub fn bban(&self) -> &str { &self.0[4..] }
  }
  ```
- **REFACTOR**: Consider per-country length table (optional for v1 — structural + checksum
  already catches most forgeries; document the decision)
- **CLEANUP**: Lint, commit

### Cycle 3: `Isin`

ISIN structure: `[A-Z]{2}[A-Z0-9]{9}[0-9]` (exactly 12 chars), Luhn check digit on the
numeric expansion of all characters.

- **RED**: Tests for —
  - `Isin::try_from("US0231351067")` → Ok (Apple)
  - `Isin::try_from("GB0002634946")` → Ok (GlaxoSmithKline)
  - `Isin::try_from("us0231351067")` → Ok (normalise)
  - `Isin::try_from("US023135106X")` → Err (bad check digit)
  - `Isin::try_from("XX0231351067")` → Err (invalid country? — see note)
  - `Isin::try_from("US02313510678")` → Err (too long)
  - `isin.country()` → `"US"`
  - `isin.nsin()` → `"023135106"`
  - `isin.check_digit()` → `'7'`
- **GREEN**:
  ```rust
  pub struct Isin(String);  // stored uppercase
  impl TryFrom<&str> for Isin {
      // 1. Uppercase, exactly 12 chars
      // 2. chars 0–1: [A-Z]{2}
      // 3. chars 2–10: [A-Z0-9]{9}
      // 4. char 11: digit
      // 5. Luhn on numeric expansion (A→10, B→11, …, Z→35 then concatenate)
  }
  ```
  Note: country code validation is intentionally lenient (format only, not ISO 3166-1 whitelist)
  because ISINs exist for territories not in the standard country code list (e.g. XS for
  Euroclear, EU for pan-European instruments).
- **REFACTOR**: Extract `isin_numeric_expansion(s: &str) -> String` as a private helper
- **CLEANUP**: Lint, commit

### Cycle 4: `CurrencyCode`

ISO 4217 alpha-3 whitelist, ~180 codes. Generated from CSV via `build.rs`.

- **RED**: Tests for —
  - `CurrencyCode::try_from("USD")` → Ok
  - `CurrencyCode::try_from("EUR")` → Ok
  - `CurrencyCode::try_from("XTS")` → Ok (reserved for testing in ISO 4217)
  - `CurrencyCode::try_from("usd")` → Ok (normalise to uppercase)
  - `CurrencyCode::try_from("ABC")` → Err (not in whitelist)
  - `CurrencyCode::try_from("US")` → Err (wrong length)
  - `currency.as_str()` → `"USD"`
- **GREEN**:
  - Add `data/iso4217_currencies.csv` (one column: `alpha_code`)
  - `build.rs` emits `const CURRENCY_CODES: &[&str]` (sorted)
  - `CurrencyCode([u8; 3])` — fixed-size storage, zero heap
  ```rust
  pub struct CurrencyCode([u8; 3]);
  impl TryFrom<&str> for CurrencyCode {
      // 1. Uppercase
      // 2. Exactly 3 chars, all ASCII alpha
      // 3. Binary search in CURRENCY_CODES
  }
  impl Display for CurrencyCode { /* from [u8; 3] */ }
  impl CurrencyCode { pub fn as_str(&self) -> &str { ... } }
  ```
- **REFACTOR**: The `[u8; 3]` → `&str` conversion is shared with `CountryCode` later;
  extract a private macro or inline it (decide in Phase 7)
- **CLEANUP**: Lint, commit

### Cycle 5: `SwiftBic`

SWIFT BIC structure (ISO 9362): `[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3})?`
— 8 or 11 chars. Purely structural, no checksum.

- **RED**: Tests for —
  - `SwiftBic::try_from("DEUTDEDB")` → Ok (8-char, Deutsche Bank Frankfurt)
  - `SwiftBic::try_from("DEUTDEDBBER")` → Ok (11-char, branch)
  - `SwiftBic::try_from("deutdedb")` → Ok (normalise to uppercase)
  - `SwiftBic::try_from("DEUT1EDB")` → Err (digit in institution code)
  - `SwiftBic::try_from("DEUTDE")` → Err (too short)
  - `SwiftBic::try_from("DEUTDEDB1234")` → Err (too long / not 8 or 11)
  - `bic.institution_code()` → `"DEUT"`
  - `bic.country_code()` → `"DE"`
  - `bic.location_code()` → `"DB"`
  - `bic.branch_code()` → `Some("BER")` or `None`
- **GREEN**:
  ```rust
  pub struct SwiftBic(String);  // stored uppercase, 8 or 11 chars
  impl TryFrom<&str> for SwiftBic { ... }  // regex: ^[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}([A-Z0-9]{3})?$
  impl SwiftBic {
      pub fn institution_code(&self) -> &str { &self.0[..4] }
      pub fn country_code(&self) -> &str { &self.0[4..6] }
      pub fn location_code(&self) -> &str { &self.0[6..8] }
      pub fn branch_code(&self) -> Option<&str> { ... }
  }
  ```
- **REFACTOR**: Verify country_code accessor could share validation with CountryCode whitelist
  in a later phase (document as a future improvement)
- **CLEANUP**: Lint, commit

## Dependencies

- Requires: Phase 1 complete
- Blocks: Phase 4 (securities types reuse Luhn from this phase)

## Status

[ ] Not Started

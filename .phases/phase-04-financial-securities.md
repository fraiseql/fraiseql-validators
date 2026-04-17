# Phase 4: Financial — Securities Types

## Objective

Implement `Cusip`, `Sedol`, `Lei`, `Figi`, and `Mic` under the `financial_securities`
feature flag. All reuse the Luhn algorithm from Phase 3.

## Success Criteria

- [ ] All five types implement `TryFrom<&str>` and `Display`
- [ ] `Mic` whitelist generated from ISO 10383 CSV via `build.rs`
- [ ] Each type: valid + all rejection reasons covered in tests
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean

## Data sources

- `Mic`: ISO 10383 — download from iso20022.org/market-identifier-codes (free CSV).
  Commit to `data/iso10383_mic.csv`. Extract the `MIC` column.

## TDD Cycles

### Cycle 1: `Cusip`

CUSIP structure (ANSI X9.6): `[0-9A-Z*@#]{8}[0-9]` — exactly 9 chars.
Check digit: Luhn-like weighted sum (different weights than standard Luhn — see ANSI X9.6).

- **RED**: Tests for —
  - `Cusip::try_from("037833100")` → Ok (Apple)
  - `Cusip::try_from("38259P508")` → Ok (Google / Alphabet)
  - `Cusip::try_from("037833100".to_lowercase())` → Ok (normalise)
  - `Cusip::try_from("037833101")` → Err (bad check digit)
  - `Cusip::try_from("03783310")` → Err (too short)
  - `Cusip::try_from("0378331001")` → Err (too long)
  - `cusip.issuer_code()` → `"037833"` (first 6 chars)
  - `cusip.issue_number()` → `"10"` (chars 7–8)
  - `cusip.check_digit()` → `'0'`
- **GREEN**:
  CUSIP check digit algorithm (ANSI X9.6):
  - Map each char: digits → face value; A–Z → 10–35; `*` → 36; `@` → 37; `#` → 38
  - Multiply odd-position values × 1, even-position × 2
  - Sum individual digits of each product, then mod 10
  - Check digit = (10 - sum % 10) % 10
  ```rust
  pub struct Cusip(String);  // stored uppercase
  ```
- **REFACTOR**: Extract `cusip_check_digit(s: &[u8]) -> u8` as private fn
- **CLEANUP**: Lint, commit

### Cycle 2: `Sedol`

SEDOL structure (London Stock Exchange): `[0-9B-DF-HJ-NP-TV-Z]{6}[0-9]` — exactly 7 chars
(digits and consonants only — vowels excluded from positions 1–6).
Check digit: weighted sum mod 10.

- **RED**: Tests for —
  - `Sedol::try_from("0263494")` → Ok (BAE Systems)
  - `Sedol::try_from("B06YXY3")` → Ok (alphanumeric SEDOL)
  - `Sedol::try_from("b06yxy3")` → Ok (normalise)
  - `Sedol::try_from("0263495")` → Err (bad check digit)
  - `Sedol::try_from("026349")` → Err (too short)
  - `Sedol::try_from("A263494")` → Err (vowel in position 0)
  - `sedol.check_digit()` → `'4'`
- **GREEN**:
  SEDOL check digit: weights = [1, 3, 1, 7, 3, 9, 1]; sum of weight × char_value; mod 10.
  Char value: digits → face value; B–Z consonants → 11–35 (skipping A,E,I,O,U).
  ```rust
  pub struct Sedol(String);  // stored uppercase
  ```
- **REFACTOR**: Extract check-digit computation as private fn
- **CLEANUP**: Lint, commit

### Cycle 3: `Lei`

LEI structure (ISO 17442): `[A-Z0-9]{4}[0-9]{2}[A-Z0-9]{12}[0-9]{2}` — exactly 20 chars.
Check digits (positions 19–20): Mod97 on the numeric expansion (same as IBAN).

- **RED**: Tests for —
  - `Lei::try_from("7LTWFZYICNSX8D621K86")` → Ok (Apple Inc.)
  - `Lei::try_from("7ltwfzyicnsx8d621k86")` → Ok (normalise)
  - `Lei::try_from("7LTWFZYICNSX8D621K87")` → Err (bad check digits)
  - `Lei::try_from("7LTWFZYICNSX8D621K8")` → Err (too short)
  - `lei.lou_code()` → `"7LTW"` (first 4 chars, Legal Operating Unit prefix)
  - `lei.entity_code()` → `"FZYICNSX8D621K"` (chars 5–18)
  - `lei.check_digits()` → `"86"`
- **GREEN**:
  ```rust
  pub struct Lei(String);  // stored uppercase
  impl TryFrom<&str> for Lei {
      // 1. Uppercase, exactly 20 chars
      // 2. Structural char-class check
      // 3. mod97_valid (reuse from Phase 3)
  }
  impl Lei {
      pub fn lou_code(&self) -> &str { &self.0[..4] }
      pub fn entity_code(&self) -> &str { &self.0[4..18] }
      pub fn check_digits(&self) -> &str { &self.0[18..] }
  }
  ```
- **REFACTOR**: Confirm mod97 reuse path is clean
- **CLEANUP**: Lint, commit

### Cycle 4: `Figi`

FIGI structure (OpenFIGI): exactly 12 chars — `[B-DF-HJ-NP-TV-Z]{2}[G][A-Z0-9]{8}[0-9]`
(first 2 chars are consonants, char 3 is always 'G', chars 4–11 are alphanumeric, char 12 is
Luhn check digit). Validated structurally + Luhn; no full-corpus whitelist (impractical).

- **RED**: Tests for —
  - `Figi::try_from("BBG000BLNNH6")` → Ok (Apple FIGI)
  - `Figi::try_from("bbg000blnnh6")` → Ok (normalise)
  - `Figi::try_from("BBG000BLNNH7")` → Err (bad check digit)
  - `Figi::try_from("BBG000BLNNH")` → Err (too short)
  - `Figi::try_from("BAG000BLNNH6")` → Err (char 3 is not 'G')
  - `Figi::try_from("ABG000BLNNH6")` → Err (char 1 is vowel)
  - `figi.provider_code()` → `"BB"` (first 2 chars, always "BB" for Bloomberg)
  - `figi.security_code()` → `"G000BLNNH"` (chars 3–11)
  - `figi.check_digit()` → `'6'`
- **GREEN**:
  ```rust
  pub struct Figi(String);  // stored uppercase
  impl TryFrom<&str> for Figi {
      // 1. Uppercase, exactly 12 chars
      // 2. chars 0–1: consonants [B-DF-HJ-NP-TV-Z]
      // 3. char 2: 'G'
      // 4. chars 3–10: [A-Z0-9]
      // 5. char 11: digit, passes Luhn on full string numeric expansion
  }
  ```
- **REFACTOR**: Luhn expansion for FIGI: same as ISIN (letter → digit pair, then Luhn)
- **CLEANUP**: Lint, commit

### Cycle 5: `Mic`

ISO 10383 Market Identifier Code — exactly 4 uppercase alpha chars, whitelisted.
~700 active MICs. Generated from ISO 10383 CSV via `build.rs`.

- **RED**: Tests for —
  - `Mic::try_from("XNYS")` → Ok (NYSE)
  - `Mic::try_from("XLON")` → Ok (London Stock Exchange)
  - `Mic::try_from("xnys")` → Ok (normalise)
  - `Mic::try_from("ZZZZ")` → Err (not in whitelist)
  - `Mic::try_from("XNY")` → Err (too short)
  - `mic.as_str()` → `"XNYS"`
- **GREEN**:
  - Add `data/iso10383_mic.csv`
  - `build.rs` emits `const MIC_CODES: &[&str]`
  - `Mic([u8; 4])` — fixed-size storage
  ```rust
  pub struct Mic([u8; 4]);
  impl TryFrom<&str> for Mic {
      // 1. Uppercase, exactly 4 chars, all ASCII alpha
      // 2. Binary search in MIC_CODES
  }
  ```
- **REFACTOR**: `[u8; 4]` → `&str` same pattern as `CurrencyCode` — extract shared helper
  if it reduces duplication meaningfully
- **CLEANUP**: Lint, commit

## Dependencies

- Requires: Phase 3 complete (Luhn, Mod97 algorithms)

## Status

[ ] Not Started

# Phase 6: Identifier Types

## Objective

Implement `Semver`, `Slug`, `Color`, `Locale`, and `Vin` under the `identifiers` feature flag.

## Success Criteria

- [ ] All five types implement `TryFrom<&str>` and `Display`
- [ ] `Semver` implements `Ord` / `PartialOrd` per the SemVer 2.0.0 spec
- [ ] `Color` stores as packed `u32`; display is canonical `#RRGGBB`
- [ ] Each type: valid + all rejection reasons covered in tests
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean

## TDD Cycles

### Cycle 1: `Slug`

Slug: lowercase alphanum + hyphens, no leading/trailing hyphen, min 1 char.

- **RED**: Tests for —
  - `Slug::try_from("hello-world")` → Ok
  - `Slug::try_from("a")` → Ok (single char)
  - `Slug::try_from("hello-world-123")` → Ok
  - `Slug::try_from("-hello")` → Err (leading hyphen)
  - `Slug::try_from("hello-")` → Err (trailing hyphen)
  - `Slug::try_from("Hello")` → Err (uppercase)
  - `Slug::try_from("hello--world")` → Err (consecutive hyphens)
  - `Slug::try_from("")` → Err
- **GREEN**: Hand-written validator (no regex needed — simple char iterator)
  ```rust
  pub struct Slug(String);
  ```
- **REFACTOR**: Ensure "consecutive hyphens" check is explicit and documented
- **CLEANUP**: Lint, commit

### Cycle 2: `Color`

Color: accepts `#RRGGBB` or `#RGB`; stores as packed 24-bit `u32`.

- **RED**: Tests for —
  - `Color::try_from("#FF5733")` → Ok
  - `Color::try_from("#f57")` → Ok (shorthand, expands to `#FF5577`)
  - `Color::try_from("#ff5733")` → Ok (lowercase)
  - `Color::try_from("FF5733")` → Err (missing `#`)
  - `Color::try_from("#GG5733")` → Err (invalid hex)
  - `Color::try_from("#FF573")` → Err (5 digits)
  - `color.red()` → `255u8`
  - `color.green()` → `87u8`
  - `color.blue()` → `51u8`
  - `color.to_hex()` → `"#FF5733"` (always uppercase 6-digit)
  - `color.luminance()` → `f64` in [0.0, 1.0] (WCAG 2.1 relative luminance)
  - `color.to_string()` == `"#FF5733"` (Display = canonical)
- **GREEN**:
  ```rust
  pub struct Color(u32);  // packed: 0x00RRGGBB
  impl TryFrom<&str> for Color {
      // Parse #RGB (expand each nibble) or #RRGGBB
  }
  impl Color {
      pub fn red(&self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }
      pub fn green(&self) -> u8 { ((self.0 >> 8) & 0xFF) as u8 }
      pub fn blue(&self) -> u8 { (self.0 & 0xFF) as u8 }
      pub fn to_hex(&self) -> alloc::string::String { ... }
      pub fn luminance(&self) -> f64 {
          // WCAG 2.1: linearise each channel, then 0.2126R + 0.7152G + 0.0722B
      }
  }
  ```
- **REFACTOR**: `luminance()` uses the exact WCAG 2.1 formula — add a doc comment with the
  reference
- **CLEANUP**: Lint, commit

### Cycle 3: `Locale`

BCP 47 locale tag, simplified for practical use: `language[-script][-region][-variant]*`
where language is `[a-z]{2,3}`, script is `[A-Z][a-z]{3}`, region is `[A-Z]{2}` or
`[0-9]{3}`, variant is `[0-9a-z]{5,8}` or `[0-9][0-9a-z]{3}`.

For v1: validate structure only (no IANA subtag whitelist). Document this explicitly.

- **RED**: Tests for —
  - `Locale::try_from("en")` → Ok
  - `Locale::try_from("en-US")` → Ok
  - `Locale::try_from("zh-Hant-TW")` → Ok (language + script + region)
  - `Locale::try_from("fr-FR")` → Ok
  - `Locale::try_from("sr-Latn-RS")` → Ok
  - `Locale::try_from("EN")` → Err (language must be lowercase)
  - `Locale::try_from("")` → Err
  - `locale.language()` → `"en"`
  - `locale.region()` → `Some("US")`
  - `locale.script()` → `Option<&str>`
- **GREEN**:
  ```rust
  pub struct Locale(String);
  impl TryFrom<&str> for Locale {
      // regex: ^[a-z]{2,3}(-[A-Za-z0-9]{2,8})*$  (minimal BCP 47 structure check)
  }
  impl Locale {
      pub fn language(&self) -> &str { ... }
      pub fn region(&self) -> Option<&str> { ... }
      pub fn script(&self) -> Option<&str> { ... }  // 4-char titlecase subtag
  }
  ```
- **REFACTOR**: Parsing subtags by splitting on '-' and applying heuristics (length + case)
  for script vs region vs variant is cleaner than regex with named groups
- **CLEANUP**: Lint, commit

### Cycle 4: `Semver`

SemVer 2.0.0: `MAJOR.MINOR.PATCH[-pre][+build]` where each component is a non-negative
integer (no leading zeros except `0` itself). Pre-release and build metadata have their own
grammar. Full `Ord` implementation per SemVer 2.0.0 spec (build metadata ignored in ordering).

- **RED**: Tests for —
  - `Semver::try_from("1.0.0")` → Ok
  - `Semver::try_from("1.0.0-alpha.1")` → Ok
  - `Semver::try_from("1.0.0-alpha.1+build.123")` → Ok
  - `Semver::try_from("01.0.0")` → Err (leading zero in major)
  - `Semver::try_from("1.0")` → Err (missing patch)
  - `Semver::try_from("1.0.0-")` → Err (empty pre-release)
  - `semver.major()` → `1u64`
  - `semver.minor()` → `0u64`
  - `semver.patch()` → `0u64`
  - `semver.pre_release()` → `Some("alpha.1")`
  - `semver.build_metadata()` → `Some("build.123")`
  - `semver.compatible_with(base)` — same major.minor, patch ≥ base
  - `semver.caret_compatible(base)` — same major, minor.patch ≥ base
  - Ordering: `1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-beta < 1.0.0`
  - `1.0.0+build.1 == 1.0.0+build.2` (build metadata ignored)
- **GREEN**:
  Hand-written parser (split on '.', '-', '+' with care for ordering of separators).
  ```rust
  pub struct Semver {
      major: u64,
      minor: u64,
      patch: u64,
      pre_release: Option<String>,
      build_metadata: Option<String>,
  }
  impl Ord for Semver { /* SemVer 2.0.0 precedence rules */ }
  impl Display for Semver { /* reconstruct canonical string */ }
  ```
- **REFACTOR**: Pre-release comparison is the complex part — extract
  `compare_pre_release(a: &str, b: &str) -> Ordering` as a private fn with its own tests
- **CLEANUP**: Lint, commit

### Cycle 5: `Vin`

VIN (ISO 3779 / FMVSS 115): exactly 17 chars, `[A-HJ-NPR-Z0-9]{17}` (no I, O, Q).
Check digit (position 9): weighted sum mod 11 (0–9 or 'X').

- **RED**: Tests for —
  - `Vin::try_from("1HGBH41JXMN109186")` → Ok
  - `Vin::try_from("1hgbh41jxmn109186")` → Ok (normalise to uppercase)
  - `Vin::try_from("1HGBH41JOMN109186")` → Err ('O' is forbidden)
  - `Vin::try_from("1HGBH41JXMN10918")` → Err (too short)
  - `Vin::try_from("1HGBH41JXMN1091860")` → Err (too long)
  - `Vin::try_from("1HGBH41JYMN109186")` → Err (bad check digit — 'Y' at pos 9 ≠ 'X')
  - `vin.wmi()` → `"1HG"` (World Manufacturer Identifier, chars 0–2)
  - `vin.vds()` → `"BH41JX"` (Vehicle Descriptor Section, chars 3–8)
  - `vin.vis()` → `"MN109186"` (Vehicle Identifier Section, chars 9–16 per FMVSS;
    note: position 9 is check digit, position 10 is model year)
  - `vin.check_digit()` → `'X'`
  - `vin.model_year_char()` → `'M'` (char at position 9 in the VIS = position 10 overall;
    note: careful about FMVSS vs ISO indexing)
- **GREEN**:
  VIN check digit: chars map to values (1→1 … 9→9, A→1, B→2 … I excluded, etc.);
  weights = [8,7,6,5,4,3,2,10,0,9,8,7,6,5,4,3,2]; sum mod 11; check digit at position 8.
  ```rust
  pub struct Vin([u8; 17]);  // stored as uppercase ASCII
  ```
- **REFACTOR**: Extract character value and weight tables as `const` arrays
- **CLEANUP**: Lint, commit

## Dependencies

- Requires: Phase 1 complete
- `Semver` does not require regex-lite (hand-written parser)

## Status

[x] Complete

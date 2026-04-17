# Phase 7: Geographic & Locale Types

## Objective

Implement `CountryCode`, `CountryCodeAlpha3`, `LanguageCode`, `PostalCode`, `Latitude`,
`Longitude`, `IataAirportCode`, `IcaoAirportCode`, and `IanaTimezone` under the `geographic`
feature flag. All whitelist types use `build.rs` CSV generation established in Phase 1.

## Success Criteria

- [ ] All nine types implement `TryFrom<&str>` and `Display`
- [ ] All whitelist types generated from authoritative sources
- [ ] `PostalCode` validates at least 10 country patterns (see list below)
- [ ] `Latitude` and `Longitude` enforce range bounds
- [ ] Each type: valid + all rejection reasons covered in tests
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean

## Data sources

| Type | Source file | Column |
|------|------------|--------|
| `CountryCode` | `data/iso3166_countries.csv` | `alpha2` |
| `CountryCodeAlpha3` | `data/iso3166_countries.csv` | `alpha3` |
| `LanguageCode` | `data/iana_language_subtags.txt` | `Subtag` (Type: language) |
| `IataAirportCode` | `data/ourairports_airports.csv` | `iata_code` (non-empty rows) |
| `IcaoAirportCode` | `data/ourairports_airports.csv` | `icao_code` (non-empty rows) |
| `IanaTimezone` | `data/iana_zone_tab.txt` | col 3 (TZ name) |

All files committed to `data/`. OurAirports data is CC0/public domain.

## TDD Cycles

### Cycle 1: `CountryCode` and `CountryCodeAlpha3`

- **RED**: Tests for —
  - `CountryCode::try_from("FR")` → Ok
  - `CountryCode::try_from("fr")` → Ok (normalise to uppercase)
  - `CountryCode::try_from("XX")` → Err (not in ISO 3166-1)
  - `CountryCode::try_from("FRA")` → Err (too long)
  - `CountryCode::try_from("")` → Err
  - `CountryCodeAlpha3::try_from("FRA")` → Ok
  - `CountryCodeAlpha3::try_from("fra")` → Ok
  - `CountryCodeAlpha3::try_from("XXX")` → Err (not in ISO 3166-1 alpha-3)
  - Consistency: `CountryCode::try_from("FR")` and `CountryCodeAlpha3::try_from("FRA")` both
    represent the same country
- **GREEN**:
  - `build.rs` reads `data/iso3166_countries.csv`, emits two sorted arrays:
    `COUNTRY_CODES_ALPHA2: &[&str]` and `COUNTRY_CODES_ALPHA3: &[&str]`
  - `CountryCode([u8; 2])` — fixed-size
  - `CountryCodeAlpha3([u8; 3])` — fixed-size
- **REFACTOR**: Confirm the `[u8; N]` → `&str` helper is consistent with `CurrencyCode` and
  `Mic` from earlier phases
- **CLEANUP**: Lint, commit

### Cycle 2: `LanguageCode`

IANA primary language subtags (ISO 639-1 and -2/3). Validate structure + whitelist.
Format: 2–3 lowercase alpha chars for primary tag; extended subtags validated structurally only.

- **RED**: Tests for —
  - `LanguageCode::try_from("en")` → Ok
  - `LanguageCode::try_from("fra")` → Ok (ISO 639-2)
  - `LanguageCode::try_from("EN")` → Ok (normalise to lowercase)
  - `LanguageCode::try_from("xx")` → Err (not in IANA registry)
  - `LanguageCode::try_from("e")` → Err (too short)
  - `lang.primary()` → `"en"`
- **GREEN**:
  - `build.rs` parses `data/iana_language_subtags.txt` (IANA registry plain text format),
    extracts `Type: language` entries, emits `LANGUAGE_CODES: &[&str]` (sorted, lowercase)
  - `LanguageCode(String)` — variable length (2–8 chars for BCP 47 primary subtags)
- **REFACTOR**: Confirm the parse handles IANA registry continuation lines correctly
- **CLEANUP**: Lint, commit

### Cycle 3: `Latitude` and `Longitude`

Trivial range guards; no regex.

- **RED**: Tests for —
  - `Latitude::try_from(45.0_f64)` → Ok
  - `Latitude::try_from(-90.0_f64)` → Ok (boundary)
  - `Latitude::try_from(90.0_f64)` → Ok (boundary)
  - `Latitude::try_from(90.1_f64)` → Err
  - `Latitude::try_from(f64::NAN)` → Err
  - Same pattern for `Longitude` with range [-180.0, 180.0]
  - Also test `TryFrom<&str>` path (parses the float then validates)
- **GREEN**:
  ```rust
  pub struct Latitude(f64);
  pub struct Longitude(f64);
  impl TryFrom<f64> for Latitude { ... }
  impl TryFrom<&str> for Latitude { /* parse then delegate to TryFrom<f64> */ }
  impl Display for Latitude { /* f64 with enough precision */ }
  ```
- **REFACTOR**: Decide on display precision (6 decimal places = ~11cm precision — document)
- **CLEANUP**: Lint, commit

### Cycle 4: `PostalCode`

Input format: `CC:CODE` (e.g. `FR:75001`, `GB:SW1A1AA`, `US:10001`).
Validation dispatches by country code to a country-specific pattern.

v1 minimum coverage (10 countries):

| Country | Pattern | Example |
|---------|---------|---------|
| `US` | `^\d{5}(-\d{4})?$` | `10001`, `10001-1234` |
| `GB` | `^[A-Z]{1,2}\d[A-Z\d]?\s?\d[A-Z]{2}$` | `SW1A 1AA`, `SW1A1AA` |
| `FR` | `^\d{5}$` | `75001` |
| `DE` | `^\d{5}$` | `10115` |
| `CA` | `^[A-Z]\d[A-Z]\s?\d[A-Z]\d$` | `K1A 0A6`, `K1A0A6` |
| `AU` | `^\d{4}$` | `2000` |
| `NL` | `^\d{4}\s?[A-Z]{2}$` | `1234 AB`, `1234AB` |
| `JP` | `^\d{3}-?\d{4}$` | `100-0001`, `1000001` |
| `CH` | `^\d{4}$` | `8001` |
| `ES` | `^\d{5}$` | `28001` |

Unknown countries: accept any non-empty CODE (warn in docs that no country-specific check
is applied).

- **RED**: Tests for —
  - `PostalCode::try_from("FR:75001")` → Ok
  - `PostalCode::try_from("GB:SW1A1AA")` → Ok
  - `PostalCode::try_from("GB:SW1A 1AA")` → Ok (spaces normalised)
  - `PostalCode::try_from("US:1000")` → Err (US zip too short)
  - `PostalCode::try_from("INVALID")` → Err (no ':' separator)
  - `PostalCode::try_from("ZZ:12345")` → Ok (unknown country, accepted)
  - `postal.country()` → `"FR"`
  - `postal.code()` → `"75001"`
  - `postal.belongs_to_country("FR")` → true
- **GREEN**:
  ```rust
  pub struct PostalCode { country: [u8; 2], code: String }
  impl TryFrom<&str> for PostalCode {
      // 1. Split on first ':'
      // 2. Validate country: 2 uppercase alpha
      // 3. Dispatch to per-country validator or accept-all fallback
  }
  ```
- **REFACTOR**: Per-country validators as a `match` on country code with compiled
  `once_cell::Lazy<Regex>` per pattern
- **CLEANUP**: Lint, commit

### Cycle 5: `IataAirportCode`

IATA airport code: exactly 3 uppercase alpha chars, whitelisted. ~10,000 active codes.

- **RED**: Tests for —
  - `IataAirportCode::try_from("CDG")` → Ok (Paris Charles de Gaulle)
  - `IataAirportCode::try_from("JFK")` → Ok
  - `IataAirportCode::try_from("cdg")` → Ok (normalise)
  - `IataAirportCode::try_from("ZZZ")` → Err (not in whitelist)
  - `IataAirportCode::try_from("CD")` → Err (too short)
- **GREEN**:
  - `build.rs` reads `data/ourairports_airports.csv`, extracts non-empty `iata_code` values,
    emits `IATA_AIRPORT_CODES: &[&str]` (sorted)
  - `IataAirportCode([u8; 3])` — fixed-size
- **REFACTOR**: —
- **CLEANUP**: Lint, commit

### Cycle 6: `IcaoAirportCode`

ICAO airport code: exactly 4 uppercase alphanumeric chars, whitelisted. ~40,000 codes.

- **RED**: Tests for —
  - `IcaoAirportCode::try_from("LFPG")` → Ok (Paris CDG)
  - `IcaoAirportCode::try_from("KJFK")` → Ok
  - `IcaoAirportCode::try_from("lfpg")` → Ok (normalise)
  - `IcaoAirportCode::try_from("ZZZZ")` → Err
  - `IcaoAirportCode::try_from("LFP")` → Err (too short)
- **GREEN**:
  - Same CSV as IATA, `icao_code` column
  - `IcaoAirportCode([u8; 4])` — fixed-size
- **REFACTOR**: Note that the generated array for ICAO is ~40k entries; verify compile time
  is acceptable before proceeding
- **CLEANUP**: Lint, commit

### Cycle 7: `IanaTimezone`

IANA timezone name: validated against zone.tab. ~600 entries. Examples: `Europe/Paris`,
`America/New_York`, `UTC`.

- **RED**: Tests for —
  - `IanaTimezone::try_from("Europe/Paris")` → Ok
  - `IanaTimezone::try_from("America/New_York")` → Ok
  - `IanaTimezone::try_from("UTC")` → Ok
  - `IanaTimezone::try_from("Europe/Atlantis")` → Err (not in IANA db)
  - `IanaTimezone::try_from("")` → Err
  - `tz.to_string()` → `"Europe/Paris"` (Display = input as-is)
- **GREEN**:
  - `build.rs` parses `data/iana_zone_tab.txt` (3rd column), emits
    `IANA_TIMEZONES: &[&str]` (sorted)
  - `IanaTimezone(String)` — variable length
- **REFACTOR**: —
- **CLEANUP**: Lint, commit

## Dependencies

- Requires: Phase 1 complete (build.rs infrastructure)
- Requires: Phase 3 complete (CountryCode structure consistent with CurrencyCode/Mic)

## Status

[x] Cycles 1-3 Complete (CountryCode, CountryCodeAlpha3, LanguageCode, Latitude, Longitude)

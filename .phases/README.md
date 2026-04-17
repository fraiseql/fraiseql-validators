# fraiseql-validators — Phase Overview

## Vision

A single `no_std`-compatible Rust validation library that enforces semantic type rules
at every layer of the fraiseql stack (GraphQL scalars and pg_scalars PostgreSQL extension).

## Type inventory: 41 types across 6 feature flags

| Feature flag           | Count | Types |
|------------------------|-------|-------|
| `contact`              | 3     | `Email`, `PhoneE164`, `DomainName` |
| `financial_banking`    | 4     | `Iban`, `Isin`, `CurrencyCode`, `SwiftBic` |
| `financial_securities` | 5     | `Cusip`, `Sedol`, `Lei`, `Figi`, `Mic` |
| `barcodes`             | 6     | `Ean8`, `Ean13`, `UpcA`, `Isbn13`, `Issn`, `Gtin14` |
| `identifiers`          | 5     | `Semver`, `Slug`, `Color`, `Locale`, `Vin` |
| `geographic`           | 9     | `CountryCode`, `CountryCodeAlpha3`, `LanguageCode`, `PostalCode`, `Latitude`, `Longitude`, `IataAirportCode`, `IcaoAirportCode`, `IanaTimezone` |
| `network`              | 6     | `Port`, `MacAddressEui48`, `MacAddressEui64`, `Ipv4Address`, `Ipv6Address`, `Asn` |

Default features: all flags enabled. Consumers may opt out of any feature.

## Architecture decisions

- **`no_std` + `alloc`**: the crate compiles without std; pgrx and WASM consumers are unblocked.
- **`regex-lite`**: drop-in near-subset of `regex`, works in `no_std` + `alloc`. All validation
  patterns use only ASCII character classes, quantifiers, non-capturing groups, and alternation —
  no Unicode properties or lookaheads required.
- **`once_cell`**: `Lazy<Regex>` for compiled patterns under std; `no_std` path uses manual
  `OnceLock` equivalents.
- **Whitelist types via `build.rs`**: authoritative CSVs committed to `data/` are processed at
  compile time into sorted `const` arrays. Binary search at runtime — zero extra dependencies,
  fully `no_std`.
- **Checksum algorithms** (Luhn, Mod97): ported from fraiseql-core, pure arithmetic, no deps.
- **No regex for checksum-primary types**: `Iban`, `Isin`, `Cusip`, `Sedol`, `Ean*`, `Isbn*`,
  `Issn`, `Gtin14`, `Figi` — format is a simple structural check; the real work is the checksum.

## Sources for authoritative data

| Type(s) | Source | Format |
|---------|--------|--------|
| `CurrencyCode` | ISO 4217 — iso.org maintenance agency | CSV |
| `CountryCode`, `CountryCodeAlpha3` | ISO 3166-1 — iso.org | CSV |
| `LanguageCode` | IANA language subtag registry | plain text (tab-separated) |
| `IataAirportCode` | OurAirports airports.csv (open data) | CSV |
| `IcaoAirportCode` | OurAirports airports.csv (same file, icao_code column) | CSV |
| `IanaTimezone` | IANA tz database zone.tab | plain text |
| `Mic` | ISO 10383 — iso20022.org/market-identifier-codes | CSV |

## Phase summary

| Phase | Title | Status |
|-------|-------|--------|
| 01 | Foundation | [ ] Not Started |
| 02 | Contact types | [ ] Not Started |
| 03 | Financial — banking | [ ] Not Started |
| 04 | Financial — securities | [ ] Not Started |
| 05 | Barcodes & product IDs | [ ] Not Started |
| 06 | Identifiers | [ ] Not Started |
| 07 | Geographic & locale | [x] Complete |
| 08 | Networking | [ ] Not Started |
| 09 | Finalize | [ ] Not Started |

# Product Requirements Document — fraiseql-validators

## Vision

A single Rust validation library that enforces the same semantic type rules at every layer of the stack:
- **Application layer** (fraiseql GraphQL scalars)
- **Storage layer** (pg_scalars PostgreSQL extension)

No more duplicated regexes. No more validation that passes at the API but fails on a direct SQL insert.

---

## Problem Statement

### Current state

fraiseql defines ~51 "rich scalar" types (Email, IBAN, PhoneNumber, Semver, etc.) with validation logic living exclusively in the GraphQL layer. The database stores all of them as `TEXT`. This means:

1. Any client that bypasses fraiseql (migrations, psql sessions, background jobs, other services) can insert invalid data with no error.
2. SQL queries cannot use type-specific operators (`email_domain()`, semver ordering, etc.) — every extraction is ad-hoc string manipulation.
3. The PostgreSQL ecosystem offers no maintained alternative: the existing per-type C extensions (pgemailaddr, grzm/e164, yorickdewid/PostgreSQL-IBAN) are all abandoned, alpha-quality, or operator-less.

### Desired state

```
fraiseql-validators  (Rust crate, no_std compatible)
       ├── fraiseql              uses it for GraphQL scalar parse/serialize
       └── pg_scalars            uses it, exposes validated types + operators to PostgreSQL via pgrx
```

One crate owns the validation rules. Two consumers compile it in. The rules are identical at both layers by construction.

---

## Scope

### In scope

**fraiseql-validators** — a `no_std`-compatible Rust crate that provides:
- Validated newtype wrappers for each scalar type
- Parse (from `&str`) and serialize (to `&str`) for each type
- Type-specific accessor functions (e.g., `email.domain()`, `iban.country()`)
- Zero runtime dependencies beyond `regex` (or `once_cell` for compiled patterns)

**pg_scalars** — a pgrx PostgreSQL extension that:
- Wraps each fraiseql-validators type as a native PostgreSQL type
- Provides input/output functions, casts to/from `text`
- Implements btree + hash operator classes for all equality/orderable types
- Exposes type-specific SQL functions as operators

The pg_scalars extension is a separate repository/crate that depends on fraiseql-validators. It is out of scope for this document but its requirements drive the API design of fraiseql-validators.

### Out of scope

- Full postal addresses (inherently relational, country-specific formatting; individual components like `postal_code`, `latitude`, `longitude` are in scope)
- Types already well-served by PostgreSQL core: `inet`, `cidr`, `macaddr`, `uuid`, `money`
- Types already covered by pg_treekey: `ltree`
- URL, Markdown, HTML, MIME type, File, Image (too variable, not meaningfully queryable as distinct types)
- libphonenumber integration (heavy C++ dependency, incompatible with pgrx build model)

---

## Type Inventory

### Contact

| Type | Internal storage | Validation rule | Key accessors |
|------|-----------------|-----------------|---------------|
| `Email` | `String` | RFC 5321 practical subset: `local@domain`, domain must have 2+ labels, max 254 chars | `local()`, `domain()` |
| `PhoneE164` | `String` | `+[1-3 digit country code][6-14 digits]`, max 20 chars | `country_code()`, `national_number()` |
| `DomainName` | `String` | Valid DNS hostname: labels 1–63 chars, alphanum + hyphen, no leading/trailing hyphen, no scheme/path | — |

### Financial

| Type | Internal storage | Validation rule | Key accessors |
|------|-----------------|-----------------|---------------|
| `Iban` | `String` | 15–34 alphanumeric chars, mod-97 checksum (ISO 13616) | `country()`, `bban()` |
| `Isin` | `String` | 12 chars: 2-letter country code + 9 alphanum + Luhn check digit (ISO 6166) | `country()`, `nsin()` |
| `CurrencyCode` | `[u8; 3]` | ISO 4217 alpha-3 whitelist (hardcoded, ~180 codes) | — |

### Identifiers

| Type | Internal storage | Validation rule | Key accessors |
|------|-----------------|-----------------|---------------|
| `Semver` | `String` | SemVer 2.0.0: `MAJOR.MINOR.PATCH[-pre][+build]` | `major()`, `minor()`, `patch()`, `pre_release()`, `build_metadata()` |
| `Slug` | `String` | Lowercase alphanum + hyphens only, no leading/trailing hyphen, min 1 char | — |
| `Color` | `u32` | Hex `#RRGGBB` or `#RGB` on input, stored as packed 24-bit int | `red()`, `green()`, `blue()`, `to_hex()`, `luminance()` |

### Geographic / Locale

| Type | Internal storage | Validation rule | Key accessors |
|------|-----------------|-----------------|---------------|
| `CountryCode` | `[u8; 2]` | ISO 3166-1 alpha-2 whitelist (hardcoded, 249 codes) | — |
| `LanguageCode` | `String` | BCP 47 / ISO 639-1: `[a-z]{2,3}(-[A-Za-z0-9]{2,8})*` | `primary()`, `subtag()` |
| `PostalCode` | `String` | Input: `CC:CODE` (e.g. `FR:75001`, `GB:SW1A1AA`). Validation dispatches by country code. | `country()`, `code()` |
| `Latitude` | `f64` | Range `[-90.0, 90.0]` | — |
| `Longitude` | `f64` | Range `[-180.0, 180.0]` | — |

### Networking

| Type | Internal storage | Validation rule | Key accessors |
|------|-----------------|-----------------|---------------|
| `Port` | `u16` | Range `[0, 65535]` (type system sufficient, no regex needed) | — |
| `MacAddressEui64` | `[u8; 8]` | 8 octets, colon- or hyphen-separated hex on input | `to_canonical()` |

---

## Operator Inventory (for pg_scalars)

These are the SQL-level operators and functions that pg_scalars will expose. They drive the accessor API in fraiseql-validators.

### `email`

| SQL | fraiseql-validators | Description |
|-----|-------------------|-------------|
| `email_local(e)` | `Email::local()` | Local part (`user@example.com → user`) |
| `email_domain(e)` | `Email::domain()` | Domain part (`user@example.com → example.com`) |
| `e @> 'example.com'` | `Email::belongs_to_domain(d)` | Email belongs to domain |
| `=` (case-insensitive) | `PartialEq` | RFC 5321 case-insensitive equality |

### `phone_e164`

| SQL | fraiseql-validators | Description |
|-----|-------------------|-------------|
| `phone_country_code(p)` | `PhoneE164::country_code()` | Country code digits (`+33...` → `33`) |
| `phone_national(p)` | `PhoneE164::national_number()` | Subscriber number without `+` and country code |

### `iban`

| SQL | fraiseql-validators | Description |
|-----|-------------------|-------------|
| `iban_country(i)` | `Iban::country()` | ISO country code (`GB29NWBK... → GB`) |
| `iban_bban(i)` | `Iban::bban()` | Basic Bank Account Number portion |

### `isin`

| SQL | fraiseql-validators | Description |
|-----|-------------------|-------------|
| `isin_country(i)` | `Isin::country()` | Country code (`US0231351067 → US`) |
| `isin_nsin(i)` | `Isin::nsin()` | National Securities Identifying Number portion |

### `semver`

| SQL | fraiseql-validators | Description |
|-----|-------------------|-------------|
| `<`, `<=`, `=`, `>=`, `>`, `<>` | `Ord` / `PartialOrd` | Semantic ordering per SemVer 2.0.0 spec |
| `semver_major(v)` | `Semver::major()` | Major component |
| `semver_minor(v)` | `Semver::minor()` | Minor component |
| `semver_patch(v)` | `Semver::patch()` | Patch component |
| `v ~> base` | `Semver::compatible_with(base)` | Same major.minor, patch ≥ base |
| `v ^> base` | `Semver::caret_compatible(base)` | Same major, minor.patch ≥ base |

### `color`

| SQL | fraiseql-validators | Description |
|-----|-------------------|-------------|
| `color_red(c)` | `Color::red()` | Red channel `[0, 255]` |
| `color_green(c)` | `Color::green()` | Green channel `[0, 255]` |
| `color_blue(c)` | `Color::blue()` | Blue channel `[0, 255]` |
| `color_to_hex(c)` | `Color::to_hex()` | Render as `#RRGGBB` |
| `color_luminance(c)` | `Color::luminance()` | WCAG 2.1 relative luminance `[0.0, 1.0]` |

### `postal_code`

| SQL | fraiseql-validators | Description |
|-----|-------------------|-------------|
| `postal_code_country(p)` | `PostalCode::country()` | Country code (`FR:75001 → FR`) |
| `postal_code_code(p)` | `PostalCode::code()` | Code portion (`FR:75001 → 75001`) |
| `p @> 'FR'` | `PostalCode::belongs_to_country(cc)` | Postal code belongs to country |

---

## API Design

### Parse / serialize contract

Every type implements:

```rust
impl TryFrom<&str> for T {
    type Error = ValidationError;
    fn try_from(s: &str) -> Result<Self, Self::Error>;
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    // Canonical serialization, always round-trips: T::try_from(&t.to_string()) == Ok(t)
}
```

### Error type

```rust
pub struct ValidationError {
    pub type_name: &'static str,   // e.g. "Email"
    pub input: String,             // the rejected input
    pub reason: &'static str,      // human-readable reason, no dynamic allocation
}
```

### no_std compatibility

The crate must compile with `#![no_std]` + `alloc`. This ensures:
- pgrx can link it without pulling in `std` panic machinery unexpectedly
- Embedded/WASM consumers are not blocked

Regex patterns are compiled once via `once_cell::sync::Lazy` (or `std::sync::OnceLock` under std).

---

## Non-functional Requirements

| Requirement | Target |
|---|---|
| Zero heap allocation in the hot path for fixed-size types (`Color`, `Port`, `CountryCode`, `CurrencyCode`, `MacAddressEui64`) | Enforced by internal `[u8; N]` / `u32` / `u16` storage |
| No panics in parse path | All parse functions return `Result`, never `unwrap` |
| Deterministic serialization | `Display` output is canonical and stable across versions |
| Compile time for `cargo check` | Under 5 seconds on a modern laptop |
| Test coverage | All valid and invalid examples from the relevant RFC/ISO spec covered |
| No runtime dependencies beyond `regex` + `once_cell` | Enforced in `Cargo.toml` |

---

## Integration Contracts

### fraiseql integration

fraiseql's `CustomScalar` trait currently has three methods: `serialize()`, `parse_value()`, `parse_literal()`. The integration is a thin adapter:

```rust
// In fraiseql, not in fraiseql-validators
impl CustomScalar for EmailScalar {
    fn parse_value(v: &Value) -> Result<Self, ScalarError> {
        let s = v.as_str()?;
        Email::try_from(s).map(EmailScalar).map_err(Into::into)
    }
    fn serialize(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
```

fraiseql adds `fraiseql-validators` as a dependency. fraiseql-validators has no dependency on fraiseql.

### pg_scalars integration

pg_scalars wraps each type via pgrx. The pgrx input function delegates to `TryFrom<&str>`:

```rust
// In pg_scalars, not in fraiseql-validators
#[pg_extern]
fn email_in(input: &str) -> Email {
    Email::try_from(input).unwrap_or_else(|e| {
        error!("{}", e)  // pgrx error! maps to PostgreSQL ERROR
    })
}
```

pg_scalars adds `fraiseql-validators` as a dependency. fraiseql-validators has no dependency on pgrx.

---

## Delivery Phasing

This document covers fraiseql-validators only. pg_scalars is a follow-on project.

**Phase 1 — Foundation**
Core infrastructure: `ValidationError`, `TryFrom`/`Display` contract, crate skeleton, CI.

**Phase 2 — Contact types**
`Email`, `PhoneE164`, `DomainName`

**Phase 3 — Financial types**
`Iban`, `Isin`, `CurrencyCode`

**Phase 4 — Identifier types**
`Semver`, `Slug`, `Color`

**Phase 5 — Geographic / locale types**
`CountryCode`, `LanguageCode`, `PostalCode`, `Latitude`, `Longitude`

**Phase 6 — Networking types**
`Port`, `MacAddressEui64`

**Phase 7 — Finalize**
API review, documentation, publish to crates.io.

---

## Success Criteria

- `cargo test` passes with zero failures
- `cargo clippy --all-targets --all-features -- -D warnings` passes clean
- Every type: one test for a valid input, one for each documented rejection reason
- `fraiseql` compiles with fraiseql-validators replacing its inline validation logic for at least `Email` and `PhoneE164` (integration smoke test)
- Published to crates.io as `fraiseql-validators`

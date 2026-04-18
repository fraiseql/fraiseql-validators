# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-04-18

### Added

- `fraiseql` feature flag with optional `fraiseql-error` dependency
- `ValidationError::into_fraiseql_error()` for conversion to `FraiseQLError::Validation`
- `ValidationError::into_field_error(field)` for conversion to `ValidationFieldError`

## [0.1.0] - 2026-04-17

### Added

- **contact**: `Email`, `PhoneE164`, `DomainName` validators
- **financial_banking**: `Iban`, `Isin`, `CurrencyCode` (ISO 4217), `SwiftBic` validators
- **financial_securities**: `Cusip`, `Sedol`, `Lei`, `Figi`, `Mic` validators
- **barcodes**: `Ean8`, `Ean13`, `UpcA`, `Isbn13`, `Issn`, `Gtin14` validators with GS1/Luhn check digits
- **identifiers**: `Semver`, `Slug`, `Color`, `Locale`, `Vin` validators
- **geographic**: `CountryCode` (ISO 3166-1 alpha-2/alpha-3), `LanguageCode`, `PostalCode`, `Latitude`, `Longitude`, `IataAirportCode`, `IcaoAirportCode`, `IanaTimezone` validators
- **network**: `Port`, `MacAddressEui48`, `MacAddressEui64`, `Ipv4Address`, `Ipv6Address`, `Asn` validators
- `no_std` + `alloc` compatible across all modules
- Compile-time whitelist generation from authoritative CSV/text data via `build.rs`
- Checksum algorithms: Luhn, MOD-97, GS1, ISIN numeric expansion
- `ValidationError` with type name, input, and reason fields

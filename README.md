# fraiseql-validators

A `no_std`-compatible Rust library providing type-safe validated wrappers for common identifiers, codes, and formats.

Each type enforces its specification at construction time via `TryFrom<&str>`, ensuring invalid values never exist in your program.

## Installation

```toml
[dependencies]
fraiseql-validators = "0.1"
```

All feature flags are enabled by default. To select only what you need:

```toml
[dependencies]
fraiseql-validators = { version = "0.2", default-features = false, features = ["contact", "barcodes"] }
```

## Feature flags

| Feature                | Types                                                                                                     |
|------------------------|-----------------------------------------------------------------------------------------------------------|
| `contact`              | `Email`, `PhoneE164`, `DomainName`                                                                        |
| `financial_banking`    | `Iban`, `Isin`, `CurrencyCode`, `SwiftBic`                                                                |
| `financial_securities` | `Cusip`, `Sedol`, `Lei`, `Figi`, `Mic`                                                                    |
| `barcodes`             | `Ean8`, `Ean13`, `UpcA`, `Isbn13`, `Issn`, `Gtin14`                                                      |
| `identifiers`          | `Semver`, `Slug`, `Color`, `Locale`, `Vin`                                                                |
| `geographic`           | `CountryCode`, `CountryCodeAlpha3`, `LanguageCode`, `PostalCode`, `Latitude`, `Longitude`, `IataAirportCode`, `IcaoAirportCode`, `IanaTimezone` |
| `network`              | `Port`, `MacAddressEui48`, `MacAddressEui64`, `Ipv4Address`, `Ipv6Address`, `Asn`                         |
| `fraiseql`             | `ValidationError` conversion to `FraiseQLError` and `ValidationFieldError`                                |

## Usage

Every type follows the same pattern: construct via `TryFrom<&str>`, access fields via methods, display via `Display`.

### Contact

```rust
use fraiseql_validators::contact::{Email, PhoneE164, DomainName};

let email = Email::try_from("user@example.com").unwrap();
assert_eq!(email.local(), "user");
assert_eq!(email.domain(), "example.com");

let phone = PhoneE164::try_from("+33123456789").unwrap();
assert_eq!(phone.country_code(), "33");

let domain = DomainName::try_from("example.com").unwrap();
```

### Financial banking

```rust
use fraiseql_validators::financial_banking::{Iban, CurrencyCode, SwiftBic};

let iban = Iban::try_from("DE89370400440532013000").unwrap();
assert_eq!(iban.country(), "DE");

let currency = CurrencyCode::try_from("EUR").unwrap();
let bic = SwiftBic::try_from("DEUTDEFF").unwrap();
```

### Financial securities

```rust
use fraiseql_validators::financial_securities::{Cusip, Sedol, Lei, Figi, Mic};

let cusip = Cusip::try_from("037833100").unwrap();
assert_eq!(cusip.issuer_code(), "037833");

let mic = Mic::try_from("XNYS").unwrap();
```

### Barcodes

```rust
use fraiseql_validators::barcodes::{Ean13, Isbn13, UpcA};

let ean = Ean13::try_from("5901234123457").unwrap();
assert_eq!(ean.gs1_prefix(), "590");

let isbn = Isbn13::try_from("9780306406157").unwrap();
let upc = UpcA::try_from("012345678905").unwrap();
```

### Identifiers

```rust
use fraiseql_validators::identifiers::{Semver, Slug, Color, Locale, Vin};

let version = Semver::try_from("1.2.3-alpha.1").unwrap();
assert_eq!(version.major(), 1);

let slug = Slug::try_from("hello-world").unwrap();
let color = Color::try_from("#FF5733").unwrap();
assert_eq!(color.red(), 255);

let locale = Locale::try_from("en-US").unwrap();
assert_eq!(locale.language(), "en");
```

### Geographic

```rust
use fraiseql_validators::geographic::{CountryCode, Latitude, Longitude, IanaTimezone};

let country = CountryCode::try_from("FR").unwrap();
let lat = Latitude::try_from(48.8566).unwrap();
let lon = Longitude::try_from(2.3522).unwrap();
let tz = IanaTimezone::try_from("Europe/Paris").unwrap();
```

### Network

```rust
use fraiseql_validators::network::{Ipv4Address, Ipv6Address, Port, MacAddressEui48};

let ip = Ipv4Address::try_from("192.168.1.1").unwrap();
assert!(ip.is_private());

let port = Port::try_from("443").unwrap();
assert_eq!(port.value(), 443);

let mac = MacAddressEui48::try_from("00:1A:2B:3C:4D:5E").unwrap();
assert_eq!(mac.to_canonical(), "00:1a:2b:3c:4d:5e");
```

## Error handling

All types return `ValidationError` on failure:

```rust
use fraiseql_validators::contact::Email;

let result = Email::try_from("invalid");
assert!(result.is_err());

let err = result.unwrap_err();
assert_eq!(err.type_name, "Email");
assert_eq!(err.input, "invalid");
// err.reason describes what went wrong
```

## FraiseQL integration

Enable the `fraiseql` feature to get direct error conversion for GraphQL responses:

```toml
[dependencies]
fraiseql-validators = { version = "0.2", features = ["fraiseql", "contact"] }
```

```rust
use fraiseql_validators::contact::Email;

match Email::try_from(user_input) {
    Ok(email) => { /* use validated email */ }
    Err(e) => {
        // Convert to a ValidationFieldError for GraphQL error responses
        let field_err = e.into_field_error("user.email");

        // Or convert to a FraiseQLError directly
        // let err = e.into_fraiseql_error();
    }
}
```

## `no_std` support

This crate is `no_std` compatible with `alloc`. It works in embedded and WASM environments.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

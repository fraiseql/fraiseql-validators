use fraiseql_validators::financial_banking::{CurrencyCode, Iban, Isin, SwiftBic};

#[test]
fn test_iban_try_from_valid_gb() {
    let iban = Iban::try_from("GB82WEST12345698765432").unwrap();
    assert_eq!(iban.to_string(), "GB82WEST12345698765432");
}

#[test]
fn test_iban_try_from_valid_de() {
    let iban = Iban::try_from("DE89370400440532013000").unwrap();
    assert_eq!(iban.to_string(), "DE89370400440532013000");
}

#[test]
fn test_iban_try_from_lowercase() {
    let iban = Iban::try_from("gb82west12345698765432").unwrap();
    assert_eq!(iban.to_string(), "GB82WEST12345698765432");
}

#[test]
fn test_iban_try_from_empty() {
    let result = Iban::try_from("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "empty string");
}

#[test]
fn test_iban_try_from_invalid_checksum() {
    let result = Iban::try_from("GB82WEST1234569876543X");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "checksum validation failed");
}

#[test]
fn test_iban_try_from_invalid_check_digits() {
    let result = Iban::try_from("GBXXWEST12345698765432");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "check digits must be two digits");
}

#[test]
fn test_iban_accessors() {
    let iban = Iban::try_from("GB82WEST12345698765432").unwrap();
    assert_eq!(iban.country(), "GB");
    assert_eq!(iban.check_digits(), "82");
    assert_eq!(iban.bban(), "WEST12345698765432");
}

#[test]
fn test_isin_try_from_valid_us() {
    let isin = Isin::try_from("US0231351067").unwrap();
    assert_eq!(isin.to_string(), "US0231351067");
}

#[test]
fn test_isin_try_from_valid_gb() {
    let isin = Isin::try_from("GB0002634946").unwrap();
    assert_eq!(isin.to_string(), "GB0002634946");
}

#[test]
fn test_isin_try_from_lowercase() {
    let isin = Isin::try_from("us0231351067").unwrap();
    assert_eq!(isin.to_string(), "US0231351067");
}

#[test]
fn test_isin_try_from_invalid_checksum() {
    let result = Isin::try_from("US0231351068");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "Luhn checksum validation failed");
}

#[test]
fn test_isin_try_from_too_long() {
    let result = Isin::try_from("US02313510678");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "ISIN must be exactly 12 characters");
}

#[test]
fn test_isin_accessors() {
    let isin = Isin::try_from("US0231351067").unwrap();
    assert_eq!(isin.country(), "US");
    assert_eq!(isin.nsin(), "023135106");
    assert_eq!(isin.check_digit(), '7');
}

#[test]
fn test_currency_code_try_from_valid_usd() {
    let currency = CurrencyCode::try_from("USD").unwrap();
    assert_eq!(currency.to_string(), "USD");
    assert_eq!(currency.as_str(), "USD");
}

#[test]
fn test_currency_code_try_from_valid_eur() {
    let currency = CurrencyCode::try_from("EUR").unwrap();
    assert_eq!(currency.to_string(), "EUR");
}

#[test]
fn test_currency_code_try_from_lowercase() {
    let currency = CurrencyCode::try_from("usd").unwrap();
    assert_eq!(currency.to_string(), "USD");
}

#[test]
fn test_currency_code_try_from_invalid() {
    let result = CurrencyCode::try_from("ABC");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "not a valid ISO 4217 currency code");
}

#[test]
fn test_currency_code_try_from_wrong_length() {
    let result = CurrencyCode::try_from("US");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "currency code must be exactly 3 characters");
}

#[test]
fn test_swift_bic_try_from_valid_8_char() {
    let bic = SwiftBic::try_from("DEUTDEDB").unwrap();
    assert_eq!(bic.to_string(), "DEUTDEDB");
}

#[test]
fn test_swift_bic_try_from_valid_11_char() {
    let bic = SwiftBic::try_from("DEUTDEDBBER").unwrap();
    assert_eq!(bic.to_string(), "DEUTDEDBBER");
}

#[test]
fn test_swift_bic_try_from_lowercase() {
    let bic = SwiftBic::try_from("deutdedb").unwrap();
    assert_eq!(bic.to_string(), "DEUTDEDB");
}

#[test]
fn test_swift_bic_try_from_invalid_digit_in_institution() {
    let result = SwiftBic::try_from("DE1TDEDB");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "institution code must be 4 letters");
}

#[test]
fn test_swift_bic_try_from_too_short() {
    let result = SwiftBic::try_from("DEUTDE");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "BIC must be 8 or 11 characters");
}

#[test]
fn test_swift_bic_try_from_too_long() {
    let result = SwiftBic::try_from("DEUTDEDB1234");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "BIC must be 8 or 11 characters");
}

#[test]
fn test_swift_bic_accessors_8_char() {
    let bic = SwiftBic::try_from("DEUTDEDB").unwrap();
    assert_eq!(bic.institution_code(), "DEUT");
    assert_eq!(bic.country_code(), "DE");
    assert_eq!(bic.location_code(), "DB");
    assert_eq!(bic.branch_code(), None);
}

#[test]
fn test_swift_bic_accessors_11_char() {
    let bic = SwiftBic::try_from("DEUTDEDBBER").unwrap();
    assert_eq!(bic.institution_code(), "DEUT");
    assert_eq!(bic.country_code(), "DE");
    assert_eq!(bic.location_code(), "DB");
    assert_eq!(bic.branch_code(), Some("BER"));
}

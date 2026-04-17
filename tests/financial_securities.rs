use fraiseql_validators::financial_securities::{Cusip, Figi, Lei, Mic, Sedol};
use fraiseql_validators::ValidationError;

#[test]
fn test_cusip_try_from_valid_apple() {
    let cusip = Cusip::try_from("037833100").unwrap();
    assert_eq!(cusip.to_string(), "037833100");
}

#[test]
fn test_cusip_try_from_valid_google() {
    let cusip = Cusip::try_from("38259P508").unwrap();
    assert_eq!(cusip.to_string(), "38259P508");
}

#[test]
fn test_cusip_try_from_lowercase() {
    let cusip = Cusip::try_from("037833100").unwrap();
    assert_eq!(cusip.to_string(), "037833100");
}

#[test]
fn test_cusip_try_from_bad_checksum() {
    let result = Cusip::try_from("037833101");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "checksum validation failed");
}

#[test]
fn test_cusip_try_from_too_short() {
    let result = Cusip::try_from("03783310");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "CUSIP must be exactly 9 characters");
}

#[test]
fn test_cusip_try_from_too_long() {
    let result = Cusip::try_from("0378331001");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "CUSIP must be exactly 9 characters");
}

#[test]
fn test_cusip_accessors() {
    let cusip = Cusip::try_from("037833100").unwrap();
    assert_eq!(cusip.issuer_code(), "037833");
    assert_eq!(cusip.issue_number(), "10");
    assert_eq!(cusip.check_digit(), '0');
}

#[test]
fn test_sedol_try_from_valid_bae() {
    let sedol = Sedol::try_from("0263494").unwrap();
    assert_eq!(sedol.to_string(), "0263494");
}

#[test]
fn test_sedol_try_from_valid_alphanumeric() {
    let sedol = Sedol::try_from("B06YXY6").unwrap();
    assert_eq!(sedol.to_string(), "B06YXY6");
}

#[test]
fn test_sedol_try_from_lowercase() {
    let sedol = Sedol::try_from("b06yxy6").unwrap();
    assert_eq!(sedol.to_string(), "B06YXY6");
}

#[test]
fn test_sedol_try_from_bad_checksum() {
    let result = Sedol::try_from("B06YXY7");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "checksum validation failed");
}

#[test]
fn test_sedol_try_from_too_short() {
    let result = Sedol::try_from("026349");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "SEDOL must be exactly 7 characters");
}

#[test]
fn test_sedol_try_from_vowel_in_position() {
    let result = Sedol::try_from("A263494");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(
        err.reason,
        "invalid SEDOL character or vowel in positions 1-6"
    );
}

#[test]
fn test_sedol_accessor() {
    let sedol = Sedol::try_from("0263494").unwrap();
    assert_eq!(sedol.check_digit(), '4');
}

#[test]
fn test_lei_try_from_valid_apple() {
    let lei = Lei::try_from("7LTWFZYICNSX8D621K86").unwrap();
    assert_eq!(lei.to_string(), "7LTWFZYICNSX8D621K86");
}

#[test]
fn test_lei_try_from_lowercase() {
    let lei = Lei::try_from("7ltwfzyicnsx8d621k86").unwrap();
    assert_eq!(lei.to_string(), "7LTWFZYICNSX8D621K86");
}

#[test]
fn test_lei_try_from_bad_checksum() {
    let result = Lei::try_from("7LTWFZYICNSX8D621K87");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "checksum validation failed");
}

#[test]
fn test_lei_try_from_too_short() {
    let result = Lei::try_from("7LTWFZYICNSX8D621K8");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "LEI must be exactly 20 characters");
}

#[test]
fn test_lei_accessors() {
    let lei = Lei::try_from("7LTWFZYICNSX8D621K86").unwrap();
    assert_eq!(lei.lou_code(), "7LTW");
    assert_eq!(lei.entity_code(), "FZYICNSX8D621K");
    assert_eq!(lei.check_digits(), "86");
}

#[test]
fn test_figi_try_from_valid_apple() {
    let figi = Figi::try_from("BBG000BLNNH6").unwrap();
    assert_eq!(figi.to_string(), "BBG000BLNNH6");
}

#[test]
fn test_figi_try_from_lowercase() {
    let figi = Figi::try_from("bbg000blnnh6").unwrap();
    assert_eq!(figi.to_string(), "BBG000BLNNH6");
}

#[test]
fn test_figi_try_from_bad_checksum() {
    let result = Figi::try_from("BBG000BLNNH7");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "Luhn checksum validation failed");
}

#[test]
fn test_figi_try_from_too_short() {
    let result = Figi::try_from("BBG000BLNNH");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "FIGI must be exactly 12 characters");
}

#[test]
fn test_figi_try_from_wrong_third_char() {
    let result = Figi::try_from("BAG000BLNNH6");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "third character must be 'G'");
}

#[test]
fn test_figi_try_from_vowel_first() {
    let result = Figi::try_from("ABG000BLNNH6");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "first 2 characters must be consonants");
}

#[test]
fn test_figi_accessors() {
    let figi = Figi::try_from("BBG000BLNNH6").unwrap();
    assert_eq!(figi.provider_code(), "BB");
    assert_eq!(figi.security_code(), "G000BLNNH");
    assert_eq!(figi.check_digit(), '6');
}

#[test]
fn test_mic_try_from_valid_nyse() {
    let mic = Mic::try_from("XNYS").unwrap();
    assert_eq!(mic.to_string(), "XNYS");
    assert_eq!(mic.as_str(), "XNYS");
}

#[test]
fn test_mic_try_from_valid_london() {
    let mic = Mic::try_from("XLON").unwrap();
    assert_eq!(mic.to_string(), "XLON");
}

#[test]
fn test_mic_try_from_lowercase() {
    let mic = Mic::try_from("xnys").unwrap();
    assert_eq!(mic.to_string(), "XNYS");
}

#[test]
fn test_mic_try_from_invalid() {
    let result = Mic::try_from("ZZZZ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "not a valid ISO 10383 MIC");
}

#[test]
fn test_mic_try_from_wrong_length() {
    let result = Mic::try_from("XNY");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "MIC must be exactly 4 characters");
}

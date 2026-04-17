use fraiseql_validators::barcodes::{Ean8, UpcA, Ean13, Isbn13, Issn, Gtin14};

#[test]
fn test_ean8_try_from_valid() {
    let ean8 = Ean8::try_from("73513537").unwrap();
    assert_eq!(format!("{}", ean8), "73513537");
    assert_eq!(ean8.check_digit(), '7');
}

#[test]
fn test_ean8_try_from_bad_check_digit() {
    let result = Ean8::try_from("73513538");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid check digit");
}

#[test]
fn test_ean8_try_from_too_short() {
    let result = Ean8::try_from("7351353");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 8 characters");
}

#[test]
fn test_ean8_try_from_too_long() {
    let result = Ean8::try_from("735135370");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 8 characters");
}

#[test]
fn test_ean8_try_from_non_digit() {
    let result = Ean8::try_from("7351353A");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must contain only digits");
}

#[test]
fn test_upca_try_from_valid() {
    let upca = UpcA::try_from("012345678905").unwrap();
    assert_eq!(format!("{}", upca), "012345678905");
}

#[test]
fn test_upca_try_from_bad_check_digit() {
    let result = UpcA::try_from("012345678906");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid check digit");
}

#[test]
fn test_upca_try_from_too_short() {
    let result = UpcA::try_from("01234567890");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 12 characters");
}

#[test]
fn test_upca_to_ean13() {
    let upca = UpcA::try_from("012345678905").unwrap();
    let ean13 = upca.to_ean13();
    assert_eq!(format!("{}", ean13), "0012345678905");
}

#[test]
fn test_ean13_try_from_valid() {
    let ean13 = Ean13::try_from("5901234123457").unwrap();
    assert_eq!(format!("{}", ean13), "5901234123457");
    assert_eq!(ean13.gs1_prefix(), "590");
}

#[test]
fn test_ean13_try_from_bad_check_digit() {
    let result = Ean13::try_from("5901234123458");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid check digit");
}

#[test]
fn test_ean13_try_from_too_short() {
    let result = Ean13::try_from("978030640615");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 13 characters");
}

#[test]
fn test_isbn13_try_from_valid_978() {
    let isbn = Isbn13::try_from("9780306406157").unwrap();
    assert_eq!(format!("{}", isbn), "9780306406157");
    assert_eq!(isbn.registration_group(), "0");
}

#[test]
fn test_isbn13_try_from_valid_979() {
    let isbn = Isbn13::try_from("9790306406156").unwrap();
    assert_eq!(format!("{}", isbn), "9790306406156");
}

#[test]
fn test_isbn13_try_from_wrong_prefix() {
    let result = Isbn13::try_from("9770306406158");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "ISBN-13 must have prefix 978 or 979");
}

#[test]
fn test_isbn13_try_from_bad_check_digit() {
    let result = Isbn13::try_from("9780306406158");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid check digit");
}

#[test]
fn test_isbn13_as_ean13() {
    let isbn = Isbn13::try_from("9780306406157").unwrap();
    let ean13 = isbn.as_ean13();
    assert_eq!(format!("{}", ean13), "9780306406157");
    assert_eq!(ean13.gs1_prefix(), "978");
}

#[test]
fn test_issn_try_from_valid_with_hyphen() {
    let issn = Issn::try_from("0317-8471").unwrap();
    assert_eq!(format!("{}", issn), "0317-8471");
}

#[test]
fn test_issn_try_from_valid_without_hyphen() {
    let issn = Issn::try_from("03178471").unwrap();
    assert_eq!(format!("{}", issn), "0317-8471");
}

#[test]
fn test_issn_try_from_valid_with_x() {
    let issn = Issn::try_from("0378-5955").unwrap();
    assert_eq!(format!("{}", issn), "0378-5955");
}

#[test]
fn test_issn_try_from_bad_check_digit() {
    let result = Issn::try_from("0317-8472");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid check character");
}

#[test]
fn test_gtin14_try_from_valid() {
    let gtin = Gtin14::try_from("10614141000415").unwrap();
    assert_eq!(format!("{}", gtin), "10614141000415");
    assert_eq!(gtin.indicator_digit(), '1');
}

#[test]
fn test_gtin14_try_from_bad_check_digit() {
    let result = Gtin14::try_from("10614141000416");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid check digit");
}

#[test]
fn test_gtin14_try_from_too_short() {
    let result = Gtin14::try_from("1061414100041");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 14 characters");
}

#[test]
fn test_gtin14_as_ean13_some() {
    let gtin = Gtin14::try_from("01061414100044").unwrap(); // Indicator 0
    let ean13 = gtin.as_ean13().unwrap();
    assert_eq!(format!("{}", ean13), "1061414100044");
}

#[test]
fn test_gtin14_as_ean13_none() {
    let gtin = Gtin14::try_from("10614141000415").unwrap(); // Indicator 1
    assert!(gtin.as_ean13().is_none());
}
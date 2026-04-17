use fraiseql_validators::geographic::{CountryCode, CountryCodeAlpha3, LanguageCode, Latitude, Longitude};

#[test]
fn test_country_code_try_from_valid() {
    let code = CountryCode::try_from("FR").unwrap();
    assert_eq!(code.as_str(), "FR");
    assert_eq!(format!("{}", code), "FR");
}

#[test]
fn test_country_code_try_from_lowercase() {
    let code = CountryCode::try_from("fr").unwrap();
    assert_eq!(code.as_str(), "FR");
}

#[test]
fn test_country_code_try_from_invalid() {
    let result = CountryCode::try_from("XX");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "not a valid ISO 3166-1 alpha-2 country code");
}

#[test]
fn test_country_code_try_from_wrong_length() {
    let result = CountryCode::try_from("FRA");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 2 characters");
}

#[test]
fn test_country_code_alpha3_try_from_valid() {
    let code = CountryCodeAlpha3::try_from("FRA").unwrap();
    assert_eq!(code.as_str(), "FRA");
    assert_eq!(format!("{}", code), "FRA");
}

#[test]
fn test_country_code_alpha3_try_from_lowercase() {
    let code = CountryCodeAlpha3::try_from("fra").unwrap();
    assert_eq!(code.as_str(), "FRA");
}

#[test]
fn test_country_code_alpha3_try_from_invalid() {
    let result = CountryCodeAlpha3::try_from("XXX");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "not a valid ISO 3166-1 alpha-3 country code");
}

#[test]
fn test_country_code_alpha3_try_from_wrong_length() {
    let result = CountryCodeAlpha3::try_from("FR");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 3 characters");
}

#[test]
fn test_country_codes_consistency() {
    let alpha2 = CountryCode::try_from("FR").unwrap();
    let alpha3 = CountryCodeAlpha3::try_from("FRA").unwrap();
    // Both should succeed and represent the same country
    assert_eq!(alpha2.as_str(), "FR");
    assert_eq!(alpha3.as_str(), "FRA");
}

#[test]
fn test_language_code_try_from_valid() {
    let lang = LanguageCode::try_from("en").unwrap();
    assert_eq!(lang.primary(), "en");
    assert_eq!(format!("{}", lang), "en");
}

#[test]
fn test_language_code_try_from_3_letter() {
    let lang = LanguageCode::try_from("fra").unwrap();
    assert_eq!(lang.primary(), "fra");
    assert_eq!(format!("{}", lang), "fra");
}

#[test]
fn test_language_code_try_from_uppercase() {
    let lang = LanguageCode::try_from("EN").unwrap();
    assert_eq!(lang.primary(), "en");
}

#[test]
fn test_language_code_try_from_invalid() {
    let result = LanguageCode::try_from("xx");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "not a valid IANA language subtag");
}

#[test]
fn test_language_code_try_from_too_short() {
    let result = LanguageCode::try_from("e");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be 2-3 characters");
}

#[test]
fn test_latitude_try_from_valid() {
    let lat = Latitude::try_from(45.0).unwrap();
    assert_eq!(lat.degrees(), 45.0);
    assert_eq!(format!("{}", lat), "45.000000");
}

#[test]
fn test_latitude_try_from_boundary_min() {
    let lat = Latitude::try_from(-90.0).unwrap();
    assert_eq!(lat.degrees(), -90.0);
}

#[test]
fn test_latitude_try_from_boundary_max() {
    let lat = Latitude::try_from(90.0).unwrap();
    assert_eq!(lat.degrees(), 90.0);
}

#[test]
fn test_latitude_try_from_out_of_range() {
    let result = Latitude::try_from(90.1);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be in range [-90.0, 90.0]");
}

#[test]
fn test_latitude_try_from_nan() {
    let result = Latitude::try_from(f64::NAN);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be a finite number");
}

#[test]
fn test_latitude_try_from_string() {
    let lat = Latitude::try_from("45.123").unwrap();
    assert_eq!(lat.degrees(), 45.123);
}

#[test]
fn test_latitude_try_from_invalid_string() {
    let result = Latitude::try_from("not-a-number");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be a valid number");
}

#[test]
fn test_longitude_try_from_valid() {
    let lon = Longitude::try_from(-75.0).unwrap();
    assert_eq!(lon.degrees(), -75.0);
    assert_eq!(format!("{}", lon), "-75.000000");
}

#[test]
fn test_longitude_try_from_boundary_min() {
    let lon = Longitude::try_from(-180.0).unwrap();
    assert_eq!(lon.degrees(), -180.0);
}

#[test]
fn test_longitude_try_from_boundary_max() {
    let lon = Longitude::try_from(180.0).unwrap();
    assert_eq!(lon.degrees(), 180.0);
}

#[test]
fn test_longitude_try_from_out_of_range() {
    let result = Longitude::try_from(180.1);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be in range [-180.0, 180.0]");
}

#[test]
fn test_longitude_try_from_string() {
    let lon = Longitude::try_from("-122.4194").unwrap();
    assert_eq!(lon.degrees(), -122.4194);
}
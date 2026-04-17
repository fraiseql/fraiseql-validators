use fraiseql_validators::geographic::{
    CountryCode, CountryCodeAlpha3, IanaTimezone, IataAirportCode, IcaoAirportCode, LanguageCode,
    Latitude, Longitude, PostalCode,
};

#[test]
fn test_country_code_try_from_valid() {
    let code = CountryCode::try_from("FR").unwrap();
    assert_eq!(code.as_str(), "FR");
    assert_eq!(format!("{code}"), "FR");
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
    assert_eq!(format!("{code}"), "FRA");
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
    assert_eq!(format!("{lang}"), "en");
}

#[test]
fn test_language_code_try_from_3_letter() {
    let lang = LanguageCode::try_from("aaa").unwrap();
    assert_eq!(lang.primary(), "aaa");
    assert_eq!(format!("{lang}"), "aaa");
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
    assert!((lat.degrees() - 45.0).abs() < f64::EPSILON);
    assert_eq!(format!("{lat}"), "45.000000");
}

#[test]
fn test_latitude_try_from_boundary_min() {
    let lat = Latitude::try_from(-90.0).unwrap();
    assert!((lat.degrees() - (-90.0)).abs() < f64::EPSILON);
}

#[test]
fn test_latitude_try_from_boundary_max() {
    let lat = Latitude::try_from(90.0).unwrap();
    assert!((lat.degrees() - 90.0).abs() < f64::EPSILON);
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
    assert!((lat.degrees() - 45.123).abs() < f64::EPSILON);
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
    assert!((lon.degrees() - (-75.0)).abs() < f64::EPSILON);
    assert_eq!(format!("{lon}"), "-75.000000");
}

#[test]
fn test_longitude_try_from_boundary_min() {
    let lon = Longitude::try_from(-180.0).unwrap();
    assert!((lon.degrees() - (-180.0)).abs() < f64::EPSILON);
}

#[test]
fn test_longitude_try_from_boundary_max() {
    let lon = Longitude::try_from(180.0).unwrap();
    assert!((lon.degrees() - 180.0).abs() < f64::EPSILON);
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
    assert!((lon.degrees() - (-122.4194)).abs() < f64::EPSILON);
}

#[test]
fn test_postal_code_try_from_fr_valid() {
    let postal = PostalCode::try_from("FR:75001").unwrap();
    assert_eq!(postal.country(), "FR");
    assert_eq!(postal.code(), "75001");
    assert_eq!(format!("{postal}"), "FR:75001");
}

#[test]
fn test_postal_code_try_from_gb_valid() {
    let postal = PostalCode::try_from("GB:SW1A1AA").unwrap();
    assert_eq!(postal.country(), "GB");
    assert_eq!(postal.code(), "SW1A1AA");
}

#[test]
fn test_postal_code_try_from_gb_with_spaces() {
    let postal = PostalCode::try_from("GB:SW1A 1AA").unwrap();
    assert_eq!(postal.country(), "GB");
    assert_eq!(postal.code(), "SW1A1AA");
}

#[test]
fn test_postal_code_try_from_us_invalid() {
    let result = PostalCode::try_from("US:1000");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid postal code format for country US");
}

#[test]
fn test_postal_code_try_from_invalid_format() {
    let result = PostalCode::try_from("INVALID");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid format, expected COUNTRY:CODE");
}

#[test]
fn test_postal_code_try_from_unknown_country() {
    let postal = PostalCode::try_from("ZZ:12345").unwrap();
    assert_eq!(postal.country(), "ZZ");
    assert_eq!(postal.code(), "12345");
}

#[test]
fn test_postal_code_belongs_to_country() {
    let postal = PostalCode::try_from("FR:75001").unwrap();
    assert!(postal.belongs_to_country("FR"));
    assert!(!postal.belongs_to_country("US"));
}

#[test]
fn test_iata_airport_code_try_from_valid() {
    let code = IataAirportCode::try_from("CDG").unwrap();
    assert_eq!(code.as_str(), "CDG");
    assert_eq!(format!("{code}"), "CDG");
}

#[test]
fn test_iata_airport_code_try_from_lowercase() {
    let code = IataAirportCode::try_from("jfk").unwrap();
    assert_eq!(code.as_str(), "JFK");
}

#[test]
fn test_iata_airport_code_try_from_invalid() {
    let result = IataAirportCode::try_from("ZZZ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, String::from("not a valid IATA airport code"));
}

#[test]
fn test_iata_airport_code_try_from_wrong_length() {
    let result = IataAirportCode::try_from("CD");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, String::from("must be exactly 3 characters"));
}

#[test]
fn test_icao_airport_code_try_from_valid() {
    let code = IcaoAirportCode::try_from("LFPG").unwrap();
    assert_eq!(code.as_str(), "LFPG");
    assert_eq!(format!("{code}"), "LFPG");
}

#[test]
fn test_icao_airport_code_try_from_lowercase() {
    let code = IcaoAirportCode::try_from("kjfk").unwrap();
    assert_eq!(code.as_str(), "KJFK");
}

#[test]
fn test_icao_airport_code_try_from_invalid() {
    let result = IcaoAirportCode::try_from("ZZZZ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, String::from("not a valid ICAO airport code"));
}

#[test]
fn test_icao_airport_code_try_from_wrong_length() {
    let result = IcaoAirportCode::try_from("LFP");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, String::from("must be exactly 4 characters"));
}

#[test]
fn test_iana_timezone_try_from_valid() {
    let tz = IanaTimezone::try_from("Europe/Paris").unwrap();
    assert_eq!(tz.as_str(), "Europe/Paris");
    assert_eq!(format!("{tz}"), "Europe/Paris");
}

#[test]
fn test_iana_timezone_try_from_america() {
    let tz = IanaTimezone::try_from("America/New_York").unwrap();
    assert_eq!(tz.as_str(), "America/New_York");
}

#[test]
fn test_iana_timezone_try_from_utc() {
    let tz = IanaTimezone::try_from("UTC").unwrap();
    assert_eq!(tz.as_str(), "UTC");
}

#[test]
fn test_iana_timezone_try_from_invalid() {
    let result = IanaTimezone::try_from("Europe/Atlantis");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, String::from("not a valid IANA timezone"));
}

#[test]
fn test_iana_timezone_try_from_empty() {
    let result = IanaTimezone::try_from("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, String::from("empty string"));
}

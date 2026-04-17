use fraiseql_validators::identifiers::{Slug, Color, Locale, Semver, Vin};

#[test]
fn test_slug_try_from_valid() {
    let slug = Slug::try_from("hello-world").unwrap();
    assert_eq!(format!("{}", slug), "hello-world");
}

#[test]
fn test_slug_try_from_single_char() {
    let slug = Slug::try_from("a").unwrap();
    assert_eq!(format!("{}", slug), "a");
}

#[test]
fn test_slug_try_from_with_numbers() {
    let slug = Slug::try_from("hello-world-123").unwrap();
    assert_eq!(format!("{}", slug), "hello-world-123");
}

#[test]
fn test_slug_try_from_leading_hyphen() {
    let result = Slug::try_from("-hello");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "starts with hyphen");
}

#[test]
fn test_slug_try_from_trailing_hyphen() {
    let result = Slug::try_from("hello-");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "ends with hyphen");
}

#[test]
fn test_slug_try_from_uppercase() {
    let result = Slug::try_from("Hello");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "contains invalid character (must be lowercase letters, digits, or hyphens)");
}

#[test]
fn test_slug_try_from_consecutive_hyphens() {
    let result = Slug::try_from("hello--world");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "consecutive hyphens");
}

#[test]
fn test_slug_try_from_empty() {
    let result = Slug::try_from("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "empty string");
}

#[test]
fn test_color_try_from_hex6() {
    let color = Color::try_from("#FF5733").unwrap();
    assert_eq!(color.red(), 255);
    assert_eq!(color.green(), 87);
    assert_eq!(color.blue(), 51);
    assert_eq!(color.to_hex(), "#FF5733");
    assert_eq!(format!("{}", color), "#FF5733");
}

#[test]
fn test_color_try_from_hex3() {
    let color = Color::try_from("#f57").unwrap();
    assert_eq!(color.red(), 255); // f*17 = 15*17=255
    assert_eq!(color.green(), 85); // 5*17=85
    assert_eq!(color.blue(), 119); // 7*17=119
    assert_eq!(color.to_hex(), "#FF5577");
}

#[test]
fn test_color_try_from_lowercase() {
    let color = Color::try_from("#ff5733").unwrap();
    assert_eq!(color.to_hex(), "#FF5733");
}

#[test]
fn test_color_try_from_missing_hash() {
    let result = Color::try_from("FF5733");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must start with '#'");
}

#[test]
fn test_color_try_from_invalid_hex() {
    let result = Color::try_from("#GG5733");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid hex digits");
}

#[test]
fn test_color_try_from_wrong_length() {
    let result = Color::try_from("#FF573");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be 3 or 6 hex digits after '#'");
}

#[test]
fn test_color_luminance() {
    let black = Color::try_from("#000000").unwrap();
    assert!((black.luminance() - 0.0).abs() < 0.001);

    let white = Color::try_from("#FFFFFF").unwrap();
    assert!((white.luminance() - 1.0).abs() < 0.001);

    let red = Color::try_from("#FF0000").unwrap();
    assert!((red.luminance() - 0.2126).abs() < 0.001);
}

#[test]
fn test_locale_try_from_valid() {
    let locale = Locale::try_from("en").unwrap();
    assert_eq!(format!("{}", locale), "en");
    assert_eq!(locale.language(), "en");
    assert_eq!(locale.region(), None);
    assert_eq!(locale.script(), None);
}

#[test]
fn test_locale_try_from_with_region() {
    let locale = Locale::try_from("en-US").unwrap();
    assert_eq!(locale.language(), "en");
    assert_eq!(locale.region(), Some("US"));
    assert_eq!(locale.script(), None);
}

#[test]
fn test_locale_try_from_with_script_region() {
    let locale = Locale::try_from("zh-Hant-TW").unwrap();
    assert_eq!(locale.language(), "zh");
    assert_eq!(locale.script(), Some("Hant"));
    assert_eq!(locale.region(), Some("TW"));
}

#[test]
fn test_locale_try_from_french() {
    let locale = Locale::try_from("fr-FR").unwrap();
    assert_eq!(locale.language(), "fr");
    assert_eq!(locale.region(), Some("FR"));
}

#[test]
fn test_locale_try_from_serbian() {
    let locale = Locale::try_from("sr-Latn-RS").unwrap();
    assert_eq!(locale.language(), "sr");
    assert_eq!(locale.script(), Some("Latn"));
    assert_eq!(locale.region(), Some("RS"));
}

#[test]
fn test_locale_try_from_uppercase_language() {
    let result = Locale::try_from("EN");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "language must be 2-3 lowercase letters");
}

#[test]
fn test_locale_try_from_empty() {
    let result = Locale::try_from("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "empty string");
}

#[test]
fn test_semver_try_from_valid() {
    let semver = Semver::try_from("1.0.0").unwrap();
    assert_eq!(semver.major(), 1);
    assert_eq!(semver.minor(), 0);
    assert_eq!(semver.patch(), 0);
    assert_eq!(semver.pre_release(), None);
    assert_eq!(semver.build_metadata(), None);
    assert_eq!(format!("{}", semver), "1.0.0");
}

#[test]
fn test_semver_try_from_with_pre_release() {
    let semver = Semver::try_from("1.0.0-alpha.1").unwrap();
    assert_eq!(semver.major(), 1);
    assert_eq!(semver.minor(), 0);
    assert_eq!(semver.patch(), 0);
    assert_eq!(semver.pre_release(), Some("alpha.1"));
    assert_eq!(semver.build_metadata(), None);
    assert_eq!(format!("{}", semver), "1.0.0-alpha.1");
}

#[test]
fn test_semver_try_from_with_build() {
    let semver = Semver::try_from("1.0.0-alpha.1+build.123").unwrap();
    assert_eq!(semver.major(), 1);
    assert_eq!(semver.minor(), 0);
    assert_eq!(semver.patch(), 0);
    assert_eq!(semver.pre_release(), Some("alpha.1"));
    assert_eq!(semver.build_metadata(), Some("build.123"));
    assert_eq!(format!("{}", semver), "1.0.0-alpha.1+build.123");
}

#[test]
fn test_semver_try_from_leading_zero() {
    let result = Semver::try_from("01.0.0");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "version components cannot have leading zeros");
}

#[test]
fn test_semver_try_from_missing_patch() {
    let result = Semver::try_from("1.0");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must have exactly 3 numeric parts separated by dots");
}

#[test]
fn test_semver_try_from_empty_pre_release() {
    let result = Semver::try_from("1.0.0-");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid pre-release identifier");
}

#[test]
fn test_semver_compatible_with() {
    let v100 = Semver::try_from("1.0.0").unwrap();
    let v101 = Semver::try_from("1.0.1").unwrap();
    let v110 = Semver::try_from("1.1.0").unwrap();
    let v200 = Semver::try_from("2.0.0").unwrap();

    assert!(v101.compatible_with(&v100)); // same major.minor, patch >=
    assert!(!v110.compatible_with(&v100)); // different minor
    assert!(!v100.compatible_with(&v101)); // patch < base.patch
    assert!(!v200.compatible_with(&v100)); // different major
}

#[test]
fn test_semver_caret_compatible_with() {
    let v100 = Semver::try_from("1.0.0").unwrap();
    let v101 = Semver::try_from("1.0.1").unwrap();
    let v110 = Semver::try_from("1.1.0").unwrap();
    let v200 = Semver::try_from("2.0.0").unwrap();

    assert!(v101.caret_compatible_with(&v100));
    assert!(v110.caret_compatible_with(&v100));
    assert!(!v100.caret_compatible_with(&v101));
    assert!(!v200.caret_compatible_with(&v100));
}

#[test]
fn test_semver_ordering() {
    let v100 = Semver::try_from("1.0.0").unwrap();
    let v101 = Semver::try_from("1.0.1").unwrap();
    let v110 = Semver::try_from("1.1.0").unwrap();
    let v100_alpha = Semver::try_from("1.0.0-alpha").unwrap();
    let v100_beta = Semver::try_from("1.0.0-beta").unwrap();
    let v100_alpha1 = Semver::try_from("1.0.0-alpha.1").unwrap();
    let v100_build1 = Semver::try_from("1.0.0+build.1").unwrap();
    let v100_build2 = Semver::try_from("1.0.0+build.2").unwrap();

    assert!(v100 < v101);
    assert!(v101 < v110);
    assert!(v100_alpha < v100);
    assert!(v100_alpha < v100_beta);
    assert!(v100_alpha < v100_alpha1);
    assert_eq!(v100_build1, v100_build2); // build metadata ignored
}

#[test]
fn test_semver_roundtrip() {
    let original = "1.0.0-alpha.1+build.123";
    let semver = Semver::try_from(original).unwrap();
    let roundtrip = format!("{}", semver);
    assert_eq!(roundtrip, original);
}

#[test]
fn test_vin_try_from_valid() {
    let vin = Vin::try_from("1HGBH41JXMN109186").unwrap();
    assert_eq!(format!("{}", vin), "1HGBH41JXMN109186");
    assert_eq!(vin.wmi(), "1HG");
    assert_eq!(vin.vds(), "BH41JX");
    assert_eq!(vin.vis(), "MN109186");
    assert_eq!(vin.check_digit(), 'X');
    assert_eq!(vin.model_year_char(), 'M');
}

#[test]
fn test_vin_try_from_lowercase() {
    let vin = Vin::try_from("1hgbh41jxmn109186").unwrap();
    assert_eq!(format!("{}", vin), "1HGBH41JXMN109186");
}

#[test]
fn test_vin_try_from_invalid_char() {
    let result = Vin::try_from("1HGBH41JOMN109186");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "contains invalid character (must be A-H, J-N, P-R, S-Z, or 0-9, excluding I, O, Q)");
}

#[test]
fn test_vin_try_from_too_short() {
    let result = Vin::try_from("1HGBH41JXMN10918");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 17 characters");
}

#[test]
fn test_vin_try_from_too_long() {
    let result = Vin::try_from("1HGBH41JXMN1091860");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must be exactly 17 characters");
}

#[test]
fn test_vin_try_from_bad_check_digit() {
    let result = Vin::try_from("1HGBH41JYMN109186");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid check digit");
}
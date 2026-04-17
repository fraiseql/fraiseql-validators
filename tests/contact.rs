use fraiseql_validators::contact::{DomainName, Email, PhoneE164};

#[test]
fn test_email_try_from_valid() {
    let email = Email::try_from("user@example.com").unwrap();
    assert_eq!(format!("{}", email), "user@example.com");
}

#[test]
fn test_email_try_from_localhost_domain() {
    let result = Email::try_from("user@localhost");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "single-label domain not allowed");
}

#[test]
fn test_email_try_from_empty() {
    let result = Email::try_from("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "empty string");
}

#[test]
fn test_email_try_from_too_long() {
    let long_email = "a".repeat(255) + "@example.com";
    let result = Email::try_from(long_email.as_str());
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "email too long (> 254 characters)");
}

#[test]
fn test_email_accessors() {
    let email = Email::try_from("user@example.com").unwrap();
    assert_eq!(email.local(), "user");
    assert_eq!(email.domain(), "example.com");
}

#[test]
fn test_email_belongs_to_domain() {
    let email = Email::try_from("user@example.com").unwrap();
    assert!(email.belongs_to_domain("example.com"));
    assert!(!email.belongs_to_domain("other.com"));
}

#[test]
fn test_email_case_insensitive_eq() {
    let email1 = Email::try_from("User@Example.COM").unwrap();
    let email2 = Email::try_from("user@example.com").unwrap();
    assert_eq!(email1, email2);
}

#[test]
fn test_email_roundtrip() {
    let original = "test.email+tag@example.co.uk";
    let email = Email::try_from(original).unwrap();
    let roundtrip = format!("{}", email);
    let email2 = Email::try_from(roundtrip.as_str()).unwrap();
    assert_eq!(email, email2);
}

#[test]
fn test_phone_e164_try_from_valid() {
    let phone = PhoneE164::try_from("+14155552671").unwrap();
    assert_eq!(format!("{}", phone), "+14155552671");
}

#[test]
fn test_phone_e164_try_from_missing_plus() {
    let result = PhoneE164::try_from("14155552671");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "must start with '+'");
}

#[test]
fn test_phone_e164_try_from_too_short() {
    let result = PhoneE164::try_from("+1");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid length (6-14 digits after +)");
}

#[test]
fn test_phone_e164_try_from_too_long() {
    let result = PhoneE164::try_from("+123456789012345678");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid length (6-14 digits after +)");
}

#[test]
fn test_phone_e164_accessors() {
    let phone = PhoneE164::try_from("+14155552671").unwrap();
    assert_eq!(phone.country_code(), "1");
    assert_eq!(phone.national_number(), "4155552671");
}

#[test]
fn test_phone_e164_roundtrip() {
    let original = "+33123456789";
    let phone = PhoneE164::try_from(original).unwrap();
    let roundtrip = format!("{}", phone);
    assert_eq!(roundtrip, original);
    let phone2 = PhoneE164::try_from(roundtrip.as_str()).unwrap();
    assert_eq!(phone2.country_code(), "33");
    assert_eq!(phone2.national_number(), "123456789");
}

#[test]
fn test_domain_name_try_from_valid() {
    let domain = DomainName::try_from("example.com").unwrap();
    assert_eq!(format!("{}", domain), "example.com");
}

#[test]
fn test_domain_name_try_from_subdomain() {
    let domain = DomainName::try_from("sub.example.co.uk").unwrap();
    assert_eq!(format!("{}", domain), "sub.example.co.uk");
}

#[test]
fn test_domain_name_try_from_localhost() {
    let domain = DomainName::try_from("localhost").unwrap();
    assert_eq!(format!("{}", domain), "localhost");
}

#[test]
fn test_domain_name_try_from_starts_with_hyphen() {
    let result = DomainName::try_from("-example.com");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "label starts with hyphen");
}

#[test]
fn test_domain_name_try_from_ends_with_hyphen() {
    let result = DomainName::try_from("example-.com");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "label ends with hyphen");
}

#[test]
fn test_domain_name_try_from_empty_label() {
    let result = DomainName::try_from("example..com");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "empty label");
}

#[test]
fn test_domain_name_try_from_spaces() {
    let result = DomainName::try_from("exa mple.com");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "spaces not allowed");
}

#[test]
fn test_domain_name_try_from_scheme() {
    let result = DomainName::try_from("https://example.com");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "scheme not allowed");
}

#[test]
fn test_domain_name_roundtrip() {
    let original = "Sub.Example.Co.UK";
    let domain = DomainName::try_from(original).unwrap();
    let roundtrip = format!("{}", domain);
    assert_eq!(roundtrip, "sub.example.co.uk");
    let domain2 = DomainName::try_from(roundtrip.as_str()).unwrap();
    assert_eq!(format!("{}", domain2), "sub.example.co.uk");
}

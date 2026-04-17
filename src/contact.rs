//! Validation for contact information types.
//!
//! This module provides validators for email addresses, E.164 phone numbers,
//! and domain names.

use crate::ValidationError;
use alloc::string::String;
use once_cell::sync::Lazy;
use regex_lite::Regex;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap()
});

#[derive(Clone, Debug)]
pub struct Email(String);

impl Email {
    pub fn local(&self) -> &str {
        self.0
            .split('@')
            .next()
            .expect("Email invariant: always contains '@'")
    }

    pub fn domain(&self) -> &str {
        self.0
            .split('@')
            .nth(1)
            .expect("Email invariant: always contains '@'")
    }

    pub fn belongs_to_domain(&self, domain: &str) -> bool {
        self.domain().eq_ignore_ascii_case(domain)
    }
}

impl core::fmt::Display for Email {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for Email {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Email",
                input: String::from(value),
                reason: "empty string",
            });
        }

        if value.len() > 254 {
            return Err(ValidationError {
                type_name: "Email",
                input: String::from(value),
                reason: "email too long (> 254 characters)",
            });
        }

        if !EMAIL_REGEX.is_match(value) {
            return Err(ValidationError {
                type_name: "Email",
                input: String::from(value),
                reason: "invalid email format",
            });
        }

        // Check for single-label domain (no dots)
        if let Some(at_pos) = value.find('@') {
            let domain = &value[at_pos + 1..];
            if !domain.contains('.') {
                return Err(ValidationError {
                    type_name: "Email",
                    input: String::from(value),
                    reason: "single-label domain not allowed",
                });
            }
        }

        Ok(Email(value.to_lowercase()))
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for Email {}

impl core::hash::Hash for Email {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhoneE164(String);

impl PhoneE164 {
    pub fn country_code(&self) -> &str {
        // Simple heuristic: 1 digit if starts with 1 or 7, else 2 digits
        let s = &self.0[1..]; // skip +
        if s.starts_with('1') || s.starts_with('7') {
            &s[0..1]
        } else {
            &s[0..2]
        }
    }

    pub fn national_number(&self) -> &str {
        let cc_len = self.country_code().len();
        &self.0[1 + cc_len..]
    }
}

impl core::fmt::Display for PhoneE164 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for PhoneE164 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with('+') {
            return Err(ValidationError {
                type_name: "PhoneE164",
                input: String::from(value),
                reason: "must start with '+'",
            });
        }

        let digits = &value[1..];
        if digits.len() < 6 || digits.len() > 14 {
            return Err(ValidationError {
                type_name: "PhoneE164",
                input: String::from(value),
                reason: "invalid length (6-14 digits after +)",
            });
        }

        if !digits.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError {
                type_name: "PhoneE164",
                input: String::from(value),
                reason: "must contain only digits after +",
            });
        }

        Ok(PhoneE164(String::from(value)))
    }
}

fn is_valid_domain_label(label: &str) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }
    if label.starts_with('-') || label.ends_with('-') {
        return false;
    }
    label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DomainName(String);

impl DomainName {
    // No specific accessors needed
}

impl core::fmt::Display for DomainName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for DomainName {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "DomainName",
                input: String::from(value),
                reason: "empty string",
            });
        }

        if value.len() > 253 {
            return Err(ValidationError {
                type_name: "DomainName",
                input: String::from(value),
                reason: "domain too long (> 253 characters)",
            });
        }

        if value.contains("://") {
            return Err(ValidationError {
                type_name: "DomainName",
                input: String::from(value),
                reason: "scheme not allowed",
            });
        }

        if value.contains(' ') {
            return Err(ValidationError {
                type_name: "DomainName",
                input: String::from(value),
                reason: "spaces not allowed",
            });
        }

        let labels: alloc::vec::Vec<&str> = value.split('.').collect();
        for (i, label) in labels.iter().enumerate() {
            if !is_valid_domain_label(label) {
                return Err(ValidationError {
                    type_name: "DomainName",
                    input: String::from(value),
                    reason: if label.starts_with('-') {
                        "label starts with hyphen"
                    } else if label.ends_with('-') {
                        "label ends with hyphen"
                    } else if label.is_empty() && i > 0 {
                        "empty label"
                    } else {
                        "invalid label characters"
                    },
                });
            }
        }

        Ok(DomainName(value.to_lowercase()))
    }
}

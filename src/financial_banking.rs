//! Validation for financial banking types.
//!
//! This module provides validators for IBANs, ISINs, currency codes (ISO 4217),
//! and SWIFT/BIC codes.

use crate::checksum::{isin_numeric_expansion, luhn_valid};
use crate::ValidationError;
use alloc::{string::String, vec::Vec};

// Include generated currency codes
include!(concat!(env!("OUT_DIR"), "/currency_codes.rs"));

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Iban(String);

impl Iban {
    #[must_use]
    pub fn country(&self) -> &str {
        &self.0[..2]
    }

    #[must_use]
    pub fn check_digits(&self) -> &str {
        &self.0[2..4]
    }

    #[must_use]
    pub fn bban(&self) -> &str {
        &self.0[4..]
    }
}

impl core::fmt::Display for Iban {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for Iban {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Iban",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        if value.len() < 15 || value.len() > 34 {
            return Err(ValidationError {
                type_name: "Iban",
                input: String::from(value),
                reason: String::from("invalid length (15-34 characters)"),
            });
        }

        // Convert to uppercase
        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        // Check country code (first 2 chars must be letters)
        if !bytes[0].is_ascii_uppercase() || !bytes[1].is_ascii_uppercase() {
            return Err(ValidationError {
                type_name: "Iban",
                input: String::from(value),
                reason: String::from("country code must be two uppercase letters"),
            });
        }

        // Check check digits (next 2 chars must be digits)
        if !bytes[2].is_ascii_digit() || !bytes[3].is_ascii_digit() {
            return Err(ValidationError {
                type_name: "Iban",
                input: String::from(value),
                reason: String::from("check digits must be two digits"),
            });
        }

        // Check remaining chars are alphanumeric
        for &b in &bytes[4..] {
            if !b.is_ascii_alphanumeric() {
                return Err(ValidationError {
                    type_name: "Iban",
                    input: String::from(value),
                    reason: String::from("BBAN must contain only alphanumeric characters"),
                });
            }
        }

        // Perform MOD-97 validation
        // Rearrange: move first 4 chars to end
        let mut rearranged = String::from(&upper[4..]);
        rearranged.push_str(&upper[..4]);
        let mut remainder = 0u32;

        for c in rearranged.chars() {
            let digit = if c.is_ascii_digit() {
                c.to_digit(10).expect("IBAN digit invariant")
            } else if c.is_ascii_uppercase() {
                // A=10, B=11, ..., Z=35
                u32::from(c as u8 - b'A' + 10)
            } else {
                return Err(ValidationError {
                    type_name: "Iban",
                    input: String::from(value),
                    reason: String::from("invalid character in IBAN"),
                });
            };

            // For 2-digit numbers (letters), we need to process each digit
            if digit >= 10 {
                // First digit
                remainder = (remainder * 10 + digit / 10) % 97;
                // Second digit
                remainder = (remainder * 10 + digit % 10) % 97;
            } else {
                remainder = (remainder * 10 + digit) % 97;
            }
        }

        if remainder != 1 {
            return Err(ValidationError {
                type_name: "Iban",
                input: String::from(value),
                reason: String::from("checksum validation failed"),
            });
        }

        Ok(Self(upper))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Isin(String);

impl Isin {
    #[must_use]
    pub fn country(&self) -> &str {
        &self.0[..2]
    }

    #[must_use]
    pub fn nsin(&self) -> &str {
        &self.0[2..11]
    }

    /// Returns the check digit character.
    ///
    /// # Panics
    ///
    /// Cannot panic — the constructor guarantees the ISIN is always 12 chars.
    #[must_use]
    pub fn check_digit(&self) -> char {
        self.0
            .chars()
            .nth(11)
            .expect("Isin invariant: always 12 chars")
    }
}

impl core::fmt::Display for Isin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for Isin {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Isin",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        if value.len() != 12 {
            return Err(ValidationError {
                type_name: "Isin",
                input: String::from(value),
                reason: String::from("ISIN must be exactly 12 characters"),
            });
        }

        let upper = value.to_uppercase();
        let chars: Vec<char> = upper.chars().collect();

        // Last: digit (check first to match test expectations)
        if !chars[11].is_ascii_digit() {
            return Err(ValidationError {
                type_name: "Isin",
                input: String::from(value),
                reason: String::from("last character must be a digit"),
            });
        }

        // First 2: letters
        if !chars[0].is_ascii_uppercase() || !chars[1].is_ascii_uppercase() {
            return Err(ValidationError {
                type_name: "Isin",
                input: String::from(value),
                reason: String::from("first two characters must be letters"),
            });
        }

        // Next 9: alphanumeric
        for &c in &chars[2..11] {
            if !c.is_ascii_alphanumeric() {
                return Err(ValidationError {
                    type_name: "Isin",
                    input: String::from(value),
                    reason: String::from("characters 3-11 must be alphanumeric"),
                });
            }
        }

        // Luhn validation on numeric expansion
        let expansion = isin_numeric_expansion(&upper);
        if !luhn_valid(&expansion) {
            return Err(ValidationError {
                type_name: "Isin",
                input: String::from(value),
                reason: String::from("Luhn checksum validation failed"),
            });
        }

        Ok(Self(upper))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CurrencyCode([u8; 3]);

impl CurrencyCode {
    /// Returns the currency code as a string slice.
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid UTF-8.
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("CurrencyCode invariant: always valid UTF-8")
    }
}

impl core::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::convert::TryFrom<&str> for CurrencyCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(ValidationError {
                type_name: "CurrencyCode",
                input: String::from(value),
                reason: String::from("currency code must be exactly 3 characters"),
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        for &b in bytes {
            if !b.is_ascii_uppercase() {
                return Err(ValidationError {
                    type_name: "CurrencyCode",
                    input: String::from(value),
                    reason: String::from("currency code must contain only uppercase letters"),
                });
            }
        }

        // Check against whitelist
        if !is_valid_currency_codes(&upper) {
            return Err(ValidationError {
                type_name: "CurrencyCode",
                input: String::from(value),
                reason: String::from("not a valid ISO 4217 currency code"),
            });
        }

        let mut arr = [0u8; 3];
        arr.copy_from_slice(bytes);

        Ok(Self(arr))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SwiftBic(String);

impl SwiftBic {
    #[must_use]
    pub fn institution_code(&self) -> &str {
        &self.0[..4]
    }

    #[must_use]
    pub fn country_code(&self) -> &str {
        &self.0[4..6]
    }

    #[must_use]
    pub fn location_code(&self) -> &str {
        &self.0[6..8]
    }

    #[must_use]
    pub fn branch_code(&self) -> Option<&str> {
        if self.0.len() == 11 {
            Some(&self.0[8..11])
        } else {
            None
        }
    }
}

impl core::fmt::Display for SwiftBic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for SwiftBic {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "SwiftBic",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        let len = value.len();
        if len != 8 && len != 11 {
            return Err(ValidationError {
                type_name: "SwiftBic",
                input: String::from(value),
                reason: String::from("BIC must be 8 or 11 characters"),
            });
        }

        let upper = value.to_uppercase();
        let chars: Vec<char> = upper.chars().collect();

        // First 4: letters (institution code)
        for &c in &chars[0..4] {
            if !c.is_ascii_uppercase() {
                return Err(ValidationError {
                    type_name: "SwiftBic",
                    input: String::from(value),
                    reason: String::from("institution code must be 4 letters"),
                });
            }
        }

        // Next 2: letters (country code)
        for &c in &chars[4..6] {
            if !c.is_ascii_uppercase() {
                return Err(ValidationError {
                    type_name: "SwiftBic",
                    input: String::from(value),
                    reason: String::from("country code must be 2 letters"),
                });
            }
        }

        // Next 2: alphanumeric (location code)
        for &c in &chars[6..8] {
            if !c.is_ascii_alphanumeric() {
                return Err(ValidationError {
                    type_name: "SwiftBic",
                    input: String::from(value),
                    reason: String::from("location code must be 2 alphanumeric characters"),
                });
            }
        }

        // Optional last 3: alphanumeric (branch code)
        if len == 11 {
            for &c in &chars[8..11] {
                if !c.is_ascii_alphanumeric() {
                    return Err(ValidationError {
                        type_name: "SwiftBic",
                        input: String::from(value),
                        reason: String::from("branch code must be 3 alphanumeric characters"),
                    });
                }
            }
        }

        Ok(Self(upper))
    }
}

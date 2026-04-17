//! Validation for financial securities types.
//!
//! This module provides validators for CUSIP, SEDOL, LEI, FIGI, and MIC
//! (Market Identifier Code) identifiers.

use crate::checksum::{isin_numeric_expansion, luhn_valid};
use crate::ValidationError;
use alloc::{string::String, vec::Vec};

// Include generated MIC codes
include!(concat!(env!("OUT_DIR"), "/mic_codes.rs"));

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cusip(String);

impl Cusip {
    #[must_use]
    pub fn issuer_code(&self) -> &str {
        &self.0[..6]
    }

    #[must_use]
    pub fn issue_number(&self) -> &str {
        &self.0[6..8]
    }

    /// Returns the check digit character.
    ///
    /// # Panics
    ///
    /// Cannot panic — the constructor guarantees the CUSIP is always 9 chars.
    #[must_use]
    pub fn check_digit(&self) -> char {
        self.0
            .chars()
            .nth(8)
            .expect("Cusip invariant: always 9 chars")
    }
}

impl core::fmt::Display for Cusip {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

const fn cusip_char_value(c: char) -> Option<u32> {
    match c {
        '0'..='9' => Some(c.to_digit(10).expect("Digit char invariant")),
        'A'..='Z' => Some(10 + (c as u32 - 'A' as u32)),
        '*' => Some(36),
        '@' => Some(37),
        '#' => Some(38),
        _ => None,
    }
}

#[allow(clippy::cast_possible_truncation)] // result is always 0-9
fn cusip_check_digit(s: &str) -> u8 {
    let mut sum = 0;
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if let Some(value) = cusip_char_value(c) {
            let weight = if (i + 1) % 2 == 0 { 2 } else { 1 };
            let product = value * weight;
            sum += product / 10 + product % 10;
        }
    }

    ((10 - (sum % 10)) % 10) as u8
}

impl core::convert::TryFrom<&str> for Cusip {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Cusip",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        if value.len() != 9 {
            return Err(ValidationError {
                type_name: "Cusip",
                input: String::from(value),
                reason: String::from("CUSIP must be exactly 9 characters"),
            });
        }

        let upper = value.to_uppercase();

        // Check that all characters are valid CUSIP chars
        for c in upper.chars() {
            if cusip_char_value(c).is_none() {
                return Err(ValidationError {
                    type_name: "Cusip",
                    input: String::from(value),
                    reason: String::from("invalid CUSIP character"),
                });
            }
        }

        // Check check digit
        let expected_check = cusip_check_digit(&upper[..8]);
        #[allow(clippy::cast_possible_truncation)] // digit is 0-9
        let actual_check = upper
            .chars()
            .nth(8)
            .expect("Cusip invariant: always 9 chars")
            .to_digit(10)
            .expect("Cusip invariant: check digit is digit") as u8;

        if expected_check != actual_check {
            return Err(ValidationError {
                type_name: "Cusip",
                input: String::from(value),
                reason: String::from("checksum validation failed"),
            });
        }

        Ok(Self(upper))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Sedol(String);

impl Sedol {
    /// Returns the check digit character.
    ///
    /// # Panics
    ///
    /// Cannot panic — the constructor guarantees the SEDOL is always 7 chars.
    #[must_use]
    pub fn check_digit(&self) -> char {
        self.0
            .chars()
            .nth(6)
            .expect("Sedol invariant: always 7 chars")
    }
}

impl core::fmt::Display for Sedol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

const fn sedol_char_value(c: char) -> Option<u32> {
    match c {
        '0'..='9' => Some(c.to_digit(10).expect("Digit char invariant")),
        'B' => Some(11),
        'C' => Some(12),
        'D' => Some(13),
        'F' => Some(14),
        'G' => Some(15),
        'H' => Some(16),
        'J' => Some(17),
        'K' => Some(18),
        'L' => Some(19),
        'M' => Some(20),
        'N' => Some(21),
        'P' => Some(22),
        'Q' => Some(23),
        'R' => Some(24),
        'S' => Some(25),
        'T' => Some(26),
        'V' => Some(27),
        'W' => Some(28),
        'X' => Some(29),
        'Y' => Some(30),
        'Z' => Some(31),
        _ => None,
    }
}

#[allow(clippy::cast_possible_truncation)] // result is always 0-9
fn sedol_check_digit(s: &str) -> u8 {
    let weights: [u8; 7] = [1, 3, 1, 7, 3, 9, 1];
    let mut sum = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(value) = sedol_char_value(c) {
            sum += value * u32::from(weights[i]);
        }
    }

    ((10 - (sum % 10)) % 10) as u8
}

impl core::convert::TryFrom<&str> for Sedol {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Sedol",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        if value.len() != 7 {
            return Err(ValidationError {
                type_name: "Sedol",
                input: String::from(value),
                reason: String::from("SEDOL must be exactly 7 characters"),
            });
        }

        let upper = value.to_uppercase();
        let chars: Vec<char> = upper.chars().collect();

        // Check first 6 characters are valid SEDOL chars (no vowels)
        for &c in &chars[0..6] {
            if sedol_char_value(c).is_none() {
                return Err(ValidationError {
                    type_name: "Sedol",
                    input: String::from(value),
                    reason: String::from("invalid SEDOL character or vowel in positions 1-6"),
                });
            }
        }

        // Last character must be digit
        if !chars[6].is_ascii_digit() {
            return Err(ValidationError {
                type_name: "Sedol",
                input: String::from(value),
                reason: String::from("last character must be a digit"),
            });
        }

        // Check check digit
        let expected_check = sedol_check_digit(&upper[..6]);
        #[allow(clippy::cast_possible_truncation)] // digit is 0-9
        let actual_check = chars[6]
            .to_digit(10)
            .expect("Sedol invariant: last char is digit") as u8;

        if expected_check != actual_check {
            return Err(ValidationError {
                type_name: "Sedol",
                input: String::from(value),
                reason: String::from("checksum validation failed"),
            });
        }

        Ok(Self(upper))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Lei(String);

impl Lei {
    #[must_use]
    pub fn lou_code(&self) -> &str {
        &self.0[..4]
    }

    #[must_use]
    pub fn entity_code(&self) -> &str {
        &self.0[4..18]
    }

    #[must_use]
    pub fn check_digits(&self) -> &str {
        &self.0[18..]
    }
}

impl core::fmt::Display for Lei {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for Lei {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Lei",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        if value.len() != 20 {
            return Err(ValidationError {
                type_name: "Lei",
                input: String::from(value),
                reason: String::from("LEI must be exactly 20 characters"),
            });
        }

        let upper = value.to_uppercase();
        let chars: Vec<char> = upper.chars().collect();

        // First 18 characters must be alphanumeric
        for &c in &chars[0..18] {
            if !c.is_ascii_alphanumeric() {
                return Err(ValidationError {
                    type_name: "Lei",
                    input: String::from(value),
                    reason: String::from("first 18 characters must be alphanumeric"),
                });
            }
        }

        // Last 2 characters must be digits
        for &c in &chars[18..20] {
            if !c.is_ascii_digit() {
                return Err(ValidationError {
                    type_name: "Lei",
                    input: String::from(value),
                    reason: String::from("last 2 characters must be digits"),
                });
            }
        }

        // Check MOD-97 validation
        let mut numeric = String::with_capacity(40);
        for (i, &c) in chars.iter().enumerate() {
            if i >= 18 {
                numeric.push('0');
                numeric.push('0');
            } else if c.is_ascii_digit() {
                numeric.push('0');
                numeric.push(c);
            } else {
                let num = 10 + (c as u8 - b'A');
                numeric.push((b'0' + num / 10) as char);
                numeric.push((b'0' + num % 10) as char);
            }
        }
        let mut remainder = 0u32;
        for c in numeric.chars() {
            let digit = c
                .to_digit(10)
                .expect("LEI numeric expansion invariant: all chars are digits");
            remainder = (remainder * 10 + digit) % 97;
        }
        let expected_check = if remainder == 0 { 98 } else { 98 - remainder };
        let actual_check_str = &upper[18..];
        let actual_check: u32 = actual_check_str
            .parse()
            .expect("LEI invariant: last 2 chars are digits");
        if expected_check != actual_check {
            return Err(ValidationError {
                type_name: "Lei",
                input: String::from(value),
                reason: String::from("checksum validation failed"),
            });
        }

        Ok(Self(upper))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Figi(String);

impl Figi {
    #[must_use]
    pub fn provider_code(&self) -> &str {
        &self.0[..2]
    }

    #[must_use]
    pub fn security_code(&self) -> &str {
        &self.0[2..11]
    }

    /// Returns the check digit character.
    ///
    /// # Panics
    ///
    /// Cannot panic — the constructor guarantees the FIGI is always 12 chars.
    #[must_use]
    pub fn check_digit(&self) -> char {
        self.0
            .chars()
            .nth(11)
            .expect("Figi invariant: always 12 chars")
    }
}

impl core::fmt::Display for Figi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

const fn figi_consonant(c: char) -> bool {
    matches!(c, 'B'..='D' | 'F'..='H' | 'J'..='N' | 'P'..='T' | 'V'..='Z')
}

impl core::convert::TryFrom<&str> for Figi {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Figi",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        if value.len() != 12 {
            return Err(ValidationError {
                type_name: "Figi",
                input: String::from(value),
                reason: String::from("FIGI must be exactly 12 characters"),
            });
        }

        let upper = value.to_uppercase();
        let chars: Vec<char> = upper.chars().collect();

        // First 2 characters must be consonants
        if !figi_consonant(chars[0]) || !figi_consonant(chars[1]) {
            return Err(ValidationError {
                type_name: "Figi",
                input: String::from(value),
                reason: String::from("first 2 characters must be consonants"),
            });
        }

        // Third character must be 'G'
        if chars[2] != 'G' {
            return Err(ValidationError {
                type_name: "Figi",
                input: String::from(value),
                reason: String::from("third character must be 'G'"),
            });
        }

        // Characters 3-10 must be alphanumeric
        for &c in &chars[3..11] {
            if !c.is_ascii_alphanumeric() {
                return Err(ValidationError {
                    type_name: "Figi",
                    input: String::from(value),
                    reason: String::from("characters 4-11 must be alphanumeric"),
                });
            }
        }

        // Last character must be digit
        if !chars[11].is_ascii_digit() {
            return Err(ValidationError {
                type_name: "Figi",
                input: String::from(value),
                reason: String::from("last character must be a digit"),
            });
        }

        // Luhn validation on numeric expansion
        let expansion = isin_numeric_expansion(&upper);
        if !luhn_valid(&expansion) {
            return Err(ValidationError {
                type_name: "Figi",
                input: String::from(value),
                reason: String::from("Luhn checksum validation failed"),
            });
        }

        Ok(Self(upper))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Mic([u8; 4]);

impl Mic {
    /// Returns the MIC as a string slice.
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid UTF-8.
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("Mic invariant: always valid UTF-8")
    }
}

impl core::fmt::Display for Mic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::convert::TryFrom<&str> for Mic {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(ValidationError {
                type_name: "Mic",
                input: String::from(value),
                reason: String::from("MIC must be exactly 4 characters"),
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        for &b in bytes {
            if !b.is_ascii_uppercase() {
                return Err(ValidationError {
                    type_name: "Mic",
                    input: String::from(value),
                    reason: String::from("MIC must contain only uppercase letters"),
                });
            }
        }

        // Check against whitelist
        if !is_valid_mic_codes(&upper) {
            return Err(ValidationError {
                type_name: "Mic",
                input: String::from(value),
                reason: String::from("not a valid ISO 10383 MIC"),
            });
        }

        let mut arr = [0u8; 4];
        arr.copy_from_slice(bytes);

        Ok(Self(arr))
    }
}

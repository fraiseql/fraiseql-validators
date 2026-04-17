//! Validation for barcode and product identifier types.
//!
//! This module provides validators for EAN-8, EAN-13, UPC-A, ISBN-13, ISSN,
//! and GTIN-14 identifiers.

use crate::ValidationError;
use alloc::string::String;
use core::fmt;

/// GS1 check digit validator for barcodes
fn gs1_check_digit_valid(digits: &str) -> bool {
    crate::checksum::gs1_check_digit_valid(digits)
}

/// EAN-8: exactly 8 digits, GS1 check digit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ean8([u8; 8]);

impl Ean8 {
    /// Returns the check digit as a char.
    #[must_use]
    pub const fn check_digit(&self) -> char {
        self.0[7] as char
    }
}

impl fmt::Display for Ean8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in &self.0 {
            write!(f, "{}", digit as char)?;
        }
        Ok(())
    }
}

impl core::convert::TryFrom<&str> for Ean8 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(ValidationError {
                type_name: "Ean8",
                input: String::from(value),
                reason: String::from("must be exactly 8 characters"),
            });
        }

        let mut digits = [0u8; 8];
        for (i, c) in value.chars().enumerate() {
            if c.is_ascii_digit() {
                digits[i] = c as u8;
            } else {
                return Err(ValidationError {
                    type_name: "Ean8",
                    input: String::from(value),
                    reason: String::from("must contain only digits"),
                });
            }
        }

        if !gs1_check_digit_valid(value) {
            return Err(ValidationError {
                type_name: "Ean8",
                input: String::from(value),
                reason: String::from("invalid check digit"),
            });
        }

        Ok(Self(digits))
    }
}

/// EAN-13: exactly 13 digits, GS1 check digit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ean13([u8; 13]);

impl Ean13 {
    /// Returns the GS1 company prefix (first 3 digits).
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid ASCII digits.
    #[must_use]
    pub fn gs1_prefix(&self) -> &str {
        core::str::from_utf8(&self.0[0..3]).unwrap()
    }
}

impl fmt::Display for Ean13 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in &self.0 {
            write!(f, "{}", digit as char)?;
        }
        Ok(())
    }
}

impl core::convert::TryFrom<&str> for Ean13 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 13 {
            return Err(ValidationError {
                type_name: "Ean13",
                input: String::from(value),
                reason: String::from("must be exactly 13 characters"),
            });
        }

        let mut digits = [0u8; 13];
        for (i, c) in value.chars().enumerate() {
            if c.is_ascii_digit() {
                digits[i] = c as u8;
            } else {
                return Err(ValidationError {
                    type_name: "Ean13",
                    input: String::from(value),
                    reason: String::from("must contain only digits"),
                });
            }
        }

        if !gs1_check_digit_valid(value) {
            return Err(ValidationError {
                type_name: "Ean13",
                input: String::from(value),
                reason: String::from("invalid check digit"),
            });
        }

        Ok(Self(digits))
    }
}

/// UPC-A: exactly 12 digits, GS1 check digit. (Structurally EAN-13 with implicit leading 0.)
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UpcA([u8; 12]);

impl UpcA {
    /// Converts to EAN-13 by prepending "0".
    #[must_use]
    pub fn to_ean13(&self) -> Ean13 {
        let mut ean13_digits = [b'0'; 13]; // Start with '0'
        ean13_digits[1..].copy_from_slice(&self.0);
        Ean13(ean13_digits)
    }
}

impl fmt::Display for UpcA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in &self.0 {
            write!(f, "{}", digit as char)?;
        }
        Ok(())
    }
}

impl core::convert::TryFrom<&str> for UpcA {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 12 {
            return Err(ValidationError {
                type_name: "UpcA",
                input: String::from(value),
                reason: String::from("must be exactly 12 characters"),
            });
        }

        let mut digits = [0u8; 12];
        for (i, c) in value.chars().enumerate() {
            if c.is_ascii_digit() {
                digits[i] = c as u8;
            } else {
                return Err(ValidationError {
                    type_name: "UpcA",
                    input: String::from(value),
                    reason: String::from("must contain only digits"),
                });
            }
        }

        if !gs1_check_digit_valid(value) {
            return Err(ValidationError {
                type_name: "UpcA",
                input: String::from(value),
                reason: String::from("invalid check digit"),
            });
        }

        Ok(Self(digits))
    }
}

/// ISBN-13 is EAN-13 restricted to prefixes 978 or 979.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Isbn13([u8; 13]);

impl Isbn13 {
    /// Returns the registration group element (chars 4–4, simplified).
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid ASCII digits.
    #[must_use]
    pub fn registration_group(&self) -> &str {
        core::str::from_utf8(&self.0[3..4]).unwrap()
    }

    /// Converts to EAN-13.
    #[must_use]
    pub const fn as_ean13(&self) -> Ean13 {
        Ean13(self.0)
    }
}

impl fmt::Display for Isbn13 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in &self.0 {
            write!(f, "{}", digit as char)?;
        }
        Ok(())
    }
}

impl core::convert::TryFrom<&str> for Isbn13 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ean13 = Ean13::try_from(value)?;

        // Check prefix is 978 or 979
        let prefix = ean13.gs1_prefix();
        if prefix != "978" && prefix != "979" {
            return Err(ValidationError {
                type_name: "Isbn13",
                input: String::from(value),
                reason: String::from("ISBN-13 must have prefix 978 or 979"),
            });
        }

        // Transmute since memory layout is identical
        Ok(Self(ean13.0))
    }
}

/// ISSN: exactly 8 chars — 7 digits + check char (digit or 'X').
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Issn([u8; 8]);

impl Issn {
    // No specific accessors needed
}

impl fmt::Display for Issn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Render as XXXX-XXXX
        write!(
            f,
            "{}{}{}{}-{}{}{}{}",
            self.0[0] as char,
            self.0[1] as char,
            self.0[2] as char,
            self.0[3] as char,
            self.0[4] as char,
            self.0[5] as char,
            self.0[6] as char,
            self.0[7] as char
        )
    }
}

impl core::convert::TryFrom<&str> for Issn {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Strip hyphens
        let stripped: String = value.chars().filter(|&c| c != '-').collect();

        if stripped.len() != 8 {
            return Err(ValidationError {
                type_name: "Issn",
                input: String::from(value),
                reason: String::from("must be exactly 8 characters (excluding hyphens)"),
            });
        }

        let mut digits = [0u8; 8];
        for (i, c) in stripped.chars().enumerate() {
            if i < 7 {
                if let Some(d) = c.to_digit(10) {
                    #[allow(clippy::cast_possible_truncation)] // d is 0-9
                    let byte = d as u8;
                    digits[i] = byte + b'0'; // Store as ASCII
                } else {
                    return Err(ValidationError {
                        type_name: "Issn",
                        input: String::from(value),
                        reason: String::from("first 7 characters must be digits"),
                    });
                }
            } else {
                // Last char: digit or 'X'
                if c.is_ascii_digit() {
                    digits[i] = c as u8;
                } else if c == 'X' {
                    digits[i] = b'X';
                } else {
                    return Err(ValidationError {
                        type_name: "Issn",
                        input: String::from(value),
                        reason: String::from("last character must be digit or 'X'"),
                    });
                }
            }
        }

        // Weighted sum check: weights 8,7,6,5,4,3,2 for first 7 digits
        let mut sum = 0u32;
        for (i, &d) in digits[..7].iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)] // i is 0-6
            let weight = 8 - i as u32;
            let digit = u32::from(d - b'0');
            sum += digit * weight;
        }

        let check = (11 - (sum % 11)) % 11;
        let expected_check = if check == 10 {
            b'X'
        } else {
            #[allow(clippy::cast_possible_truncation)] // check is 0-9
            let byte = check as u8;
            b'0' + byte
        };

        if digits[7] != expected_check {
            return Err(ValidationError {
                type_name: "Issn",
                input: String::from(value),
                reason: String::from("invalid check character"),
            });
        }

        Ok(Self(digits))
    }
}

/// GTIN-14: exactly 14 digits, GS1 check digit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gtin14([u8; 14]);

impl Gtin14 {
    /// Returns the indicator digit (first char, packaging level 0-8).
    #[must_use]
    pub const fn indicator_digit(&self) -> char {
        self.0[0] as char
    }

    /// Converts to EAN-13 if indicator digit is 0.
    #[must_use]
    pub fn as_ean13(&self) -> Option<Ean13> {
        if self.0[0] == b'0' {
            let mut ean13_digits = [0u8; 13];
            ean13_digits.copy_from_slice(&self.0[1..]);
            Some(Ean13(ean13_digits))
        } else {
            None
        }
    }
}

impl fmt::Display for Gtin14 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in &self.0 {
            write!(f, "{}", digit as char)?;
        }
        Ok(())
    }
}

impl core::convert::TryFrom<&str> for Gtin14 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 14 {
            return Err(ValidationError {
                type_name: "Gtin14",
                input: String::from(value),
                reason: String::from("must be exactly 14 characters"),
            });
        }

        let mut digits = [0u8; 14];
        for (i, c) in value.chars().enumerate() {
            if c.is_ascii_digit() {
                digits[i] = c as u8;
            } else {
                return Err(ValidationError {
                    type_name: "Gtin14",
                    input: String::from(value),
                    reason: String::from("must contain only digits"),
                });
            }
        }

        if !gs1_check_digit_valid(value) {
            return Err(ValidationError {
                type_name: "Gtin14",
                input: String::from(value),
                reason: String::from("invalid check digit"),
            });
        }

        Ok(Self(digits))
    }
}

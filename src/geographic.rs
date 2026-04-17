//! Validation for geographic and locale types.
//!
//! This module provides validators for country codes, language codes, postal codes,
//! geographic coordinates, airport codes, and timezones.

use crate::ValidationError;
use alloc::string::String;
use core::fmt;

// Include the generated code from build.rs
include!(concat!(env!("OUT_DIR"), "/country_codes.rs"));
include!(concat!(env!("OUT_DIR"), "/language_codes.rs"));

/// Country code (ISO 3166-1 alpha-2): exactly 2 uppercase letters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CountryCode([u8; 2]);

impl CountryCode {
    /// Returns the country code as a string slice.
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).unwrap()
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::convert::TryFrom<&str> for CountryCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(ValidationError {
                type_name: "CountryCode",
                input: String::from(value),
                reason: "must be exactly 2 characters",
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        if !bytes.iter().all(|&b| b.is_ascii_uppercase()) {
            return Err(ValidationError {
                type_name: "CountryCode",
                input: String::from(value),
                reason: "must contain only uppercase letters",
            });
        }

        // Check if it's in the whitelist
        if !is_valid_country_codes_alpha2(&upper) {
            return Err(ValidationError {
                type_name: "CountryCode",
                input: String::from(value),
                reason: "not a valid ISO 3166-1 alpha-2 country code",
            });
        }

        let mut result = [0u8; 2];
        result.copy_from_slice(bytes);
        Ok(CountryCode(result))
    }
}

/// Country code (ISO 3166-1 alpha-3): exactly 3 uppercase letters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CountryCodeAlpha3([u8; 3]);

impl CountryCodeAlpha3 {
    /// Returns the country code as a string slice.
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).unwrap()
    }
}

impl fmt::Display for CountryCodeAlpha3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::convert::TryFrom<&str> for CountryCodeAlpha3 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(ValidationError {
                type_name: "CountryCodeAlpha3",
                input: String::from(value),
                reason: "must be exactly 3 characters",
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        if !bytes.iter().all(|&b| b.is_ascii_uppercase()) {
            return Err(ValidationError {
                type_name: "CountryCodeAlpha3",
                input: String::from(value),
                reason: "must contain only uppercase letters",
            });
        }

        // Check if it's in the whitelist
        if !is_valid_country_codes_alpha3(&upper) {
            return Err(ValidationError {
                type_name: "CountryCodeAlpha3",
                input: String::from(value),
                reason: "not a valid ISO 3166-1 alpha-3 country code",
            });
        }

        let mut result = [0u8; 3];
        result.copy_from_slice(bytes);
        Ok(CountryCodeAlpha3(result))
    }
}

/// Language code: IANA primary language subtags (ISO 639-1 and -2/3).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LanguageCode(String);

impl LanguageCode {
    /// Returns the primary language subtag.
    pub fn primary(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for LanguageCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "LanguageCode",
                input: String::from(value),
                reason: "empty string",
            });
        }

        if value.len() < 2 || value.len() > 3 {
            return Err(ValidationError {
                type_name: "LanguageCode",
                input: String::from(value),
                reason: "must be 2-3 characters",
            });
        }

        let lower = value.to_lowercase();

        if !lower.chars().all(|c| c.is_ascii_lowercase()) {
            return Err(ValidationError {
                type_name: "LanguageCode",
                input: String::from(value),
                reason: "must contain only lowercase letters",
            });
        }

        // Check if it's in the IANA whitelist
        if !is_valid_language_codes(&lower) {
            return Err(ValidationError {
                type_name: "LanguageCode",
                input: String::from(value),
                reason: "not a valid IANA language subtag",
            });
        }

        Ok(LanguageCode(lower))
    }
}

/// Latitude: decimal degrees in range [-90.0, 90.0].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Latitude(f64);

impl Latitude {
    /// Returns the latitude value in degrees.
    pub fn degrees(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6}", self.0)
    }
}

impl core::convert::TryFrom<f64> for Latitude {
    type Error = ValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(ValidationError {
                type_name: "Latitude",
                input: alloc::format!("{}", value),
                reason: "must be a finite number",
            });
        }

        if value < -90.0 || value > 90.0 {
            return Err(ValidationError {
                type_name: "Latitude",
                input: alloc::format!("{}", value),
                reason: "must be in range [-90.0, 90.0]",
            });
        }

        Ok(Latitude(value))
    }
}

impl core::convert::TryFrom<&str> for Latitude {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<f64>() {
            Ok(num) => Latitude::try_from(num),
            Err(_) => Err(ValidationError {
                type_name: "Latitude",
                input: String::from(value),
                reason: "must be a valid number",
            }),
        }
    }
}

/// Longitude: decimal degrees in range [-180.0, 180.0].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Longitude(f64);

impl Longitude {
    /// Returns the longitude value in degrees.
    pub fn degrees(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6}", self.0)
    }
}

impl core::convert::TryFrom<f64> for Longitude {
    type Error = ValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(ValidationError {
                type_name: "Longitude",
                input: alloc::format!("{}", value),
                reason: "must be a finite number",
            });
        }

        if value < -180.0 || value > 180.0 {
            return Err(ValidationError {
                type_name: "Longitude",
                input: alloc::format!("{}", value),
                reason: "must be in range [-180.0, 180.0]",
            });
        }

        Ok(Longitude(value))
    }
}

impl core::convert::TryFrom<&str> for Longitude {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<f64>() {
            Ok(num) => Longitude::try_from(num),
            Err(_) => Err(ValidationError {
                type_name: "Longitude",
                input: String::from(value),
                reason: "must be a valid number",
            }),
        }
    }
}
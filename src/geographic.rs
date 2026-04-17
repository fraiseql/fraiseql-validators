//! Validation for geographic and locale types.
//!
//! This module provides validators for country codes, language codes, postal codes,
//! geographic coordinates, airport codes, and timezones.

use crate::ValidationError;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use once_cell::sync::Lazy;
use regex_lite::Regex;

// Include the generated code from build.rs
include!(concat!(env!("OUT_DIR"), "/country_codes.rs"));
include!(concat!(env!("OUT_DIR"), "/language_codes.rs"));
include!(concat!(env!("OUT_DIR"), "/airport_codes.rs"));
include!(concat!(env!("OUT_DIR"), "/timezone_codes.rs"));

// Postal code validation regexes
static POSTAL_CODE_US: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{5}(-\d{4})?$").unwrap());
static POSTAL_CODE_GB: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z]{1,2}\d[A-Z\d]?\s?\d[A-Z]{2}$").unwrap());
static POSTAL_CODE_FR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{5}$").unwrap());
static POSTAL_CODE_DE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{5}$").unwrap());
static POSTAL_CODE_CA: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z]\d[A-Z]\s?\d[A-Z]\d$").unwrap());
static POSTAL_CODE_AU: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4}$").unwrap());
static POSTAL_CODE_NL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4}\s?[A-Z]{2}$").unwrap());
static POSTAL_CODE_JP: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{3}-?\d{4}$").unwrap());
static POSTAL_CODE_CH: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4}$").unwrap());
static POSTAL_CODE_ES: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{5}$").unwrap());

/// Country code (ISO 3166-1 alpha-2): exactly 2 uppercase letters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CountryCode([u8; 2]);

impl CountryCode {
    /// Returns the country code as a string slice.
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid ASCII.
    #[must_use]
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
                reason: String::from("must be exactly 2 characters"),
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        if !bytes.iter().all(u8::is_ascii_uppercase) {
            return Err(ValidationError {
                type_name: "CountryCode",
                input: String::from(value),
                reason: String::from("must contain only uppercase letters"),
            });
        }

        // Check if it's in the whitelist
        if !is_valid_country_codes_alpha2(&upper) {
            return Err(ValidationError {
                type_name: "CountryCode",
                input: String::from(value),
                reason: String::from("not a valid ISO 3166-1 alpha-2 country code"),
            });
        }

        let mut result = [0u8; 2];
        result.copy_from_slice(bytes);
        Ok(Self(result))
    }
}

/// Country code (ISO 3166-1 alpha-3): exactly 3 uppercase letters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CountryCodeAlpha3([u8; 3]);

impl CountryCodeAlpha3 {
    /// Returns the country code as a string slice.
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid ASCII.
    #[must_use]
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
                reason: String::from("must be exactly 3 characters"),
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        if !bytes.iter().all(u8::is_ascii_uppercase) {
            return Err(ValidationError {
                type_name: "CountryCodeAlpha3",
                input: String::from(value),
                reason: String::from("must contain only uppercase letters"),
            });
        }

        // Check if it's in the whitelist
        if !is_valid_country_codes_alpha3(&upper) {
            return Err(ValidationError {
                type_name: "CountryCodeAlpha3",
                input: String::from(value),
                reason: String::from("not a valid ISO 3166-1 alpha-3 country code"),
            });
        }

        let mut result = [0u8; 3];
        result.copy_from_slice(bytes);
        Ok(Self(result))
    }
}

/// Language code: IANA primary language subtags (ISO 639-1 and -2/3).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LanguageCode(String);

impl LanguageCode {
    /// Returns the primary language subtag.
    #[must_use]
    pub const fn primary(&self) -> &str {
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
                reason: String::from("empty string"),
            });
        }

        if value.len() < 2 || value.len() > 3 {
            return Err(ValidationError {
                type_name: "LanguageCode",
                input: String::from(value),
                reason: String::from("must be 2-3 characters"),
            });
        }

        let lower = value.to_lowercase();

        if !lower.chars().all(|c| c.is_ascii_lowercase()) {
            return Err(ValidationError {
                type_name: "LanguageCode",
                input: String::from(value),
                reason: String::from("must contain only lowercase letters"),
            });
        }

        // Check if it's in the IANA whitelist
        if !is_valid_language_codes(&lower) {
            return Err(ValidationError {
                type_name: "LanguageCode",
                input: String::from(value),
                reason: String::from("not a valid IANA language subtag"),
            });
        }

        Ok(Self(lower))
    }
}

/// Latitude: decimal degrees in range [-90.0, 90.0].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Latitude(f64);

impl Latitude {
    /// Returns the latitude value in degrees.
    #[must_use]
    pub const fn degrees(&self) -> f64 {
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
                input: alloc::format!("{value}"),
                reason: String::from("must be a finite number"),
            });
        }

        if !(-90.0..=90.0).contains(&value) {
            return Err(ValidationError {
                type_name: "Latitude",
                input: alloc::format!("{value}"),
                reason: String::from("must be in range [-90.0, 90.0]"),
            });
        }

        Ok(Self(value))
    }
}

impl core::convert::TryFrom<&str> for Latitude {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<f64>().map_or_else(
            |_| {
                Err(ValidationError {
                    type_name: "Latitude",
                    input: String::from(value),
                    reason: String::from("must be a valid number"),
                })
            },
            Self::try_from,
        )
    }
}

/// Longitude: decimal degrees in range [-180.0, 180.0].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Longitude(f64);

impl Longitude {
    /// Returns the longitude value in degrees.
    #[must_use]
    pub const fn degrees(&self) -> f64 {
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
                input: alloc::format!("{value}"),
                reason: String::from("must be a finite number"),
            });
        }

        if !(-180.0..=180.0).contains(&value) {
            return Err(ValidationError {
                type_name: "Longitude",
                input: alloc::format!("{value}"),
                reason: String::from("must be in range [-180.0, 180.0]"),
            });
        }

        Ok(Self(value))
    }
}

impl core::convert::TryFrom<&str> for Longitude {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<f64>().map_or_else(
            |_| {
                Err(ValidationError {
                    type_name: "Longitude",
                    input: String::from(value),
                    reason: String::from("must be a valid number"),
                })
            },
            Self::try_from,
        )
    }
}

/// Postal code: country-specific validation in format CC:CODE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostalCode {
    country: [u8; 2],
    code: String,
}

impl PostalCode {
    /// Returns the country code (uppercase).
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid ASCII.
    #[must_use]
    pub fn country(&self) -> &str {
        core::str::from_utf8(&self.country).unwrap()
    }

    /// Returns the postal code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Checks if this postal code belongs to the given country.
    #[must_use]
    pub fn belongs_to_country(&self, country: &str) -> bool {
        self.country() == country.to_uppercase()
    }
}

impl fmt::Display for PostalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.country(), self.code)
    }
}

impl core::convert::TryFrom<&str> for PostalCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(ValidationError {
                type_name: "PostalCode",
                input: String::from(value),
                reason: String::from("invalid format, expected COUNTRY:CODE"),
            });
        }

        let country_str = parts[0];
        let code_str = parts[1];

        if country_str.len() != 2 {
            return Err(ValidationError {
                type_name: "PostalCode",
                input: String::from(value),
                reason: String::from("country code must be exactly 2 characters"),
            });
        }

        let upper_country = country_str.to_uppercase();
        let country_bytes = upper_country.as_bytes();

        if !country_bytes.iter().all(u8::is_ascii_uppercase) {
            return Err(ValidationError {
                type_name: "PostalCode",
                input: String::from(value),
                reason: String::from("country code must contain only uppercase letters"),
            });
        }

        if code_str.is_empty() {
            return Err(ValidationError {
                type_name: "PostalCode",
                input: String::from(value),
                reason: String::from("postal code cannot be empty"),
            });
        }

        let normalized_code = code_str.replace(' ', "");

        // Validate the code based on country
        let is_valid_code = match upper_country.as_str() {
            "US" => POSTAL_CODE_US.is_match(&normalized_code),
            "GB" => POSTAL_CODE_GB.is_match(code_str), // GB allows spaces in input
            "FR" => POSTAL_CODE_FR.is_match(&normalized_code),
            "DE" => POSTAL_CODE_DE.is_match(&normalized_code),
            "CA" => POSTAL_CODE_CA.is_match(&normalized_code),
            "AU" => POSTAL_CODE_AU.is_match(&normalized_code),
            "NL" => POSTAL_CODE_NL.is_match(&normalized_code),
            "JP" => POSTAL_CODE_JP.is_match(&normalized_code),
            "CH" => POSTAL_CODE_CH.is_match(&normalized_code),
            "ES" => POSTAL_CODE_ES.is_match(&normalized_code),
            _ => true, // Accept any non-empty code for unknown countries
        };

        if !is_valid_code {
            return Err(ValidationError {
                type_name: "PostalCode",
                input: String::from(value),
                reason: alloc::format!("invalid postal code format for country {upper_country}"),
            });
        }

        let mut country = [0u8; 2];
        country.copy_from_slice(country_bytes);

        Ok(Self {
            country,
            code: normalized_code,
        })
    }
}

/// IATA airport code: exactly 3 uppercase letters, whitelisted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IataAirportCode([u8; 3]);

impl IataAirportCode {
    /// Returns the airport code as a string slice.
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid ASCII.
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).unwrap()
    }
}

impl fmt::Display for IataAirportCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::convert::TryFrom<&str> for IataAirportCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(ValidationError {
                type_name: "IataAirportCode",
                input: String::from(value),
                reason: String::from("must be exactly 3 characters"),
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        if !bytes.iter().all(u8::is_ascii_uppercase) {
            return Err(ValidationError {
                type_name: "IataAirportCode",
                input: String::from(value),
                reason: String::from("must contain only uppercase letters"),
            });
        }

        // Check if it's in the whitelist
        if !is_valid_iata_airport_codes(&upper) {
            return Err(ValidationError {
                type_name: "IataAirportCode",
                input: String::from(value),
                reason: String::from("not a valid IATA airport code"),
            });
        }

        let mut result = [0u8; 3];
        result.copy_from_slice(bytes);
        Ok(Self(result))
    }
}

/// ICAO airport code: exactly 4 uppercase alphanumeric chars, whitelisted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IcaoAirportCode([u8; 4]);

impl IcaoAirportCode {
    /// Returns the airport code as a string slice.
    ///
    /// # Panics
    ///
    /// Cannot panic — the inner array is always valid ASCII.
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).unwrap()
    }
}

impl fmt::Display for IcaoAirportCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::convert::TryFrom<&str> for IcaoAirportCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(ValidationError {
                type_name: "IcaoAirportCode",
                input: String::from(value),
                reason: String::from("must be exactly 4 characters"),
            });
        }

        let upper = value.to_uppercase();
        let bytes = upper.as_bytes();

        if !bytes
            .iter()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Err(ValidationError {
                type_name: "IcaoAirportCode",
                input: String::from(value),
                reason: String::from("must contain only uppercase letters and digits"),
            });
        }

        // Check if it's in the whitelist
        if !is_valid_icao_airport_codes(&upper) {
            return Err(ValidationError {
                type_name: "IcaoAirportCode",
                input: String::from(value),
                reason: String::from("not a valid ICAO airport code"),
            });
        }

        let mut result = [0u8; 4];
        result.copy_from_slice(bytes);
        Ok(Self(result))
    }
}

/// IANA timezone name: validated against zone.tab.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IanaTimezone(String);

impl IanaTimezone {
    /// Returns the timezone name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IanaTimezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for IanaTimezone {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "IanaTimezone",
                input: String::from(value),
                reason: String::from("empty string"),
            });
        }

        // Check if it's in the IANA whitelist
        if !is_valid_iana_timezones(value) {
            return Err(ValidationError {
                type_name: "IanaTimezone",
                input: String::from(value),
                reason: String::from("not a valid IANA timezone"),
            });
        }

        Ok(Self(String::from(value)))
    }
}

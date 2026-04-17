//! Validation for identifier types.
//!
//! This module provides validators for semver versions, URL slugs, colors,
//! locale identifiers, and vehicle identification numbers.

use crate::ValidationError;
use alloc::string::String;
use core::fmt;

/// Slug: lowercase alphanum + hyphens, no leading/trailing hyphen, min 1 char.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Slug(String);

impl Slug {
    // No specific accessors needed
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for Slug {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Slug",
                input: String::from(value),
                reason: "empty string",
            });
        }

        if value.starts_with('-') {
            return Err(ValidationError {
                type_name: "Slug",
                input: String::from(value),
                reason: "starts with hyphen",
            });
        }

        if value.ends_with('-') {
            return Err(ValidationError {
                type_name: "Slug",
                input: String::from(value),
                reason: "ends with hyphen",
            });
        }

        let mut prev_was_hyphen = false;
        for c in value.chars() {
            if c == '-' {
                if prev_was_hyphen {
                    return Err(ValidationError {
                        type_name: "Slug",
                        input: String::from(value),
                        reason: "consecutive hyphens",
                    });
                }
                prev_was_hyphen = true;
            } else {
                if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
                    return Err(ValidationError {
                        type_name: "Slug",
                        input: String::from(value),
                        reason: "contains invalid character (must be lowercase letters, digits, or hyphens)",
                    });
                }
                prev_was_hyphen = false;
            }
        }

        Ok(Slug(String::from(value)))
    }
}

/// Color: accepts `#RRGGBB` or `#RGB`; stores as packed 24-bit `u32`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color(u32);  // packed: 0x00RRGGBB

impl Color {
    /// Returns the red component (0-255).
    pub fn red(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Returns the green component (0-255).
    pub fn green(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Returns the blue component (0-255).
    pub fn blue(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns the color as a hex string in canonical `#RRGGBB` format.
    pub fn to_hex(&self) -> String {
        alloc::format!("#{:02X}{:02X}{:02X}", self.red(), self.green(), self.blue())
    }

    /// Returns the relative luminance in [0.0, 1.0] per WCAG 2.1.
    pub fn luminance(&self) -> f64 {
        // WCAG 2.1 relative luminance: https://www.w3.org/TR/WCAG21/#dfn-relative-luminance
        let r = self.red() as f64 / 255.0;
        let g = self.green() as f64 / 255.0;
        let b = self.blue() as f64 / 255.0;

        // Linearize each channel
        let r_linear = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
        let g_linear = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
        let b_linear = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };

        // Combine with luminance coefficients
        0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl core::convert::TryFrom<&str> for Color {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with('#') {
            return Err(ValidationError {
                type_name: "Color",
                input: String::from(value),
                reason: "must start with '#'",
            });
        }

        let hex_part = &value[1..];
        let hex_len = hex_part.len();

        let (r, g, b) = match hex_len {
            3 => {
                // #RGB format - expand each nibble
                let r_nibble = hex_part.chars().nth(0).unwrap();
                let g_nibble = hex_part.chars().nth(1).unwrap();
                let b_nibble = hex_part.chars().nth(2).unwrap();

                if !r_nibble.is_ascii_hexdigit() || !g_nibble.is_ascii_hexdigit() || !b_nibble.is_ascii_hexdigit() {
                    return Err(ValidationError {
                        type_name: "Color",
                        input: String::from(value),
                        reason: "invalid hex digits",
                    });
                }

                let r = (hex_to_u8(r_nibble) as u16 * 17) as u8;
                let g = (hex_to_u8(g_nibble) as u16 * 17) as u8;
                let b = (hex_to_u8(b_nibble) as u16 * 17) as u8;
                (r, g, b)
            }
            6 => {
                // #RRGGBB format
                if hex_part.chars().any(|c| !c.is_ascii_hexdigit()) {
                    return Err(ValidationError {
                        type_name: "Color",
                        input: String::from(value),
                        reason: "invalid hex digits",
                    });
                }

                let r = hex_pair_to_u8(&hex_part[0..2]);
                let g = hex_pair_to_u8(&hex_part[2..4]);
                let b = hex_pair_to_u8(&hex_part[4..6]);
                (r, g, b)
            }
            _ => {
                return Err(ValidationError {
                    type_name: "Color",
                    input: String::from(value),
                    reason: "must be 3 or 6 hex digits after '#'",
                });
            }
        };

        Ok(Color(((r as u32) << 16) | ((g as u32) << 8) | (b as u32)))
    }
}

/// Convert a single hex char to u8 value.
fn hex_to_u8(c: char) -> u8 {
    let upper = c.to_ascii_uppercase();
    match upper {
        '0'..='9' => (upper as u8) - b'0',
        'A'..='F' => 10 + (upper as u8 - b'A'),
        _ => 0, // Should not happen due to validation
    }
}

/// Convert a hex pair (2 chars) to u8 value.
fn hex_pair_to_u8(pair: &str) -> u8 {
    let high = hex_to_u8(pair.chars().nth(0).unwrap());
    let low = hex_to_u8(pair.chars().nth(1).unwrap());
    (high << 4) | low
}

/// Locale: BCP 47 locale tag, simplified for practical use.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Locale(String);

impl Locale {
    /// Returns the language subtag (lowercase).
    pub fn language(&self) -> &str {
        self.0.split('-').next().unwrap()
    }

    /// Returns the region subtag if present (uppercase).
    pub fn region(&self) -> Option<&str> {
        for part in self.0.split('-') {
            if part.len() == 2 && part.chars().all(|c| c.is_ascii_uppercase()) {
                return Some(part);
            }
            if part.len() == 3 && part.chars().all(|c| c.is_ascii_digit()) {
                return Some(part);
            }
        }
        None
    }

    /// Returns the script subtag if present (titlecase).
    pub fn script(&self) -> Option<&str> {
        for part in self.0.split('-') {
            if part.len() == 4 && part.chars().next().unwrap().is_ascii_uppercase()
                && part.chars().skip(1).all(|c| c.is_ascii_lowercase()) {
                return Some(part);
            }
        }
        None
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::convert::TryFrom<&str> for Locale {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Locale",
                input: String::from(value),
                reason: "empty string",
            });
        }

        // Basic BCP 47 structure check: language[-script][-region][-variant]*
        // Language: 2-3 lowercase letters
        // Script: 4 chars, titlecase
        // Region: 2 uppercase or 3 digits
        // Variant: 5-8 alphanum starting with digit or 4 chars starting with digit

        let parts: alloc::vec::Vec<&str> = value.split('-').collect();
        if parts.is_empty() {
            return Err(ValidationError {
                type_name: "Locale",
                input: String::from(value),
                reason: "invalid format",
            });
        }

        // Check language (first part)
        let lang = parts[0];
        if !(lang.len() >= 2 && lang.len() <= 3 && lang.chars().all(|c| c.is_ascii_lowercase())) {
            return Err(ValidationError {
                type_name: "Locale",
                input: String::from(value),
                reason: "language must be 2-3 lowercase letters",
            });
        }

        // Check remaining parts
        let mut i = 1;
        while i < parts.len() {
            let part = parts[i];
            let len = part.len();

            if len == 4 {
                // Could be script or variant
                if part.chars().next().unwrap().is_ascii_uppercase()
                    && part.chars().skip(1).all(|c| c.is_ascii_lowercase()) {
                    // Script
                    i += 1;
                    continue;
                } else if part.chars().next().unwrap().is_ascii_digit()
                    && part.chars().all(|c| c.is_ascii_alphanumeric()) {
                    // Variant
                    i += 1;
                    continue;
                }
            } else if len == 2 {
                // Region (uppercase)
                if part.chars().all(|c| c.is_ascii_uppercase()) {
                    i += 1;
                    continue;
                }
            } else if len == 3 {
                // Region (digits)
                if part.chars().all(|c| c.is_ascii_digit()) {
                    i += 1;
                    continue;
                }
            } else if len >= 5 && len <= 8 {
                // Variant
                if part.chars().all(|c| c.is_ascii_alphanumeric()) {
                    i += 1;
                    continue;
                }
            }

            return Err(ValidationError {
                type_name: "Locale",
                input: String::from(value),
                reason: "invalid subtag format",
            });
        }

        Ok(Locale(String::from(value)))
    }
}

/// SemVer 2.0.0: MAJOR.MINOR.PATCH[-pre][+build]
#[derive(Clone, Debug)]
pub struct Semver {
    major: u64,
    minor: u64,
    patch: u64,
    pre_release: Option<String>,
    build_metadata: Option<String>,
}

impl Semver {
    /// Returns the major version.
    pub fn major(&self) -> u64 {
        self.major
    }

    /// Returns the minor version.
    pub fn minor(&self) -> u64 {
        self.minor
    }

    /// Returns the patch version.
    pub fn patch(&self) -> u64 {
        self.patch
    }

    /// Returns the pre-release identifier if present.
    pub fn pre_release(&self) -> Option<&str> {
        self.pre_release.as_deref()
    }

    /// Returns the build metadata if present.
    pub fn build_metadata(&self) -> Option<&str> {
        self.build_metadata.as_deref()
    }

    /// Checks if this version is compatible with the given base version.
    /// Compatible means same major.minor, patch >= base.patch.
    pub fn compatible_with(&self, base: &Semver) -> bool {
        self.major == base.major && self.minor == base.minor && self.patch >= base.patch
    }

    /// Checks if this version is caret-compatible with the given base version.
    /// Compatible means same major, minor.patch >= base.minor.patch.
    pub fn caret_compatible_with(&self, base: &Semver) -> bool {
        self.major == base.major &&
        (self.minor > base.minor || (self.minor == base.minor && self.patch >= base.patch))
    }
}

impl PartialEq for Semver {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major &&
        self.minor == other.minor &&
        self.patch == other.patch &&
        self.pre_release == other.pre_release
        // build_metadata ignored for equality
    }
}

impl Eq for Semver {}

impl PartialOrd for Semver {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Semver {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // 1. Compare major, minor, patch numerically
        let core_cmp = (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch));

        if core_cmp != core::cmp::Ordering::Equal {
            return core_cmp;
        }

        // 2. Pre-release: version with pre-release < version without
        match (&self.pre_release, &other.pre_release) {
            (None, Some(_)) => return core::cmp::Ordering::Greater,
            (Some(_), None) => return core::cmp::Ordering::Less,
            (Some(a), Some(b)) => {
                let pre_cmp = compare_pre_release(a, b);
                if pre_cmp != core::cmp::Ordering::Equal {
                    return pre_cmp;
                }
            }
            (None, None) => {}
        }

        // 3. Build metadata is ignored in ordering
        core::cmp::Ordering::Equal
    }
}

impl fmt::Display for Semver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        if let Some(build) = &self.build_metadata {
            write!(f, "+{}", build)?;
        }
        Ok(())
    }
}

impl core::convert::TryFrom<&str> for Semver {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError {
                type_name: "Semver",
                input: String::from(value),
                reason: "empty string",
            });
        }

        // Split into core and optional pre+build
        let mut parts = value.split(&['-', '+'][..]);

        // Parse MAJOR.MINOR.PATCH
        let core = parts.next().unwrap();
        let version_parts: alloc::vec::Vec<&str> = core.split('.').collect();

        if version_parts.len() != 3 {
            return Err(ValidationError {
                type_name: "Semver",
                input: String::from(value),
                reason: "must have exactly 3 numeric parts separated by dots",
            });
        }

        let major = parse_version_number(version_parts[0], value)?;
        let minor = parse_version_number(version_parts[1], value)?;
        let patch = parse_version_number(version_parts[2], value)?;

        // Parse pre-release and build metadata
        let mut pre_release = None;
        let mut build_metadata = None;

        for part in parts {
            if value.contains(&alloc::format!("-{}", part)) && pre_release.is_none() {
                if !is_valid_pre_release(part) {
                    return Err(ValidationError {
                        type_name: "Semver",
                        input: String::from(value),
                        reason: "invalid pre-release identifier",
                    });
                }
                pre_release = Some(String::from(part));
            } else if value.contains(&alloc::format!("+{}", part)) && build_metadata.is_none() {
                if !is_valid_build_metadata(part) {
                    return Err(ValidationError {
                        type_name: "Semver",
                        input: String::from(value),
                        reason: "invalid build metadata",
                    });
                }
                build_metadata = Some(String::from(part));
            }
        }

        Ok(Semver {
            major,
            minor,
            patch,
            pre_release,
            build_metadata,
        })
    }
}

/// Parse a version number component, ensuring no leading zeros except for 0 itself.
fn parse_version_number(s: &str, full_input: &str) -> Result<u64, ValidationError> {
    if s.is_empty() {
        return Err(ValidationError {
            type_name: "Semver",
            input: String::from(full_input),
            reason: "empty version component",
        });
    }

    if s.starts_with('0') && s.len() > 1 {
        return Err(ValidationError {
            type_name: "Semver",
            input: String::from(full_input),
            reason: "version components cannot have leading zeros",
        });
    }

    s.parse::<u64>().map_err(|_| ValidationError {
        type_name: "Semver",
        input: String::from(full_input),
        reason: "invalid numeric version component",
    })
}

/// Check if a string is a valid pre-release identifier.
fn is_valid_pre_release(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    for part in s.split('.') {
        if part.is_empty() {
            return false;
        }
        if part.chars().next().unwrap().is_ascii_digit() {
            // Numeric identifiers cannot have leading zeros
            if part.starts_with('0') && part.len() > 1 {
                return false;
            }
        }
        if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}

/// Check if a string is a valid build metadata.
fn is_valid_build_metadata(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    for part in s.split('.') {
        if part.is_empty() {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}

/// Compare two pre-release identifiers per SemVer 2.0.0 rules.
fn compare_pre_release(a: &str, b: &str) -> core::cmp::Ordering {
    let a_parts: alloc::vec::Vec<&str> = a.split('.').collect();
    let b_parts: alloc::vec::Vec<&str> = b.split('.').collect();

    let min_len = a_parts.len().min(b_parts.len());

    for i in 0..min_len {
        let a_part = a_parts[i];
        let b_part = b_parts[i];

        // Compare numerically if both are numeric
        if let (Ok(a_num), Ok(b_num)) = (a_part.parse::<u64>(), b_part.parse::<u64>()) {
            let num_cmp = a_num.cmp(&b_num);
            if num_cmp != core::cmp::Ordering::Equal {
                return num_cmp;
            }
        } else {
            // Lexical comparison
            let lex_cmp = a_part.cmp(b_part);
            if lex_cmp != core::cmp::Ordering::Equal {
                return lex_cmp;
            }
        }
    }

    // Shorter one comes first
    a_parts.len().cmp(&b_parts.len())
}

/// VIN (ISO 3779 / FMVSS 115): exactly 17 chars, [A-HJ-NPR-Z0-9]{17}
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vin([u8; 17]);  // stored as uppercase ASCII

impl Vin {
    /// Returns the World Manufacturer Identifier (chars 0–2).
    pub fn wmi(&self) -> &str {
        core::str::from_utf8(&self.0[0..3]).unwrap()
    }

    /// Returns the Vehicle Descriptor Section (chars 3–8).
    pub fn vds(&self) -> &str {
        core::str::from_utf8(&self.0[3..9]).unwrap()
    }

    /// Returns the Vehicle Identifier Section (chars 9–16 per FMVSS).
    pub fn vis(&self) -> &str {
        core::str::from_utf8(&self.0[9..17]).unwrap()
    }

    /// Returns the check digit (char at position 8, 0-based index 8).
    pub fn check_digit(&self) -> char {
        self.0[8] as char
    }

    /// Returns the model year character (char at position 9, 0-based index 9).
    pub fn model_year_char(&self) -> char {
        self.0[9] as char
    }
}

impl fmt::Display for Vin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in &self.0 {
            write!(f, "{}", digit as char)?;
        }
        Ok(())
    }
}

impl core::convert::TryFrom<&str> for Vin {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 17 {
            return Err(ValidationError {
                type_name: "Vin",
                input: String::from(value),
                reason: "must be exactly 17 characters",
            });
        }

        let mut vin_bytes = [0u8; 17];
        for (i, c) in value.chars().enumerate() {
            if !is_valid_vin_char(c) {
                return Err(ValidationError {
                    type_name: "Vin",
                    input: String::from(value),
                    reason: "contains invalid character (must be A-H, J-N, P-R, S-Z, or 0-9, excluding I, O, Q)",
                });
            }
            vin_bytes[i] = c.to_ascii_uppercase() as u8;
        }

        // Check digit validation
        let expected_check = calculate_vin_check_digit(&vin_bytes);
        if vin_bytes[8] != expected_check {
            return Err(ValidationError {
                type_name: "Vin",
                input: String::from(value),
                reason: "invalid check digit",
            });
        }

        Ok(Vin(vin_bytes))
    }
}

/// Check if a character is valid in a VIN (A-H, J-N, P-R, S-Z, 0-9).
fn is_valid_vin_char(c: char) -> bool {
    let upper = c.to_ascii_uppercase();
    upper.is_ascii_digit() ||
    (upper >= 'A' && upper <= 'H') ||
    (upper >= 'J' && upper <= 'N') ||
    (upper >= 'P' && upper <= 'R') ||
    (upper >= 'S' && upper <= 'Z')
}

/// Calculate the check digit for a VIN.
fn calculate_vin_check_digit(vin: &[u8; 17]) -> u8 {
    fn char_value(c: u8) -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'H' => 1 + (c - b'A'),
            b'J'..=b'N' => 1 + (c - b'J'),
            b'P'..=b'R' => 7 + (c - b'P'),
            b'S'..=b'Z' => 2 + (c - b'S'),
            _ => 0,
        }
    }

    // Weights: 8,7,6,5,4,3,2,10,0,9,8,7,6,5,4,3,2
    const WEIGHTS: [u8; 17] = [8,7,6,5,4,3,2,10,0,9,8,7,6,5,4,3,2];

    let mut sum = 0u32;
    for i in 0..17 {
        let value = char_value(vin[i]) as u32;
        let weight = WEIGHTS[i] as u32;
        sum += value * weight;
    }

    let remainder = sum % 11;
    match remainder {
        0..=9 => b'0' + remainder as u8,
        10 => b'X',
        _ => unreachable!(),
    }
}
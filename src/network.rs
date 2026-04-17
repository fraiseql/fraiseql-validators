use alloc::{format, string::String, vec::Vec};
use core::fmt;

use crate::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port(u16);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddressEui48([u8; 6]);

impl Port {
    pub const HTTP: Self = Self(80);
    pub const HTTPS: Self = Self(443);
    pub const SSH: Self = Self(22);

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.0
    }
}

impl From<u16> for Port {
    fn from(n: u16) -> Self {
        Self(n)
    }
}

impl core::convert::TryFrom<&str> for Port {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed: u32 = value.parse().map_err(|_| ValidationError {
            type_name: "Port",
            input: String::from(value),
            reason: String::from("invalid digit found in string"),
        })?;
        if parsed > 65535 {
            return Err(ValidationError {
                type_name: "Port",
                input: String::from(value),
                reason: String::from("value must be between 0 and 65535"),
            });
        }
        #[allow(clippy::cast_possible_truncation)]
        Ok(Self(parsed as u16))
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MacAddressEui48 {
    #[must_use]
    pub fn to_canonical(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }

    #[must_use]
    pub const fn octets(&self) -> [u8; 6] {
        self.0
    }

    #[must_use]
    pub const fn is_multicast(&self) -> bool {
        self.0[0] & 1 != 0
    }

    #[must_use]
    pub const fn is_locally_administered(&self) -> bool {
        self.0[0] & 2 != 0
    }
}

fn parse_hex_octets(s: &str, buf: &mut [u8]) -> Result<(), ValidationError> {
    let parts: Vec<&str>;

    // Detect separator
    let colon_count = s.chars().filter(|&c| c == ':').count();
    let hyphen_count = s.chars().filter(|&c| c == '-').count();

    if colon_count == buf.len() - 1 && hyphen_count == 0 {
        parts = s.split(':').collect();
    } else if hyphen_count == buf.len() - 1 && colon_count == 0 {
        parts = s.split('-').collect();
    } else if colon_count == 0 && hyphen_count == 0 && s.len() == buf.len() * 2 {
        // No separators, parse pairs
        parts = (0..buf.len()).map(|i| &s[i * 2..i * 2 + 2]).collect();
    } else {
        return Err(ValidationError {
            type_name: "MacAddress",
            input: String::from(s),
            reason: if colon_count > 0 && hyphen_count > 0 {
                String::from("inconsistent separators")
            } else {
                format!("MAC address must be exactly {} octets", buf.len())
            },
        });
    }

    if parts.len() != buf.len() {
        return Err(ValidationError {
            type_name: "MacAddress",
            input: String::from(s),
            reason: format!("MAC address must be exactly {} octets", buf.len()),
        });
    }

    for (i, part) in parts.iter().enumerate() {
        buf[i] = u8::from_str_radix(part, 16).map_err(|_| ValidationError {
            type_name: "MacAddress",
            input: String::from(s),
            reason: String::from("invalid hex digit"),
        })?;
    }

    Ok(())
}

impl core::convert::TryFrom<&str> for MacAddressEui48 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut octets = [0u8; 6];
        parse_hex_octets(value, &mut octets)?;
        Ok(Self(octets))
    }
}

impl fmt::Display for MacAddressEui48 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical())
    }
}
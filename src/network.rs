use alloc::{format, string::String, vec::Vec};
use core::fmt;

use crate::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port(u16);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddressEui48([u8; 6]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddressEui64([u8; 8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4Address([u8; 4]);

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

impl MacAddressEui64 {
    #[must_use]
    pub fn to_canonical(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7]
        )
    }

    #[must_use]
    pub const fn octets(&self) -> [u8; 8] {
        self.0
    }

    #[must_use]
    pub fn from_eui48(mac: &MacAddressEui48) -> Self {
        let mut octets = [0u8; 8];
        octets[0..3].copy_from_slice(&mac.0[0..3]);
        octets[3..5].copy_from_slice(&[0xFF, 0xFE]);
        octets[5..8].copy_from_slice(&mac.0[3..6]);
        Self(octets)
    }
}

impl core::convert::TryFrom<&str> for MacAddressEui64 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut octets = [0u8; 8];
        parse_hex_octets(value, &mut octets)?;
        Ok(Self(octets))
    }
}

impl fmt::Display for MacAddressEui64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical())
    }
}

impl Ipv4Address {
    #[must_use]
    pub const fn octets(&self) -> [u8; 4] {
        self.0
    }

    #[must_use]
    pub const fn is_loopback(&self) -> bool {
        self.0[0] == 127
    }

    #[must_use]
    pub const fn is_private(&self) -> bool {
        // 10.0.0.0/8
        self.0[0] == 10 ||
        // 172.16.0.0/12
        (self.0[0] == 172 && self.0[1] >= 16 && self.0[1] <= 31) ||
        // 192.168.0.0/16
        (self.0[0] == 192 && self.0[1] == 168)
    }

    #[must_use]
    pub const fn is_link_local(&self) -> bool {
        // 169.254.0.0/16
        self.0[0] == 169 && self.0[1] == 254
    }
}

impl core::convert::TryFrom<&str> for Ipv4Address {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() != 4 {
            return Err(ValidationError {
                type_name: "Ipv4Address",
                input: String::from(value),
                reason: String::from("IPv4 address must have exactly 4 octets"),
            });
        }

        let mut octets = [0u8; 4];
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() || (part.len() > 1 && part.starts_with('0')) {
                return Err(ValidationError {
                    type_name: "Ipv4Address",
                    input: String::from(value),
                    reason: String::from("octet cannot have leading zeros"),
                });
            }
            let octet: u8 = part.parse().map_err(|_| ValidationError {
                type_name: "Ipv4Address",
                input: String::from(value),
                reason: String::from("octet value must be between 0 and 255"),
            })?;
            octets[i] = octet;
        }

        Ok(Self(octets))
    }
}

impl fmt::Display for Ipv4Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}
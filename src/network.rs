use alloc::{format, string::{String, ToString}, vec, vec::Vec};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv6Address([u16; 8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asn(u32);

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

impl Ipv6Address {
    #[must_use]
    pub const fn segments(&self) -> [u16; 8] {
        self.0
    }

    #[must_use]
    pub fn is_loopback(&self) -> bool {
        self.0 == [0, 0, 0, 0, 0, 0, 0, 1]
    }

    #[must_use]
    pub fn is_unspecified(&self) -> bool {
        self.0 == [0, 0, 0, 0, 0, 0, 0, 0]
    }
}

impl core::convert::TryFrom<&str> for Ipv6Address {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Check for malformed :: (like :::)
        if value.contains(":::") || value.starts_with("::") && value.len() > 2 && value.chars().nth(2) == Some(':') || value.ends_with("::") && value.len() > 2 && value.chars().nth(value.len() - 3) == Some(':') {
            return Err(ValidationError {
                type_name: "Ipv6Address",
                input: String::from(value),
                reason: String::from("invalid IPv6 address format"),
            });
        }

        let mut segments = [0u16; 8];
        let mut segment_index = 0;

        // Check for :: (double colon)
        let parts: Vec<&str> = value.split("::").collect();
        if parts.len() > 2 {
            return Err(ValidationError {
                type_name: "Ipv6Address",
                input: String::from(value),
                reason: String::from("invalid IPv6 address format"),
            });
        }

        if parts.len() == 1 {
            // No ::, parse all 8 segments
            let segs: Vec<&str> = parts[0].split(':').collect();
            if segs.len() != 8 {
                return Err(ValidationError {
                    type_name: "Ipv6Address",
                    input: String::from(value),
                    reason: String::from("invalid IPv6 address format"),
                });
            }
            for (i, seg) in segs.iter().enumerate() {
                segments[i] = u16::from_str_radix(seg, 16).map_err(|_| ValidationError {
                    type_name: "Ipv6Address",
                    input: String::from(value),
                    reason: String::from("invalid hex digit"),
                })?;
            }
        } else {
            // Has ::, expand
            let left_parts: Vec<&str> = if parts[0].is_empty() { Vec::new() } else { parts[0].split(':').collect() };
            let right_parts: Vec<&str> = if parts[1].is_empty() { Vec::new() } else { parts[1].split(':').collect() };

            // Check for invalid format (segments containing ':')
            for seg in &left_parts {
                if seg.contains(':') {
                    return Err(ValidationError {
                        type_name: "Ipv6Address",
                        input: String::from(value),
                        reason: String::from("invalid IPv6 address format"),
                    });
                }
            }
            for seg in &right_parts {
                if seg.contains(':') {
                    return Err(ValidationError {
                        type_name: "Ipv6Address",
                        input: String::from(value),
                        reason: String::from("invalid IPv6 address format"),
                    });
                }
            }

            // Parse left segments
            for seg in &left_parts {
                segments[segment_index] = u16::from_str_radix(seg, 16).map_err(|_| ValidationError {
                    type_name: "Ipv6Address",
                    input: String::from(value),
                    reason: String::from("invalid hex digit"),
                })?;
                segment_index += 1;
            }

            // Skip zeros for ::
            let zeros_to_insert = 8 - left_parts.len() - right_parts.len();
            segment_index += zeros_to_insert;

            // Parse right segments
            for seg in &right_parts {
                segments[segment_index] = u16::from_str_radix(seg, 16).map_err(|_| ValidationError {
                    type_name: "Ipv6Address",
                    input: String::from(value),
                    reason: String::from("invalid hex digit"),
                })?;
                segment_index += 1;
            }
        }

        Ok(Self(segments))
    }
}

impl fmt::Display for Ipv6Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // RFC 5952 canonical form: longest run of zeros compressed with ::
        let mut best_start = -1;
        let mut best_len = 0;

        let mut current_start = -1;
        let mut current_len = 0;

        for i in 0..8 {
            if self.0[i] == 0 {
                if current_start == -1 {
                    current_start = i as i32;
                }
                current_len += 1;
                if current_len > best_len {
                    best_start = current_start;
                    best_len = current_len;
                }
            } else {
                current_start = -1;
                current_len = 0;
            }
        }

        // If best_len <= 1, don't compress
        if best_len <= 1 {
            return write!(
                f,
                "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7]
            );
        }

        // Compress the best run
        let mut result = String::new();
        let mut i = 0;
        while i < 8 {
            if i == best_start as usize {
                result.push_str("::");
                i += best_len;
            } else {
                if !result.is_empty() && !result.ends_with(':') {
                    result.push(':');
                }
                result.push_str(&format!("{:x}", self.0[i]));
                i += 1;
            }
        }

        // Handle special cases
        if result.starts_with("::") && result.len() > 2 {
            // Already good
        } else if result.ends_with(':') && result != ":" {
            // Remove trailing :
            result.pop();
        }

        write!(f, "{}", result)
    }
}

impl Asn {
    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }

    #[must_use]
    pub fn is_private(&self) -> bool {
        // 64512–65534
        (64512..=65534).contains(&self.0) ||
        // 4200000000–4294967294
        (4_200_000_000..=4_294_967_294).contains(&self.0)
    }

    #[must_use]
    pub const fn is_reserved(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn to_dotted(&self) -> String {
        let high = self.0 / 65536;
        let low = self.0 % 65536;
        format!("{}.{}", high, low)
    }
}

impl core::convert::TryFrom<&str> for Asn {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains('.') {
            // Dotted notation: high.low
            let parts: Vec<&str> = value.split('.').collect();
            if parts.len() != 2 {
                return Err(ValidationError {
                    type_name: "Asn",
                    input: String::from(value),
                    reason: String::from("invalid ASN format"),
                });
            }
            let high: u32 = parts[0].parse().map_err(|_| ValidationError {
                type_name: "Asn",
                input: String::from(value),
                reason: String::from("invalid number"),
            })?;
            let low: u32 = parts[1].parse().map_err(|_| ValidationError {
                type_name: "Asn",
                input: String::from(value),
                reason: String::from("invalid number"),
            })?;
            if high > 65535 || low > 65535 {
                return Err(ValidationError {
                    type_name: "Asn",
                    input: String::from(value),
                    reason: String::from("dotted ASN parts must be <= 65535"),
                });
            }
            let asn = high * 65536 + low;
            if asn == 0 {
                return Err(ValidationError {
                    type_name: "Asn",
                    input: String::from(value),
                    reason: String::from("ASN 0 is reserved"),
                });
            }
            Ok(Asn(asn))
        } else {
            // Plain decimal
            let asn: u32 = value.parse().map_err(|_| ValidationError {
                type_name: "Asn",
                input: String::from(value),
                reason: String::from("invalid number"),
            })?;
            if asn == 0 {
                return Err(ValidationError {
                    type_name: "Asn",
                    input: String::from(value),
                    reason: String::from("ASN 0 is reserved"),
                });
            }
            Ok(Asn(asn))
        }
    }
}

impl fmt::Display for Asn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
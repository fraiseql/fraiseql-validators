use alloc::string::String;
use core::fmt;

use crate::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port(u16);

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
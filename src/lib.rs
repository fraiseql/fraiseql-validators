#![no_std]

extern crate alloc;

use alloc::string::String;
use core::fmt;

#[derive(Debug, PartialEq)]
pub struct ValidationError {
    pub type_name: &'static str,
    pub input: String,
    pub reason: &'static str,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ValidationError {{ type: {}, input: {}, reason: {} }}",
            self.type_name, self.input, self.reason
        )
    }
}

#[cfg(feature = "contact")]
pub mod contact;

#[cfg(feature = "financial_banking")]
pub mod financial_banking;

#[cfg(feature = "financial_banking")]
mod checksum;

#[cfg(feature = "financial_securities")]
pub mod financial_securities;

#[cfg(feature = "barcodes")]
pub mod barcodes;

#[cfg(feature = "identifiers")]
pub mod identifiers;

#[cfg(feature = "geographic")]
pub mod geographic;

#[cfg(feature = "network")]
pub mod network;

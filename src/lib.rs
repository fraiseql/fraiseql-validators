//! # fraiseql-validators
//!
//! A `no_std` library for validating various financial and contact identifiers.
//!
//! This crate provides type-safe wrappers for common identifiers like email addresses,
//! phone numbers, IBANs, ISINs, and more, ensuring they conform to their respective standards.

#![no_std]

extern crate alloc;

use alloc::string::String;
use core::fmt;

/// Represents a validation error with details about what went wrong.
///
/// This error type is returned when attempting to parse invalid input
/// into a validated type.
#[derive(Debug, PartialEq, Eq)]
pub struct ValidationError {
    /// The name of the type that failed validation (e.g., "Email", "Iban").
    pub type_name: &'static str,
    /// The original input string that was invalid.
    pub input: String,
    /// A human-readable description of why validation failed.
    pub reason: String,
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

#[cfg(feature = "financial_securities")]
pub mod financial_securities;

#[cfg(any(feature = "financial_banking", feature = "financial_securities"))]
mod checksum;

#[cfg(feature = "barcodes")]
pub mod barcodes;

#[cfg(feature = "identifiers")]
pub mod identifiers;

#[cfg(feature = "network")]
pub mod network;

#[cfg(feature = "geographic")]
pub mod geographic;

#[cfg(feature = "fraiseql")]
mod fraiseql_bridge;

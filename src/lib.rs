#![no_std]

extern crate alloc;

#[cfg(feature = "contact")]
pub mod contact;

#[cfg(feature = "financial_banking")]
pub mod financial_banking;

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

//! Bridge for integration with the `FraiseQL` ecosystem.
//!
//! Enabled by the `fraiseql` feature flag, this module provides conversion
//! methods from [`ValidationError`] to `FraiseQL` error types.
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! fraiseql-validators = { version = "0.2", features = ["fraiseql", "contact"] }
//! ```
//!
//! ```ignore
//! use fraiseql_validators::contact::Email;
//!
//! let result = Email::try_from("bad-email");
//! if let Err(e) = result {
//!     // Convert to a FraiseQL field error for GraphQL responses
//!     let field_err = e.into_field_error("user.email");
//!
//!     // Or convert directly to a FraiseQLError
//!     // let err = e.into_fraiseql_error();
//! }
//! ```

use fraiseql_error::{FraiseQLError, ValidationFieldError};

use crate::ValidationError;

impl ValidationError {
    /// Convert into a [`FraiseQLError::Validation`].
    ///
    /// The `reason` field becomes the error message. No path is attached;
    /// use [`into_field_error`](Self::into_field_error) when the GraphQL
    /// field path is known.
    #[must_use]
    pub fn into_fraiseql_error(self) -> FraiseQLError {
        FraiseQLError::Validation {
            message: self.reason,
            path: None,
        }
    }

    /// Convert into a [`ValidationFieldError`] for a specific GraphQL field.
    ///
    /// Maps `type_name` to `rule_type` and `reason` to `message`.
    #[must_use]
    pub fn into_field_error(self, field: impl Into<alloc::string::String>) -> ValidationFieldError {
        ValidationFieldError::new(field, self.type_name, self.reason)
    }
}

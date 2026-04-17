use fraiseql_validators::ValidationError;

/// Assert that a type roundtrips correctly: `try_from` succeeds and `to_string` matches input
///
/// # Panics
///
/// Panics if parsing fails or if the round-trip output does not match the input.
pub fn assert_roundtrip<T>(input: &str)
where
    T: for<'a> core::convert::TryFrom<&'a str, Error = ValidationError> + core::fmt::Display,
{
    let parsed = T::try_from(input).unwrap_or_else(|_| panic!("Failed to parse '{input}'"));
    let output = parsed.to_string();
    assert_eq!(
        output, input,
        "Roundtrip failed: input '{input}' -> parsed -> output '{output}'"
    );
}

/// Assert that a type rejects invalid input with `ValidationError`
///
/// # Panics
///
/// Panics if the input is unexpectedly accepted.
pub fn assert_rejects<T>(input: &str)
where
    T: for<'a> core::convert::TryFrom<&'a str, Error = ValidationError>,
{
    let result = T::try_from(input);
    assert!(
        result.is_err(),
        "Expected '{input}' to be rejected, but it was accepted"
    );
}

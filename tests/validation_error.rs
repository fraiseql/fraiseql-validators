use fraiseql_validators::ValidationError;

#[test]
fn test_validation_error_fields() {
    let err = ValidationError {
        type_name: "TestType",
        input: "invalid input".into(),
        reason: String::from("some reason"),
    };

    assert_eq!(err.type_name, "TestType");
    assert_eq!(err.input, "invalid input");
    assert_eq!(err.reason, "some reason");
}

#[test]
fn test_validation_error_display() {
    let err = ValidationError {
        type_name: "TestType",
        input: "invalid input".into(),
        reason: String::from("some reason"),
    };

    let display = format!("{}", err);
    assert!(display.contains("TestType"));
    assert!(display.contains("invalid input"));
    assert!(display.contains("some reason"));
}

use fraiseql_validators::network::Port;

#[test]
fn test_port_try_from_valid() {
    let port = Port::try_from("80").unwrap();
    assert_eq!(port.value(), 80);
    assert_eq!(format!("{}", port), "80");
}

#[test]
fn test_port_try_from_zero() {
    let port = Port::try_from("0").unwrap();
    assert_eq!(port.value(), 0);
}

#[test]
fn test_port_try_from_max() {
    let port = Port::try_from("65535").unwrap();
    assert_eq!(port.value(), 65535);
}

#[test]
fn test_port_try_from_out_of_range() {
    let result = Port::try_from("65536");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.type_name, "Port");
    assert_eq!(err.input, "65536");
    assert_eq!(err.reason, "value must be between 0 and 65535");
}

#[test]
fn test_port_try_from_non_numeric() {
    let result = Port::try_from("abc");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid digit found in string");
}

#[test]
fn test_port_try_from_empty() {
    let result = Port::try_from("");
    assert!(result.is_err());
}

#[test]
fn test_port_try_from_negative() {
    let result = Port::try_from("-1");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid digit found in string");
}

#[test]
fn test_port_from_u16() {
    let port = Port::from(80u16);
    assert_eq!(port.value(), 80);
}

#[test]
fn test_port_const_http() {
    assert_eq!(Port::HTTP.value(), 80);
}

#[test]
fn test_port_const_https() {
    assert_eq!(Port::HTTPS.value(), 443);
}

#[test]
fn test_port_const_ssh() {
    assert_eq!(Port::SSH.value(), 22);
}
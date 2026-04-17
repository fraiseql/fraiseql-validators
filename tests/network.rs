use fraiseql_validators::network::{Port, MacAddressEui48};

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

#[test]
fn test_mac_address_eui48_try_from_colon_separated() {
    let mac = MacAddressEui48::try_from("00:1A:2B:3C:4D:5E").unwrap();
    assert_eq!(mac.to_canonical(), "00:1a:2b:3c:4d:5e");
    assert_eq!(mac.octets(), [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
}

#[test]
fn test_mac_address_eui48_try_from_hyphen_separated() {
    let mac = MacAddressEui48::try_from("00-1A-2B-3C-4D-5E").unwrap();
    assert_eq!(mac.to_canonical(), "00:1a:2b:3c:4d:5e");
}

#[test]
fn test_mac_address_eui48_try_from_no_separators() {
    let mac = MacAddressEui48::try_from("001A2B3C4D5E").unwrap();
    assert_eq!(mac.to_canonical(), "00:1a:2b:3c:4d:5e");
}

#[test]
fn test_mac_address_eui48_try_from_too_short() {
    let result = MacAddressEui48::try_from("00:1A:2B:3C:4D");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "MAC address must be exactly 6 octets");
}

#[test]
fn test_mac_address_eui48_try_from_too_long() {
    let result = MacAddressEui48::try_from("00:1A:2B:3C:4D:5E:6F");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "MAC address must be exactly 6 octets");
}

#[test]
fn test_mac_address_eui48_try_from_invalid_hex() {
    let result = MacAddressEui48::try_from("00:1A:2B:3C:4D:GG");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "invalid hex digit");
}

#[test]
fn test_mac_address_eui48_try_from_mixed_separators() {
    let result = MacAddressEui48::try_from("00:1A-2B:3C:4D:5E");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "inconsistent separators");
}

#[test]
fn test_mac_address_eui48_is_multicast() {
    let unicast = MacAddressEui48::try_from("00:1A:2B:3C:4D:5E").unwrap();
    assert!(!unicast.is_multicast());
    let multicast = MacAddressEui48::try_from("01:1A:2B:3C:4D:5E").unwrap();
    assert!(multicast.is_multicast());
}

#[test]
fn test_mac_address_eui48_is_locally_administered() {
    let global = MacAddressEui48::try_from("00:1A:2B:3C:4D:5E").unwrap();
    assert!(!global.is_locally_administered());
    let local = MacAddressEui48::try_from("02:1A:2B:3C:4D:5E").unwrap();
    assert!(local.is_locally_administered());
}
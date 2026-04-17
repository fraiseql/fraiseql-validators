use fraiseql_validators::network::{Port, MacAddressEui48, MacAddressEui64, Ipv4Address};

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

#[test]
fn test_mac_address_eui64_try_from_colon_separated() {
    let mac = MacAddressEui64::try_from("00:1A:2B:FF:FE:3C:4D:5E").unwrap();
    assert_eq!(mac.to_canonical(), "00:1a:2b:ff:fe:3c:4d:5e");
    assert_eq!(mac.octets(), [0x00, 0x1A, 0x2B, 0xFF, 0xFE, 0x3C, 0x4D, 0x5E]);
}

#[test]
fn test_mac_address_eui64_try_from_hyphen_separated() {
    let mac = MacAddressEui64::try_from("00-1A-2B-FF-FE-3C-4D-5E").unwrap();
    assert_eq!(mac.to_canonical(), "00:1a:2b:ff:fe:3c:4d:5e");
}

#[test]
fn test_mac_address_eui64_try_from_no_separators() {
    let mac = MacAddressEui64::try_from("001A2BFFFE3C4D5E").unwrap();
    assert_eq!(mac.to_canonical(), "00:1a:2b:ff:fe:3c:4d:5e");
}

#[test]
fn test_mac_address_eui64_try_from_eui48_length() {
    let result = MacAddressEui64::try_from("00:1A:2B:3C:4D:5E");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "MAC address must be exactly 8 octets");
}

#[test]
fn test_mac_address_eui64_from_eui48() {
    let eui48 = MacAddressEui48::try_from("00:1A:2B:3C:4D:5E").unwrap();
    let eui64 = MacAddressEui64::from_eui48(&eui48);
    assert_eq!(eui64.to_canonical(), "00:1a:2b:ff:fe:3c:4d:5e");
    assert_eq!(eui64.octets(), [0x00, 0x1A, 0x2B, 0xFF, 0xFE, 0x3C, 0x4D, 0x5E]);
}

#[test]
fn test_ipv4_address_try_from_valid() {
    let ip = Ipv4Address::try_from("192.168.1.1").unwrap();
    assert_eq!(ip.octets(), [192, 168, 1, 1]);
    assert_eq!(format!("{}", ip), "192.168.1.1");
}

#[test]
fn test_ipv4_address_try_from_zero() {
    let ip = Ipv4Address::try_from("0.0.0.0").unwrap();
    assert_eq!(ip.octets(), [0, 0, 0, 0]);
}

#[test]
fn test_ipv4_address_try_from_max() {
    let ip = Ipv4Address::try_from("255.255.255.255").unwrap();
    assert_eq!(ip.octets(), [255, 255, 255, 255]);
}

#[test]
fn test_ipv4_address_try_from_octet_too_large() {
    let result = Ipv4Address::try_from("192.168.1.256");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "octet value must be between 0 and 255");
}

#[test]
fn test_ipv4_address_try_from_too_few_octets() {
    let result = Ipv4Address::try_from("192.168.1");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "IPv4 address must have exactly 4 octets");
}

#[test]
fn test_ipv4_address_try_from_too_many_octets() {
    let result = Ipv4Address::try_from("192.168.1.1.1");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "IPv4 address must have exactly 4 octets");
}

#[test]
fn test_ipv4_address_try_from_leading_zero() {
    let result = Ipv4Address::try_from("192.168.01.1");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.reason, "octet cannot have leading zeros");
}

#[test]
fn test_ipv4_address_is_loopback() {
    let loopback = Ipv4Address::try_from("127.0.0.1").unwrap();
    assert!(loopback.is_loopback());
    let not_loopback = Ipv4Address::try_from("192.168.1.1").unwrap();
    assert!(!not_loopback.is_loopback());
}

#[test]
fn test_ipv4_address_is_private() {
    let private10 = Ipv4Address::try_from("10.0.0.1").unwrap();
    assert!(private10.is_private());
    let private172 = Ipv4Address::try_from("172.16.0.1").unwrap();
    assert!(private172.is_private());
    let private192 = Ipv4Address::try_from("192.168.0.1").unwrap();
    assert!(private192.is_private());
    let public = Ipv4Address::try_from("8.8.8.8").unwrap();
    assert!(!public.is_private());
}

#[test]
fn test_ipv4_address_is_link_local() {
    let link_local = Ipv4Address::try_from("169.254.0.1").unwrap();
    assert!(link_local.is_link_local());
    let not_link_local = Ipv4Address::try_from("192.168.1.1").unwrap();
    assert!(!not_link_local.is_link_local());
}
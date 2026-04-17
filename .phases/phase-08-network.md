# Phase 8: Networking Types

## Objective

Implement `Port`, `MacAddressEui48`, `MacAddressEui64`, `Ipv4Address`, `Ipv6Address`,
and `Asn` under the `network` feature flag.

## Success Criteria

- [ ] All six types implement `TryFrom<&str>` and `Display`
- [ ] `Ipv4Address` and `Ipv6Address` store as byte arrays (no heap)
- [ ] MAC address types accept both colon- and hyphen-separated input
- [ ] Each type: valid + all rejection reasons covered in tests
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean

## Notes

- No regex needed for any of these — all are structural/numeric parsers.
- `Ipv4Address` and `Ipv6Address` are intentionally simple validated newtypes. They are NOT
  in scope for PostgreSQL core types (`inet`, `cidr`) per the PRD, but remain useful as
  validated Rust types (e.g., for config parsing, API inputs).

## TDD Cycles

### Cycle 1: `Port`

`u16` range [0, 65535]. The type system enforces this; only the `TryFrom<&str>` parse path
needs validation (rejects non-numeric, out-of-range).

- **RED**: Tests for —
  - `Port::try_from("80")` → Ok
  - `Port::try_from("0")` → Ok (valid, reserved)
  - `Port::try_from("65535")` → Ok
  - `Port::try_from("65536")` → Err (out of u16 range)
  - `Port::try_from("abc")` → Err (non-numeric)
  - `Port::try_from("")` → Err
  - `Port::try_from("-1")` → Err
  - `Port::from(80u16)` → Port (infallible from u16)
  - `port.value()` → `80u16`
  - Well-known ports: `Port::HTTP` = 80, `Port::HTTPS` = 443 (const associated values)
- **GREEN**:
  ```rust
  pub struct Port(u16);
  impl Port {
      pub const HTTP: Port = Port(80);
      pub const HTTPS: Port = Port(443);
      pub const SSH: Port = Port(22);
      pub fn value(self) -> u16 { self.0 }
  }
  impl From<u16> for Port { fn from(n: u16) -> Self { Self(n) } }
  impl TryFrom<&str> for Port { /* parse u16, reject out of range */ }
  impl Display for Port { /* just the number */ }
  ```
- **REFACTOR**: —
- **CLEANUP**: Lint, commit

### Cycle 2: `MacAddressEui48`

EUI-48 (classic MAC address): 6 octets, colon- or hyphen-separated hex.
Stored as `[u8; 6]`. Canonical display: lowercase with colons.

- **RED**: Tests for —
  - `MacAddressEui48::try_from("00:1A:2B:3C:4D:5E")` → Ok
  - `MacAddressEui48::try_from("00-1A-2B-3C-4D-5E")` → Ok (hyphens)
  - `MacAddressEui48::try_from("001A2B3C4D5E")` → Ok (no separators)
  - `MacAddressEui48::try_from("00:1A:2B:3C:4D")` → Err (too short)
  - `MacAddressEui48::try_from("00:1A:2B:3C:4D:5E:6F")` → Err (too long)
  - `MacAddressEui48::try_from("00:1A:2B:3C:4D:GG")` → Err (invalid hex)
  - `MacAddressEui48::try_from("00:1A-2B:3C:4D:5E")` → Err (mixed separators)
  - `mac.to_canonical()` → `"00:1a:2b:3c:4d:5e"` (lowercase, colon-sep)
  - `mac.octets()` → `[u8; 6]`
  - `mac.is_multicast()` → bool (LSB of first octet)
  - `mac.is_locally_administered()` → bool (second LSB of first octet)
- **GREEN**:
  ```rust
  pub struct MacAddressEui48([u8; 6]);
  impl TryFrom<&str> for MacAddressEui48 {
      // Detect separator (colon, hyphen, or none)
      // Split and parse each octet as u8 from hex
  }
  impl Display for MacAddressEui48 { /* canonical: lowercase colon-sep */ }
  ```
- **REFACTOR**: Extract `parse_hex_octets(s: &str, buf: &mut [u8]) -> Result<(), ...>` —
  reused by `MacAddressEui64`
- **CLEANUP**: Lint, commit

### Cycle 3: `MacAddressEui64`

EUI-64: 8 octets, colon- or hyphen-separated hex. Stored as `[u8; 8]`.
Also accepts modified EUI-64 (EUI-48 expanded via `FF:FE` insertion at octet 3–4).

- **RED**: Tests for —
  - `MacAddressEui64::try_from("00:1A:2B:FF:FE:3C:4D:5E")` → Ok
  - `MacAddressEui64::try_from("00-1A-2B-FF-FE-3C-4D-5E")` → Ok
  - `MacAddressEui64::try_from("001A2BFFFE3C4D5E")` → Ok (no separators, 16 hex chars)
  - `MacAddressEui64::try_from("00:1A:2B:3C:4D:5E")` → Err (EUI-48 length, not 8 octets)
  - `mac.to_canonical()` → `"00:1a:2b:ff:fe:3c:4d:5e"`
  - `mac.octets()` → `[u8; 8]`
  - `MacAddressEui64::from_eui48(eui48)` → `MacAddressEui64` (inserts `FF:FE` at offset 3)
- **GREEN**:
  ```rust
  pub struct MacAddressEui64([u8; 8]);
  impl MacAddressEui64 {
      pub fn from_eui48(mac: &MacAddressEui48) -> Self { ... }
  }
  ```
- **REFACTOR**: Reuse `parse_hex_octets` from EUI-48 cycle
- **CLEANUP**: Lint, commit

### Cycle 4: `Ipv4Address`

IPv4 dotted-quad notation: four octets 0–255, no leading zeros in octets.

- **RED**: Tests for —
  - `Ipv4Address::try_from("192.168.1.1")` → Ok
  - `Ipv4Address::try_from("0.0.0.0")` → Ok
  - `Ipv4Address::try_from("255.255.255.255")` → Ok
  - `Ipv4Address::try_from("192.168.1.256")` → Err (octet > 255)
  - `Ipv4Address::try_from("192.168.1")` → Err (too few octets)
  - `Ipv4Address::try_from("192.168.1.1.1")` → Err (too many octets)
  - `Ipv4Address::try_from("192.168.01.1")` → Err (leading zero)
  - `ip.octets()` → `[u8; 4]`
  - `ip.is_loopback()` → bool (127.0.0.0/8)
  - `ip.is_private()` → bool (10/8, 172.16/12, 192.168/16)
  - `ip.is_link_local()` → bool (169.254/16)
- **GREEN**:
  ```rust
  pub struct Ipv4Address([u8; 4]);
  impl TryFrom<&str> for Ipv4Address {
      // split on '.', must have exactly 4 parts
      // each part: no leading zeros (except "0"), parse as u8
  }
  ```
- **REFACTOR**: —
- **CLEANUP**: Lint, commit

### Cycle 5: `Ipv6Address`

IPv6 in any valid text form (full, compressed `::`, mixed IPv4-mapped). Stored as `[u8; 16]`.
Use the standard parsing algorithm; handle `::` expansion.

- **RED**: Tests for —
  - `Ipv6Address::try_from("2001:db8::1")` → Ok
  - `Ipv6Address::try_from("::1")` → Ok (loopback)
  - `Ipv6Address::try_from("::")` → Ok (unspecified)
  - `Ipv6Address::try_from("2001:0db8:0000:0000:0000:0000:0000:0001")` → Ok (full form)
  - `Ipv6Address::try_from("::ffff:192.0.2.1")` → Ok (IPv4-mapped)
  - `Ipv6Address::try_from("2001:db8:::1")` → Err (double `::` with extra colon)
  - `Ipv6Address::try_from("2001:db8::1::2")` → Err (two `::` groups)
  - `ip.segments()` → `[u16; 8]`
  - `ip.is_loopback()` → bool (`::1`)
  - `ip.is_unspecified()` → bool (`::`)
  - `ip.to_string()` — canonical compressed form
- **GREEN**:
  Hand-written parser for `::` expansion (no regex — this is a deterministic grammar).
  ```rust
  pub struct Ipv6Address([u8; 16]);
  ```
  Display: implement RFC 5952 canonical compressed form (longest run of zeros, lowercase hex).
- **REFACTOR**: RFC 5952 canonical form is the tricky part — add a dedicated test comparing
  full form input to canonical compressed output
- **CLEANUP**: Lint, commit

### Cycle 6: `Asn`

Autonomous System Number: 32-bit unsigned integer [1, 4_294_967_295] (ASN 0 is reserved).
Two common text formats: plain decimal (`65001`) or dotted notation (`1.64511` = AS 131071).

- **RED**: Tests for —
  - `Asn::try_from("65001")` → Ok (plain decimal)
  - `Asn::try_from("1.64511")` → Ok (dotted = 1 × 65536 + 64511 = 131071)
  - `Asn::try_from("0")` → Err (reserved)
  - `Asn::try_from("4294967296")` → Err (> u32::MAX)
  - `Asn::try_from("0.0")` → Err (dotted form with zero = 0, reserved)
  - `asn.value()` → `u32`
  - `asn.is_private()` → bool (64512–65534 and 4200000000–4294967294)
  - `asn.is_reserved()` → bool
  - `asn.to_dotted()` → `String` (`"1.64511"` for 131071)
  - Display: plain decimal (canonical form per RFC 5396)
- **GREEN**:
  ```rust
  pub struct Asn(u32);
  impl TryFrom<&str> for Asn {
      // Try parse as plain u32; if contains '.', parse dotted (high × 65536 + low)
      // Reject 0
  }
  impl Display for Asn { /* plain decimal */ }
  ```
- **REFACTOR**: Private ranges documented with RFC references
- **CLEANUP**: Lint, commit

## Dependencies

- Requires: Phase 1 complete

## Status

[ ] Not Started

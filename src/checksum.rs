//! Checksum algorithms for validation.
//!
//! This module provides Luhn, MOD-97, GS1, and ISIN numeric expansion
//! utilities used internally by other validator modules.

use alloc::string::String;

/// Luhn algorithm validator for check digits
pub fn luhn_valid(s: &str) -> bool {
    let mut sum = 0;
    let mut alternate = false;

    for c in s.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            let mut d = digit;
            if alternate {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            sum += d;
            alternate = !alternate;
        } else {
            return false; // Non-digit character
        }
    }

    sum % 10 == 0
}

/// MOD-97 algorithm validator for IBAN
#[allow(dead_code)]
pub fn mod97_valid(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut remainder = 0u32;

    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            remainder = (remainder * 10 + digit) % 97;
        } else {
            return false; // Non-digit character
        }
    }

    remainder == 1
}

/// GS1 check digit validator for EAN/UPC/GTIN barcodes
pub fn gs1_check_digit_valid(digits: &str) -> bool {
    if digits.len() < 2 {
        return false;
    }

    let mut sum = 0;
    let mut multiplier = 3; // Start with ×3 for rightmost non-check digit

    // Process from right to left, excluding the check digit
    for c in digits.chars().rev().skip(1) {
        if let Some(digit) = c.to_digit(10) {
            sum += digit * multiplier;
            multiplier = if multiplier == 3 { 1 } else { 3 }; // Alternate 3,1,3,1,...
        } else {
            return false; // Non-digit
        }
    }

    let check_digit = (10 - (sum % 10)) % 10;

    // Last digit should be the check digit
    if let Some(last_char) = digits.chars().last() {
        if let Some(last_digit) = last_char.to_digit(10) {
            return last_digit == check_digit;
        }
    }

    false
}

/// Numeric expansion for ISIN characters (A-Z -> 10-35 as two digits)
pub fn isin_numeric_expansion(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            result.push(c);
        } else if c.is_ascii_uppercase() {
            let num = u32::from(c as u8 - b'A' + 10);
            #[allow(clippy::cast_possible_truncation)] // num is always <= 35
            let high = (num / 10) as u8;
            #[allow(clippy::cast_possible_truncation)] // num is always <= 35
            let low = (num % 10) as u8;
            result.push((b'0' + high) as char);
            result.push((b'0' + low) as char);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn_valid() {
        assert!(luhn_valid("79927398713")); // Valid Visa test number
        assert!(luhn_valid("4111111111111111")); // Valid Visa test number
        assert!(!luhn_valid("79927398714")); // Invalid (last digit changed)
        assert!(!luhn_valid("4111111111111112")); // Invalid
    }

    #[test]
    fn test_mod97_valid() {
        // IBAN test cases
        assert!(mod97_valid("3214282912345698765432161182")); // Valid IBAN checksum
        assert!(!mod97_valid("3214282912345698765432161183")); // Invalid
    }

    #[test]
    fn test_gs1_check_digit_valid() {
        // EAN-8 examples
        assert!(gs1_check_digit_valid("73513537")); // Valid EAN-8
        assert!(!gs1_check_digit_valid("73513538")); // Invalid check digit

        // EAN-13 examples
        assert!(gs1_check_digit_valid("5901234123457")); // Valid EAN-13
        assert!(!gs1_check_digit_valid("5901234123458")); // Invalid check digit

        // UPC-A examples
        assert!(gs1_check_digit_valid("012345678905")); // Valid UPC-A
        assert!(!gs1_check_digit_valid("012345678906")); // Invalid check digit

        // GTIN-14 examples
        assert!(gs1_check_digit_valid("10614141000415")); // Valid GTIN-14
        assert!(!gs1_check_digit_valid("10614141000416")); // Invalid check digit

        // Edge cases
        assert!(!gs1_check_digit_valid("1")); // Too short
        assert!(!gs1_check_digit_valid("")); // Empty
        assert!(!gs1_check_digit_valid("123A")); // Non-digit
    }
}

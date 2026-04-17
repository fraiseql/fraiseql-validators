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
}

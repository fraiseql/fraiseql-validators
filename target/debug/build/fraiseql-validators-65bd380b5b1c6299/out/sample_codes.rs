pub const SAMPLE_CODES: &[&str] = &[
    "ABC",
    "DEF",
    "GHI",
];

pub fn is_valid_sample_codes(s: &str) -> bool {
    SAMPLE_CODES.binary_search(&s).is_ok()
}

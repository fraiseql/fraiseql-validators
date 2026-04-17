pub const SAMPLE_CODE: &[&str] = &[
    "ABC",
    "DEF",
    "GHI",
];

pub fn is_valid_sample_code(s: &str) -> bool {
    SAMPLE_CODE.binary_search(&s).is_ok()
}

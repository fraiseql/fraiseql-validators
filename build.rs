// Reusable build helpers for CSV whitelist code generation
use std::env;
use std::fs;
use std::path::Path;

fn read_csv_column(path: &Path, _header: &str) -> Vec<String> {
    // Simple CSV reader for single column with header
    let content = fs::read_to_string(path).expect("Failed to read CSV");
    content
        .lines()
        .skip(1) // skip header
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn emit_sorted_str_set(name: &str, values: &[String]) -> String {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted.dedup(); // remove duplicates

    let const_name = name;
    let fn_name = format!("is_valid_{}", name.to_lowercase());

    let mut code = format!("pub const {}: &[&str] = &[\n", const_name);
    for value in &sorted {
        code.push_str(&format!("    \"{}\",\n", value));
    }
    code.push_str("];\n\n");

    code.push_str(&format!("pub fn {}(s: &str) -> bool {{\n", fn_name));
    code.push_str(&format!("    {}.binary_search(&s).is_ok()\n", const_name));
    code.push_str("}\n");

    code
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Generate currency codes
    let currencies_csv = "data/iso4217_currencies.csv";
    let currencies = read_csv_column(Path::new(currencies_csv), "alpha_code");
    let currency_code = emit_sorted_str_set("CURRENCY_CODES", &currencies);
    let currency_dest = Path::new(&out_dir).join("currency_codes.rs");
    fs::write(&currency_dest, currency_code).unwrap();
    println!("cargo:rerun-if-changed={}", currencies_csv);

    // Generate MIC codes
    let mic_csv = "data/iso10383_mic.csv";
    let mics = read_csv_column(Path::new(mic_csv), "MIC");
    let mic_code = emit_sorted_str_set("MIC_CODES", &mics);
    let mic_dest = Path::new(&out_dir).join("mic_codes.rs");
    fs::write(&mic_dest, mic_code).unwrap();
    println!("cargo:rerun-if-changed={}", mic_csv);
}

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
    let currencies_csv = "data/iso4217_currencies.csv";

    let currencies = read_csv_column(Path::new(currencies_csv), "alpha_code");
    let code = emit_sorted_str_set("CURRENCY_CODES", &currencies);

    let dest_path = Path::new(&out_dir).join("currency_codes.rs");
    fs::write(&dest_path, code).unwrap();

    println!("cargo:rerun-if-changed={}", currencies_csv);
}

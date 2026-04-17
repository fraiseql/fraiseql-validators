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

fn read_csv_columns(path: &Path) -> Vec<(String, String, String)> {
    // Read CSV with Country,Alpha2,Alpha3 format
    let content = fs::read_to_string(path).expect("Failed to read CSV");
    content
        .lines()
        .skip(1) // skip header
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 3 {
                Some((
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                    parts[2].trim().to_string(),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn read_iana_language_subtags(path: &Path) -> Vec<String> {
    // Read IANA language subtags file
    let content = fs::read_to_string(path).expect("Failed to read IANA file");
    let mut languages = Vec::new();
    let mut current_subtag = String::new();
    let mut is_language_type = false;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("%%") {
            // End of record
            if is_language_type && !current_subtag.is_empty() {
                languages.push(current_subtag.clone());
            }
            current_subtag = String::new();
            is_language_type = false;
        } else if line.starts_with("Type:") {
            is_language_type = line.contains("language");
        } else if line.starts_with("Subtag:") {
            if is_language_type {
                if let Some(value) = line.split(':').nth(1) {
                    current_subtag = value.trim().to_string();
                }
            }
        }
    }

    // Handle last record if no final %%
    if is_language_type && !current_subtag.is_empty() {
        languages.push(current_subtag);
    }

    languages
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

    // Generate country codes
    let countries_csv = "data/iso3166_countries.csv";
    let countries = read_csv_columns(Path::new(countries_csv));
    let mut alpha2_codes = Vec::new();
    let mut alpha3_codes = Vec::new();

    for (_country, alpha2, alpha3) in countries {
        alpha2_codes.push(alpha2);
        alpha3_codes.push(alpha3);
    }

    let country_code = format!(
        "{}\n{}",
        emit_sorted_str_set("COUNTRY_CODES_ALPHA2", &alpha2_codes),
        emit_sorted_str_set("COUNTRY_CODES_ALPHA3", &alpha3_codes)
    );
    let country_dest = Path::new(&out_dir).join("country_codes.rs");
    fs::write(&country_dest, country_code).unwrap();
    println!("cargo:rerun-if-changed={}", countries_csv);

    // Generate language codes
    let languages_txt = "data/iana_language_subtags.txt";
    let languages = read_iana_language_subtags(Path::new(languages_txt));
    let language_code = emit_sorted_str_set("LANGUAGE_CODES", &languages);
    let language_dest = Path::new(&out_dir).join("language_codes.rs");
    fs::write(&language_dest, language_code).unwrap();
    println!("cargo:rerun-if-changed={}", languages_txt);
}

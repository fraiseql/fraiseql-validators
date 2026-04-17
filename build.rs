// Reusable build helpers for CSV whitelist code generation
use std::env;
use std::fmt::Write;
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
        } else if line.starts_with("Subtag:") && is_language_type {
            if let Some(value) = line.split(':').nth(1) {
                current_subtag = value.trim().to_string();
            }
        }
    }

    // Handle last record if no final %%
    if is_language_type && !current_subtag.is_empty() {
        languages.push(current_subtag);
    }

    languages
}

fn read_airports(path: &Path) -> (Vec<String>, Vec<String>) {
    let mut rdr = csv::Reader::from_path(path).expect("Failed to read airports CSV");
    let mut iata_codes = Vec::new();
    let mut icao_codes = Vec::new();

    for result in rdr.records() {
        let record = result.expect("Failed to parse CSV record");
        if let (Some(iata), Some(icao)) = (record.get(13), record.get(12)) {
            if !iata.is_empty() {
                iata_codes.push(iata.to_string());
            }
            if !icao.is_empty() {
                icao_codes.push(icao.to_string());
            }
        }
    }

    (iata_codes, icao_codes)
}

fn read_iana_timezones(path: &Path) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Failed to read zone.tab");
    let mut timezones = content
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            parts.get(2).map(std::string::ToString::to_string)
        })
        .collect::<Vec<_>>();
    // Add special timezones
    timezones.push("UTC".to_string());
    timezones
}

fn emit_sorted_str_set(name: &str, values: &[String]) -> String {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted.dedup(); // remove duplicates

    let const_name = name;
    let fn_name = format!("is_valid_{}", name.to_lowercase());

    let mut code = format!("pub const {const_name}: &[&str] = &[\n");
    for value in &sorted {
        let _ = writeln!(code, "    \"{value}\",");
    }
    code.push_str("];\n\n");

    let _ = writeln!(code, "#[must_use]");
    let _ = writeln!(code, "pub fn {fn_name}(s: &str) -> bool {{");
    let _ = writeln!(code, "    {const_name}.binary_search(&s).is_ok()");
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
    println!("cargo:rerun-if-changed={currencies_csv}");

    // Generate MIC codes
    let mic_csv = "data/iso10383_mic.csv";
    let mics = read_csv_column(Path::new(mic_csv), "MIC");
    let mic_code = emit_sorted_str_set("MIC_CODES", &mics);
    let mic_dest = Path::new(&out_dir).join("mic_codes.rs");
    fs::write(&mic_dest, mic_code).unwrap();
    println!("cargo:rerun-if-changed={mic_csv}");

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
    println!("cargo:rerun-if-changed={countries_csv}");

    // Generate language codes
    let languages_txt = "data/iana_language_subtags.txt";
    let languages = read_iana_language_subtags(Path::new(languages_txt));
    let language_code = emit_sorted_str_set("LANGUAGE_CODES", &languages);
    let language_dest = Path::new(&out_dir).join("language_codes.rs");
    fs::write(&language_dest, language_code).unwrap();
    println!("cargo:rerun-if-changed={languages_txt}");

    // Generate airport codes
    let airports_csv = "data/ourairports_airports.csv";
    let (iata_codes, icao_codes) = read_airports(Path::new(airports_csv));
    let airport_code = format!(
        "{}\n{}",
        emit_sorted_str_set("IATA_AIRPORT_CODES", &iata_codes),
        emit_sorted_str_set("ICAO_AIRPORT_CODES", &icao_codes)
    );
    let airport_dest = Path::new(&out_dir).join("airport_codes.rs");
    fs::write(&airport_dest, airport_code).unwrap();
    println!("cargo:rerun-if-changed={airports_csv}");

    // Generate timezone codes
    let timezones_txt = "data/iana_zone_tab.txt";
    let timezones = read_iana_timezones(Path::new(timezones_txt));
    let timezone_code = emit_sorted_str_set("IANA_TIMEZONES", &timezones);
    let timezone_dest = Path::new(&out_dir).join("timezone_codes.rs");
    fs::write(&timezone_dest, timezone_code).unwrap();
    println!("cargo:rerun-if-changed={timezones_txt}");
}

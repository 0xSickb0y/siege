use serde_json;
use chrono::Local;
use std::fs::File;
use std::io::Write;
use colored::Colorize;
use crate::{http::FuzzResult, cli::Args};


enum OutputMode {
    Csv,
    Json,
}


impl OutputMode {
    fn process(&self, results: Vec<FuzzResult>) -> Result<(), String> {
        match self {
            OutputMode::Csv => save_as_csv(&results),
            OutputMode::Json => save_as_json(&results),
        }
    }
}


pub fn worker(args: &Args, fuzz_results: Vec<FuzzResult>) -> Result<(), String> {
    match args.output.as_deref() {
        Some("json") => OutputMode::Json.process(fuzz_results),
        Some("csv")  => OutputMode::Csv.process(fuzz_results),
        Some(_)      => unreachable!(), // Safe use of unreachable!() due to clap's 'value_parser' and CLI validation
        None => Ok(()),
    }
}


pub fn print_results_to_console(result: &FuzzResult) {
    let line = format!("/{:<20} Status: {}",result.word, result.status);
    let word_and_status = match result.status {
        200       => line.green(),
        301 | 302 => line.blue(),
        401 | 403 => line.yellow(),
        500..=599 => line.red(),
        _         => return,
    };

    println!("[{}] - [Size: {:>5} bytes  |  Duration: {:>5}ms] - {}", 
    word_and_status, result.size, result.duration.as_millis(), result.url);
}


fn generate_output_filename(extension: &str) -> String {
    let timestamp = Local::now().format("%Y-%m-%d-%H_%M_%S");
    
    return format!("siege_output_{}.{}", timestamp, extension)
}


fn save_as_json(results: &Vec<FuzzResult>) -> Result<(), String> {
    let filename = generate_output_filename("json");
    let json_data = match serde_json::to_string_pretty(results) {
        Ok(jd) => jd,
        Err(e) => return Err(format!("Failed to serialize JSON: {}", e)),
    };
    
    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(e) => return Err(format!("Failed to create file {}: {}", filename, e)),
    };

    match file.write_all(json_data.as_bytes()) {
        Ok(_) => {
            println!("\nResults saved to {}", filename);
            return Ok(())
        },
        Err(e) => return Err(format!("Failed to write to file {}: {}", filename, e)),
    }

}


fn save_as_csv(results: &Vec<FuzzResult>) -> Result<(), String> {
    Ok(()) // placeholder for future CSV implementation
}




// =======================================================
// =========================TESTS=========================
// =======================================================
 
 
 
 
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
 
    fn make_result(status: u16) -> FuzzResult {
        FuzzResult {
            url:      format!("http://target.com/{}", status),
            word:     status.to_string(),
            status,
            size:     512,
            duration: Duration::from_millis(10),
        }
    }
 
    // --- generate_output_filename ---
 
    #[test]
    fn test_filename_has_correct_extension_json() {
        let name = generate_output_filename("json");
        assert!(name.ends_with(".json"));
    }
 
    #[test]
    fn test_filename_has_correct_extension_csv() {
        let name = generate_output_filename("csv");
        assert!(name.ends_with(".csv"));
    }
 
    #[test]
    fn test_filename_starts_with_siege_output() {
        let name = generate_output_filename("json");
        assert!(name.starts_with("siege_output_"));
    }
 
    #[test]
    fn test_filename_timestamp_length() {
        let name = generate_output_filename("json");
        // siege_output_YYYY-MM-DD-HH_MM_SS.json
        let stem = name
            .trim_end_matches(".json")
            .trim_start_matches("siege_output_");
        assert_eq!(stem.len(), 19); // "2026-05-19-04_38_00"
    }
 
    // --- save_as_json ---
 
    #[test]
    fn test_save_as_json_succeeds() {
        let results = vec![make_result(200), make_result(301)];
        assert!(save_as_json(&results).is_ok());
    }
 
    #[test]
    fn test_save_as_json_empty_vec() {
        let results: Vec<FuzzResult> = vec![];
        assert!(save_as_json(&results).is_ok());
    }

    
    #[test]
    fn test_save_as_json_produces_valid_json() {
        use std::fs;

        let results  = vec![make_result(200), make_result(301)];
        let filename = generate_output_filename("json");

        save_as_json(&results).unwrap();

        let content = fs::read_to_string(&filename).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 2);

        fs::remove_file(&filename).unwrap(); // cleanup
    }
 

    // --- save_as_csv placeholder ---
    // add tests here once implemented
    #[test]
    fn test_save_as_csv_returns_ok() {
        let results = vec![make_result(200)];
        assert!(save_as_csv(&results).is_ok());
    }
}

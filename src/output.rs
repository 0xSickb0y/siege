use colored::Colorize;
use crate::http::FuzzResult;


// Save silent for -s or -q later
enum OutputMode {
    Csv,
    Json,
    Console,
    Silent,
}


pub fn worker(fuzz_results: Vec<FuzzResult>) {
    for result in fuzz_results {
        print_results(&result);
    }
}


pub fn print_results(result: &FuzzResult) {
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


pub fn save_results(output_mode: &str, fuzz_results: Vec<FuzzResult>) {

}
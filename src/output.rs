use colored::Colorize;
use crate::http::FuzzResult;

pub fn worker(fuzz_results: Vec<FuzzResult>) {
    for result in fuzz_results {
        print_results(&result);
    }
}

pub fn print_results(result: &FuzzResult) {
    let line = format!(
        "/{:<20} [Status: {}, Size: {}, Duration: {}ms]",
        result.word, result.status, result.size, result.duration.as_millis()
    );

    let colorized = match result.status {
        200       => line.green(),
        301 | 302 => line.blue(),
        401 | 403 => line.yellow(),
        500..=599 => line.red(),
        _         => return,
    };

    println!("{}", colorized);
}
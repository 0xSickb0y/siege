use colored::Colorize;
use crate::http::FuzzResult;


pub fn worker(fuzz_results: Vec<FuzzResult>) {
    for result in fuzz_results {
        print_results(&result);
    }
}


fn print_results(result: &FuzzResult) {
    let status_and_url = match result.status {
        200        => format!("{} {}", result.status.to_string().green(), result.url.to_string().green()),
        301|302    => format!("{} {}", result.status.to_string().yellow(), result.url.to_string().yellow()),
        401|403    => format!("{} {}", result.status.to_string().blue(), result.url.to_string().blue()),
        500..=599  => format!("{} {}", result.status.to_string().red(), result.url.to_string().red()),
        // _          => format!("{} {}", result.status.to_string().normal(), result.url.to_string().normal()),
        _          => return,
    };

    println!(
        "{} {}bytes {}ms",
        status_and_url,
        result.size,
        result.duration.as_millis()
    );
}
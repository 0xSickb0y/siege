use std::process::exit;
use crate::http::FuzzResult;

mod cli;
mod http;
mod banner;
mod output;
mod wordlist;

#[tokio::main]
async fn main() {
    // ARGUMENT PARSING
    let args = match cli::worker() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
    
    // PRINT BANNER
    banner::print_banner(&args);

    // PROCCESS WORDLIST AND BUILD URL FOR FUZZING
    let url_vector = match wordlist::worker(&args.url, &args.wordlist) {
        Ok(wv) => wv,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };


    // HTTP REQUESTS
    let fuzz_results: Vec<FuzzResult> = match http::worker(url_vector, args.timeout).await {
        Ok(fr) => fr,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };

    // DISPLAY OUTPUT AND RESULTS
    // TODO: find a way to print colorized requests as they go
    // maintain current logic of saving results to save to output file later
    // output::worker(fuzz_results);
}



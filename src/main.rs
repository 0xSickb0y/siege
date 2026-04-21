use std::process::exit;
use crate::http::FuzzResult;

mod cli;
mod http;
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
    output::worker(fuzz_results);
}



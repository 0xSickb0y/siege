use std::process::exit;
use crate::http::FuzzResult;

mod cli;
mod http;
mod banner;
mod output;
mod wordlist;

#[tokio::main]
async fn main() {
    // PRINT BANNER
    banner::print_banner();

    // ARGUMENT PARSING
    let args = match cli::worker() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };

    // PRINT ARGS INFO
    banner::print_args(&args);
    

    // PROCCESS WORDLIST AND BUILD URL FOR FUZZING
    let url_hashmap = match wordlist::worker(&args.url, &args.wordlist) {
        Ok(wv) => wv,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };


    // HTTP REQUESTS
    let fuzz_results: Vec<FuzzResult> = match http::worker(url_hashmap, args.timeout).await {
        Ok(fr) => fr,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };

    // DISPLAY OUTPUT AND RESULTS
    output::worker(fuzz_results);
}



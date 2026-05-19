// TODO: add custom error formating for runtime execution
// currently just prints raw:
//
// Permission denied (os error 13)


use std::process::exit;
use crate::http::FuzzResult;

mod cli;
mod http;
mod info;
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


    // PRINT BANNER, VERSION AND ARGS INFO
    info::worker(&args);


    // PROCCESS URL AND WORDLIST FOR FUZZING
    let url_hashmap = match wordlist::worker(args.url.as_ref().unwrap(), args.wordlist.as_ref().unwrap()) { // Safe use of unwrap() due to clap's 'required_unless_present' and CLI validation
        Ok(uh) => uh,
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



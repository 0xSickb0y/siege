// TODO: add custom error formating for runtime execution
// currently just prints raw:
//
// Permission denied (os error 13)

use clap;
use std::process::exit;
use crate::http::FuzzResult;


mod cli;
mod http;
mod info;
mod output;
mod wordlist;


const CRATE_NAME: &str = clap::crate_name!();
const CRATE_VERSION: &str = clap::crate_version!();
const GITHUB_URL: &str = "https://github.com/0xSickb0y/siege/";

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
    info::worker(&args, CRATE_NAME, CRATE_VERSION, GITHUB_URL);


    // PROCCESS URL AND WORDLIST FOR FUZZING
    let url_hashmap = match wordlist::worker(args.url.as_ref().unwrap(), args.wordlist.as_ref().unwrap()) { // Safe use of unwrap() due to clap's 'required_unless_present' and CLI validation
        Ok(uh) => uh,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };


    // HTTP REQUESTS
    let fuzz_results: Vec<FuzzResult> = match http::worker(url_hashmap, args.timeout, &args.status_codes).await {
        Ok(fr) => fr,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };

    // DISPLAY OUTPUT AND SAVE RESULTS
    match output::worker(&args, fuzz_results) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
}



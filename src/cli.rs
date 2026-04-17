// CLAP and Args Struct
// TODO: instead of print to stderr, define and return proper Err objects to return to the main function (better display/error handling)
// Implement THREADS and filtering later

use std::{path::PathBuf, process::exit};
use clap::{Parser, Error};
use url::{Url, ParseError};


#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    url: String,            // http://target:80/FUZZ
    #[arg(short, long)]
    wordlist: PathBuf,      // /opt/wordlists/file.txt
    #[arg(long = "mc", default_values_t = [200u16,204,301,302,307,401,403,405,500])] // explicit u16 anotation on first, inferred after
    status_codes: Vec<u16>,
    #[arg(short, long, default_value_t = 10)]
    threads: usize,         // Concurrency level
    #[arg(long, default_value_t = 5)]
    timeout: u64,           // Request timeout in seconds
}


// TODO: -> Result<Ok, Err>
fn validate_url(url: &String) {
    println!("url == {:?}", url);
    
    
    let url_result: Result<Url, ParseError> = Url::parse(url.as_str());
    println!("url_result == {:?}", url_result);

    // check FUZZ in directory path: /path/to/FUZZ
    let url_struct = match &url_result {
        Ok(url_struct) => url_struct,
        Err(error) => {
            eprintln!("Error: {error:?}");
            exit(1);
        }
    };

    println!("url_struct == {:?}", url_struct);
    let url_path = url_struct.path();
    if !url_struct.path().contains("FUZZ") {
        eprintln!("Missing 'FUZZ' keyword in {url_path}");
        exit(1);
    };

    println!("url_path == {}", url_path);

}


fn validate_wordlist(wordlist_buf: &PathBuf) {    
    
    let wordlist_option = wordlist_buf.to_str(); // returns 'None' if 'PathBuf' contains invalid UTF-8 chars
    println!("wordlist_option == {:?}", wordlist_option);
    
    let wordlist_path = match wordlist_option {
        Some(p) => p,
        None =>  {
            eprintln!("Wordlist path contains invalid UTF-8 characters: {:?}", wordlist_option);
            exit(1)
        },
    };

    
    if !wordlist_buf.exists() {
        eprintln!("Path: '{}' does not exist", wordlist_path);
        exit(1)
    } else if wordlist_buf.is_dir() {
        eprintln!("Path: '{}' is a directory", wordlist_path);
        exit(1)
    }

    println!("wordlist_path == {}", wordlist_path);
}


// -> Result<Args, Error> 
pub fn parse_args() {
    // TODO: define args
    // TODO: does URL contain "FUZZ" ?
    // TODO: does wordlist file exist and is file?
    // return Args struct or Error

    let args = Args::parse();
    validate_url(&args.url);
    validate_wordlist(&args.wordlist);
}
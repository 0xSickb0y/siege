//  TODO: Implement THREADS and filtering later

use clap::Parser;
use url::{Url, ParseError};
use std::{fmt::Debug, path::PathBuf};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub url: String,            // http://target:80/FUZZ
    #[arg(short, long)]
    pub wordlist: PathBuf,      // /opt/wordlists/file.txt
    #[arg(long = "mc", default_values_t = [200u16,204,301,302,307,401,403,405,500])] // explicit u16 anotation on first, inferred after
    pub status_codes: Vec<u16>,
    #[arg(short, long, default_value_t = 10)]
    pub threads: usize,         // Concurrency level
    #[arg(long, default_value_t = 5)]
    pub timeout: u64,           // Request timeout in seconds
}

// add -> Result<Args, Error> later to the main function, for now just print to stderr and exit. 
pub fn parse_args() -> Result<Args, String> {
    let args = Args::parse();

    match validate_url(&args.url) {
        Ok(u) => u,
        Err(e) => return Err(e),
    };

    match validate_wordlist(&args.wordlist) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    return Ok(args)
}


fn validate_url(url: &String) -> Result<Url, String> {    
    let url_result: Result<Url, ParseError> = Url::parse(url.as_str());
    let url_struct = match url_result {
        Ok(u) => u,
        Err(e) => {
            return Err(format!("Invalid URL '{url}': {e}"))
        },
    };

    let url_path = url_struct.path();
    if !url_struct.path().contains("FUZZ") {
        return Err(format!("Missing 'FUZZ' placeholder in path: '{}'", url_path))
    };

    return Ok(url_struct)

}


fn validate_wordlist(wordlist_buf: &PathBuf) -> Result<&PathBuf, String> {
    let wordlist_option = wordlist_buf.to_str(); // returns 'None' if 'PathBuf' contains invalid UTF-8 chars    
    let wordlist_path = match wordlist_option {
        Some(p) => p,
        None => {
            return Err(format!("Wordlist path contains invalid UTF-8 characters: {:?}", wordlist_option))
        }
    };

    if !wordlist_buf.exists() {
        return Err(format!("Path: '{}' does not exist", wordlist_path))
    } else if wordlist_buf.is_dir() {
        return Err(format!("Path: '{}' is a directory", wordlist_path))
    }
    return Ok(wordlist_buf)
}
use std::process::exit;

mod cli;
mod wordlist;

fn main() {
    let args = match cli::parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
    
    let wordlist_vector = match wordlist::load_wordlist(&args.url, &args.wordlist) {
        Ok(wv) => wv,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
}



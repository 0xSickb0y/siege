use std::process::exit;

mod cli;
mod http;
mod wordlist;

#[tokio::main]
async fn main() {
    let args = match cli::parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
    
    let url_vector = match wordlist::load_wordlist(&args.url, &args.wordlist) {
        Ok(wv) => wv,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };

    for url in url_vector {
        http::fuzzer(url).await;
    }
}



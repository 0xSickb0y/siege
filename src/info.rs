use clap;
use std::process::exit;
use crate::{cli::Args};


const CRATE_NAME: &str = clap::crate_name!();
const CRATE_VERSION: &str = clap::crate_version!();
const GITHUB_URL: &str = "https://github.com/0xSickb0y/siege/";


pub fn worker(args: &Args) {
  if args.version {
    println!("{} v{} - {}", CRATE_NAME, CRATE_VERSION, GITHUB_URL);
    exit(0);
  }
  
  // Safe use of unwrap() due to clap's 'required_unless_present' and CLI validation
  let url      = args.url.as_ref().unwrap(); 
  let wordlist = args.wordlist.as_ref().unwrap();


  if !args.banner {
    print_banner_and_info(args, &url, &wordlist);
  }
}


fn print_banner_and_info(args: &Args, url: &String, wordlist: &std::path::PathBuf) {
  println!(r#"
  ▄████████  ▄█     ▄████████    ▄██████▄     ▄████████
  ███    ███ ███    ███    ███   ███    ███   ███    ███
  ███    █▀  ███▌   ███    █▀    ███    █▀    ███    █▀ 
  ███        ███▌  ▄███▄▄▄      ▄███         ▄███▄▄▄    
  ▀███████████ ███▌ ▀▀███▀▀▀     ▀▀███ ████▄  ▀▀███▀▀▀    
          ███ ███    ███    █▄    ███    ███   ███    █▄ 
  ▄█    ███ ███    ███    ███   ███    ███   ███    ███
  ▄████████▀  █▀     ██████████   ████████▀    ██████████

  [{CRATE_NAME} - {CRATE_VERSION} - {GITHUB_URL}]
    "#
  );


    println!(r#"
[URL]          - {}
[WORDLIST]     - {}
[STATUS CODES] - {:?}
[THREADS]      - {}
[TIMEOUT]      - {} 
  "#, 
      url,
      wordlist.to_string_lossy(),
      args.status_codes,
      args.threads,
      args.timeout,
    );
}
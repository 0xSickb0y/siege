use std::process::exit;
use crate::{cli::Args};


pub fn worker(args: &Args, crate_name: &str, crate_version: &str, github_url: &str) {
  if args.version {
    println!("{} v{} - {}", crate_name, crate_version, github_url);
    exit(0);
  }
  
  // Safe use of unwrap() due to clap's 'required_unless_present' and CLI validation
  let url      = args.url.as_ref().unwrap(); 
  let wordlist = args.wordlist.as_ref().unwrap();


  if !args.banner {
    print_banner_and_info(args, &url, &wordlist, crate_name, crate_version, github_url);
  }
}


fn print_banner_and_info(args: &Args, url: &String, wordlist: &std::path::PathBuf, crate_name: &str, crate_version: &str, github_url: &str) {
  println!(r#"
  ▄████████  ▄█     ▄████████    ▄██████▄     ▄████████
  ███    ███ ███    ███    ███   ███    ███   ███    ███
  ███    █▀  ███▌   ███    █▀    ███    █▀    ███    █▀ 
  ███        ███▌  ▄███▄▄▄      ▄███         ▄███▄▄▄    
  ▀███████████ ███▌ ▀▀███▀▀▀     ▀▀███ ████▄  ▀▀███▀▀▀    
          ███ ███    ███    █▄    ███    ███   ███    █▄ 
  ▄█    ███ ███    ███    ███   ███    ███   ███    ███
  ▄████████▀  █▀     ██████████   ████████▀    ██████████

  [{crate_name} - {crate_version} - {github_url}]
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
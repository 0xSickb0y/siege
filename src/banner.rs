use clap;
use crate::cli::Args;

// TODO: find a better banner
pub fn print_banner(args: &Args){
  
  let name = clap::crate_name!();
  let version = format!("v{}", clap::crate_version!());
  let github = "https://github.com/0xSickb0y/siege/";


  print!(r#"
▄████████  ▄█     ▄████████    ▄██████▄     ▄████████
███    ███ ███    ███    ███   ███    ███   ███    ███
███    █▀  ███▌   ███    █▀    ███    █▀    ███    █▀ 
███        ███▌  ▄███▄▄▄      ▄███         ▄███▄▄▄    
▀███████████ ███▌ ▀▀███▀▀▀     ▀▀███ ████▄  ▀▀███▀▀▀    
        ███ ███    ███    █▄    ███    ███   ███    █▄ 
▄█    ███ ███    ███    ███   ███    ███   ███    ███
▄████████▀  █▀     ██████████   ████████▀    ██████████

[{name} - {version} - {github}]
"#
  );
  
  
  println!(r#"
[URL]          - {}
[WORDLIST]     - {}
[STATUS CODES] - {:?}
[THREADS]      - {}
[TIMEOUT]      - {} 
  "#, 
  args.url,
  args.wordlist.to_string_lossy(),
  args.status_codes,
  args.threads,
  args.timeout,
  );
}

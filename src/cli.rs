use url::Url;
use clap::Parser;
use std::{fmt::Debug, path::PathBuf, fs::File};


#[derive(Parser, Debug)]
pub struct Args {
    #[arg(
        short = 'b',
        long = "banner",
        default_value_t = false,
        help = "Suppress banner on startup"
    )]
    pub banner: bool,


    // ongoing
    #[arg(
        short = 'o',
        long,
        value_parser = validate_output,
        help = "Save results to file <CSV> <JSON>"
    )]
    pub output: Option<String>,


    #[arg(
        short = 's', 
        long = "status-codes", 
        default_values_t = [200u16,204,301,302,307,401,403,405,500]
    )]
    pub status_codes: Vec<u16>,


    #[arg(
        short = 't',
        long, default_value_t = 10
    )]
    pub threads: usize,


    #[arg(
        short = 'u', 
        long, 
        value_parser = validate_url,
        required_unless_present = "version"
    )]
    pub url: Option<String>,

    // TODO: implement
    // #[arg(
    //     short = 'v',
    //     long = "verbose",
    //     default_value_t = false,
    //     help = "Enable verbosity"
    // )]
    // pub verbose: bool,


    #[arg (
        long = "version",
        default_value_t = false,
        help = "Show version information"
    )]
    pub version: bool,

    #[arg(
        short = 'w',
        long,
        value_parser = validate_wordlist,
        required_unless_present = "version"
    )]
    pub wordlist: Option<PathBuf>,


    #[arg(
        short = 'z',
        long, default_value_t = 5,
        value_parser = validate_timeout,
        help = "Request timeout in seconds"
    )]
    pub timeout: usize,
}


pub fn worker() -> Result<Args, String> {
    let mut args = Args::parse();


    if args.version {
        return Ok(args)
    };


    // Get value from input and turn into absolute path, if possible. This is done to avoid issues with relative paths later on.
    match &args.wordlist {
        Some(w) => {
            match w.canonicalize() {
                Ok(abs) => args.wordlist = Some(abs),
                Err(e)  => return Err(e.to_string()),
            };
        },
        None => unreachable!(), // This should never happen due to clap's required_unless_present
    }  

    return Ok(args)
}


fn validate_url(url: &str) -> Result<String, String> {
    let url_struct = match Url::parse(url) {
        Ok(u)  => u,
        Err(e) => return Err(format!("Invalid URL ({e})")),
    };

    if !url_struct.path().contains("FUZZ") {
        return Err("Missing FUZZ placeholder".to_string())
    }


    return Ok(url.to_string())
}


fn validate_wordlist(path: &str) -> Result<PathBuf, String> {
    let buf = PathBuf::from(path);

    match buf.try_exists() {
        Ok(true)  => {},
        Ok(false) => return Err("Path does not exist".to_string()),
        Err(e)    => return Err(format!("Path could not be accessed ({})", e)),
    }

    if buf.is_dir() {
        return Err(format!("'{}' Is a directory", path))
    }

    if let Err(e) = File::open(&buf) {
        return Err(format!("Path is not readable ({})", e))
    }


    return Ok(buf)
}


fn validate_timeout(string: &str) -> Result<usize, String> {
    match string.parse::<usize>() {
        Ok(0)  => Err("Must be at least 1".to_string()),
        Ok(n)  => Ok(n),
        Err(_) => Err("Fractional values not supported".to_string()),
    }
}


fn validate_output(string: &str) -> Result<String, String> {
    let s_lowercase = string.to_lowercase();
    let valid_options = ["csv", "json"];

    if !valid_options.contains(&s_lowercase.as_str()) {
        return Err(format!("Invalid output mode. Valid options are: {}", valid_options.join(", ")))
    }

    return Ok(s_lowercase)
}
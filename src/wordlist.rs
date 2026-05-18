use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error};

// TODO: can the HashMap be a struct ? is there any benefit ? reduce apropagation of function parameters
// e.g. url: &String, word: &String, client: &reqwest::Client

pub fn worker(base_url: &String, wordlist: &Path) -> Result<HashMap<String, String>, Error> {
    let file =  match File::open(wordlist) {
        Ok(f) => f,
        Err(e) => return Err(e), 
    };
    
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    
    for line in reader.lines() {
        match line {
            Ok(word) => {
                if !word.starts_with('#') && !word.is_empty() {
                    let url = build_url(&base_url, &word);
                    map.insert(url, word);
                }
            },
            Err(e) => return Err(e),
        };
    };

    return Ok(map)
}


fn build_url(template: &String, word: &str) -> String {
    return template.replace("FUZZ", word)
}
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};


pub fn worker(url: &String, wordlist: &Path) -> Result<Vec<String>, Error> {
    let file =  match File::open(wordlist) {
        Ok(f) => f,
        Err(e) => return Err(e), 
    };
    let reader = BufReader::new(file);

    let mut wordlist_vector: Vec<String> = vec![];
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        if !line.starts_with('#') && !line.is_empty() {
            wordlist_vector.push(line);
        }
    };

    // TODO: add this logic to runner.rs later
    let url_vector: Vec<String> = wordlist_vector
        .iter()
        .map(|word| build_url(url, word))
        .collect();
    
    return Ok(url_vector)
}


fn build_url(template: &str, word: &str) -> String {
    return template.replace("FUZZ", word)
}
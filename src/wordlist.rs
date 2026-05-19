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




// =======================================================
// =========================TESTS=========================
// =======================================================




#[cfg(test)]
mod tests {
    use super::*;
 
    // --- build_url ---

    #[test]
    fn test_build_url_replaces_fuzz() {
        let result = build_url(&String::from("http://target.com/FUZZ"), "admin");
        assert_eq!(result, "http://target.com/admin");
    }
 
    #[test]
    fn test_build_url_empty_word() {
        let result = build_url(&String::from("http://target.com/FUZZ"), "");
        assert_eq!(result, "http://target.com/");
    }
 
    #[test]
    fn test_build_url_no_placeholder() {
        // template without FUZZ, word has nothing to replace, string unchanged
        let result = build_url(&String::from("http://target.com/path"), "admin");
        assert_eq!(result, "http://target.com/path");
    }
 
    #[test]
    fn test_build_url_multiple_fuzz() {
        // only documents current behavior, all occurrences get replaced
        let result = build_url(&String::from("http://target.com/FUZZ/FUZZ"), "admin");
        assert_eq!(result, "http://target.com/admin/admin");
    }
 
    // --- worker (line filtering logic) ---
 
    use std::io::Write;
    use std::path::PathBuf;
 
    fn write_temp_wordlist(name: &str, content: &str) -> PathBuf {
        let path = PathBuf::from(format!("/tmp/siege_{}.txt", name));
        let mut file = File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        path
    }
 
    #[test]
    fn test_worker_filters_comments() {
        let path = write_temp_wordlist("filters_comments", "# comment\nadmin\n");
        let map = worker(&"http://target.com/FUZZ".to_string(), &path).unwrap();
        assert!(!map.contains_key("http://target.com/#comment"));
        assert!(map.contains_key("http://target.com/admin"));
    }
 
    #[test]
    fn test_worker_filters_empty_lines() {
        let path = write_temp_wordlist("filters_empty_lines", "admin\n\n\nlogin\n");
        let map = worker(&"http://target.com/FUZZ".to_string(), &path).unwrap();
        assert_eq!(map.len(), 2);
    }
 
    #[test]
    fn test_worker_empty_file_produces_empty_map() {
        let path = write_temp_wordlist("empty_file", "");
        let map = worker(&"http://target.com/FUZZ".to_string(), &path).unwrap();
        assert_eq!(map.len(), 0);
    }
 
    #[test]
    fn test_worker_only_comments_produces_empty_map() {
        let path = write_temp_wordlist("only_comments", "# one\n# two\n# three\n");
        let map = worker(&"http://target.com/FUZZ".to_string(), &path).unwrap();
        assert_eq!(map.len(), 0);
    }
 
    #[test]
    fn test_worker_word_is_stored_as_value() {
        let path = write_temp_wordlist("word_as_value", "admin\n");
        let map = worker(&"http://target.com/FUZZ".to_string(), &path).unwrap();
        assert_eq!(map.get("http://target.com/admin").unwrap(), "admin");
    }
}
 

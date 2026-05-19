use reqwest;
use serde::Serialize;
use crate::output;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Serialize)]
pub struct FuzzResult {
    pub url:      String,   // URL to fetch
    pub size:     u64,      // response body size in bytes
    pub word:     String,
    pub status:   u16,      // Response status code
    pub duration: Duration, // how long the request took
}


pub async fn worker(url_hashmap: HashMap<String, String>, timeout: usize, status_codes: &Vec<u16>) -> Result<Vec<FuzzResult>, reqwest::Error> {
    let client = match build_client(timeout) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    
    let mut fuzz_results = vec![];
    for (url, word) in url_hashmap.iter() {
        match fuzzer(&url, &word, &client).await {
            Ok(fr) => if status_codes.contains(&fr.status) { 
                fuzz_results.push(fr)
            },
            Err(e) => eprintln!("Request failed: {e:?}"),
        };
    };

    return Ok(fuzz_results)
}


async fn fuzzer(url: &String, word: &String, client: &reqwest::Client) -> Result<FuzzResult, reqwest::Error> {   
    let result = match fetch(url.to_owned(), word.to_owned(), client).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    
    return Ok(result)
}


async fn fetch(url: String, word: String, client: &reqwest::Client) -> Result<FuzzResult, reqwest::Error> {
    let start = Instant::now();
    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let duration = start.elapsed();
    let status = response.status().as_u16();
    let size = response.content_length().unwrap_or(0);
    

    let result = FuzzResult { url, status, word, size, duration };
    output::print_results_to_console(&result); // Save silent for -q later and supress here
    
    
    return Ok(result)
}

fn build_client(timeout: usize) -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout as u64))
        .redirect(reqwest::redirect::Policy::none()) // disallow redirects
        .build()
}




// =======================================================
// =========================TESTS=========================
// =======================================================




#[cfg(test)]
mod tests {
    use super::*;
 
    // --- FuzzResult construction ---
 
    #[test]
    fn test_fuzz_result_fields() {
        let result = FuzzResult {
            url:      "http://target.com/admin".to_string(),
            word:     "admin".to_string(),
            status:   200,
            size:     1024,
            duration: Duration::from_millis(42),
        };
 
        assert_eq!(result.url,    "http://target.com/admin");
        assert_eq!(result.word,   "admin");
        assert_eq!(result.status, 200);
        assert_eq!(result.size,   1024);
        assert_eq!(result.duration.as_millis(), 42);
    }
 
    // --- build_client ---
 
    #[test]
    fn test_build_client_succeeds() {
        let result = build_client(5);
        assert!(result.is_ok());
    }

 
    #[test]
    fn test_build_client_large_timeout() {
        let result = build_client((u64::MAX / 1_000_000_000) as usize); // avoid overflow in Duration
        assert!(result.is_ok());
    }
}

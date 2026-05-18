use reqwest;
use crate::output;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct FuzzResult {
    pub url:      String,   // URL to fetch
    pub size:     u64,      // response body size in bytes
    pub word:     String,
    pub status:   u16,      // Response status code
    pub duration: Duration, // how long the request took
}


pub async fn worker(url_hashmap: HashMap<String, String>, timeout: u64) -> Result<Vec<FuzzResult>, reqwest::Error> {
    let client = match build_client(timeout) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    
    
    let mut fuzz_results = vec![];
    for (url, word) in url_hashmap.iter() {
        match fuzzer(&url, &word, &client).await {
            Ok(fr) => fuzz_results.push(fr),
            Err(e) => eprintln!("Request failed: {e:?}"),
        };
    };

    return Ok(fuzz_results)
}

async fn fuzzer(url: &String, word: &String, client: &reqwest::Client) -> Result<FuzzResult, reqwest::Error> {   
    let response = match fetch(url.to_owned(), word.to_owned(), client).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    
    return Ok(response)
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
    

    // println!("{} {}", response.status().to_string().green(), response.url().to_string().green());
    let result = FuzzResult { url, status, word, size, duration };
    output::print_results(&result);
    
    
    return Ok(result)
}

fn build_client(timeout: u64) -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout))
        .redirect(reqwest::redirect::Policy::none()) // disallow redirects
        .build()
}
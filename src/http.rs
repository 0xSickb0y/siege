use reqwest::Client;
use std::time::{Duration, Instant};


pub struct FuzzResult {
    pub url:      String,   // URL to fetch
    pub status:   u16,      // Response status code
    pub size:     u64,      // response body size in bytes
    pub duration: Duration, // how long the request took
}


pub async fn worker(url_vector: Vec<String> , timeout: u64) -> Result<Vec<FuzzResult>, reqwest::Error> {
    let client = match build_client(timeout) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    
    let mut fuzz_results = vec![];
    for url in url_vector {
        match fuzzer(url, &client).await {
            Ok(fr) => fuzz_results.push(fr),
            Err(e) => eprintln!("Request failed: {e}"),
        };
    };

    return Ok(fuzz_results)
}

async fn fuzzer(url: String, client: &Client) -> Result<FuzzResult, reqwest::Error> {    
    let response = match fetch(&client, url).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    return Ok(response)
}


async fn fetch(client: &Client, url: String) -> Result<FuzzResult, reqwest::Error> {
    let start = Instant::now();
    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let duration = start.elapsed();

    let status = response.status().as_u16();
    let size = response.content_length().unwrap_or(0);

    Ok(FuzzResult { url, status, size, duration })
}

fn build_client(timeout: u64) -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(Duration::from_secs(timeout))
        .redirect(reqwest::redirect::Policy::none()) // disallow redirects
        .build()
}
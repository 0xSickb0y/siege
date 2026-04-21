use reqwest::Client;
use std::time::{Duration, Instant};

struct FuzzResult {
    pub url:      String,   // URL to fetch
    pub status:   u16,      // Response status code
    pub size:     u64,      // response body size in bytes
    pub duration: Duration, // how long the request took
}


pub async fn fuzzer(url: String) {
    let client = Client::new();
    match fetch(&client, url).await {
        Ok(r) => println!("{} {} {}b {:?}", r.status, r.url, r.size, r.duration), // 404 http://172.17.0.1:8080/.php 335b 1.123544ms
        Err(e) => eprintln!("Request failed: {e}"),
    }
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
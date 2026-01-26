use reqwest;
use std::time::Duration;

const TARGET_URL: &str = "https://matchapro.web.bps.go.id/";
const TIMEOUT_SECONDS: u64 = 5;

pub async fn check_connection() -> bool {

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECONDS))
        .danger_accept_invalid_certs(true)
        .user_agent("Mozilla/5.0 (Linux; Android 12; M2010J19CG Build/SKQ1.211202.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/120.0.0.0 Mobile Safari/537.36")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Sec-CH-UA", "\"Android WebView\";v=\"120\", \"Chromium\";v=\"120\", \"Not A(Brand\";v=\"24\"".parse().unwrap());
            headers.insert("Sec-CH-UA-Mobile", "?1".parse().unwrap());
            headers.insert("Sec-CH-UA-Platform", "\"Android\"".parse().unwrap());
            headers.insert("X-Requested-With", "com.matchapro.app".parse().unwrap());
            headers
        })
        .build();
    
    if let Ok(client) = client {
        match client.get(TARGET_URL).send().await {
            Ok(response) => {
                // Consider 200-399 as success
                response.status().is_success() || response.status().is_redirection()
            }
            Err(e) => {
                eprintln!("[Connection Check] Failed: {}", e);
                false
            }
        }
    } else {
        false
    }
}

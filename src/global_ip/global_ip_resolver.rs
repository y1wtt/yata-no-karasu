use std::error::Error;
use std::fmt::Display;
use reqwest::Client;

use crate::error::invalid_response_body_error::InvalidResponseBodyError;

const DNS_ENDPOINT: &str = " https://cloudflare.com/cdn-cgi/trace";
pub
struct ResolveIp {
    client: Client,
}

trait CurrentIpResolver {
    fn new() -> Self;
    fn get_current_ip(&self) -> Result<String, Box<dyn Error>>;
}
impl CurrentIpResolver for ResolveIp {
    fn new() -> Self {
        ResolveIp { client: Client::new() }
    }

    async fn get_current_ip(&self) -> Result<String, Box<dyn Error>> {
        let uri = reqwest::Url::parse(DNS_ENDPOINT)?;
        let resp = self.client.get(uri).send().await?;
        let body = resp.text().await?;

        let ip_rows: Vec<&str> = body.split("\n").filter(|&s| {
            return s.starts_with("ip=");
        }).collect();
        ip_rows.get(0).ok_or(Box::new(InvalidResponseBodyError))
    }
}
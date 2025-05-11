use crate::DNS_ENDPOINT;
use log::Level::Error;

struct ResolveIp {}

trait CurrentIpResolver<T> {
    fn new() -> T;
    fn get_current_ip() -> Result<String, Box<dyn std::error::Error>>;
}
pub async fn get_ip_from_cloudflare() -> Result<String, Box<dyn std::error::Error>> {

    let uri = reqwest::Url::parse(DNS_ENDPOINT)?;
    let client = reqwest::Client::new();
    let resp = client.get(uri).send().await?;
    let body = resp.text().await?;

    let answer = body.split("\n").filter(|&s| {
        return s.starts_with("ip=")
    });
    let ip_rows:Vec<&str> = answer.collect();
    match ip_rows.get(0) {
        Some(&ip) => {
            Ok(ip.to_string())
        },
        None => {
            panic!("fail to found ip")
        }
    }
}

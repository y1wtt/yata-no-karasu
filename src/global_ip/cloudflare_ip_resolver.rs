use std::error::Error;
use reqwest::Client;
use crate::error::invalid_response_body_error::InvalidResponseBodyError;

const GLOBAL_IP_ENDPOINT: &str = " https://cloudflare.com/cdn-cgi/trace";

pub struct CloudFlareIpResolver {
    client: Client,
}
impl CloudFlareIpResolver {
    // Response Sample
    // fl=22f493
    // h=cloudflare.com
    // ip=1.1.1.1
    // ts=1746959668.311
    // visit_scheme=https
    // uag=Mozilla/5.0 (X11; Linux x86_64; rv:125.0) Gecko/20100101 Firefox/125.0
    // colo=NRT
    // sliver=none
    // http=http/2
    // loc=JP
    // tls=TLSv1.3
    // sni=plaintext
    // warp=off
    // gateway=off
    // rbi=off
    // kex=X25519
    fn parse_response(response: String) -> Result<String, Box<dyn Error>> {
        let ip_rows: Vec<&str> = response.split("\n").filter(|&s| {
            return s.starts_with("ip=");
        }).collect();
        Ok(
            ip_rows.get(0)
                .ok_or(InvalidResponseBodyError)?
                .replace("ip=", "")
                .to_string()
        )
    }
}

impl super::global_ip_resolver::GlobalIpResolver for CloudFlareIpResolver {
    fn new() -> Self {
        CloudFlareIpResolver { client: Client::new() }
    }

    async fn get_current_ip(&self) -> Result<String, Box<dyn Error>> {
        let uri = reqwest::Url::parse(GLOBAL_IP_ENDPOINT)?;
        let resp = self.client.get(uri).send().await?;
        let body = resp.text().await?;
        Self::parse_response(body)
    }
}
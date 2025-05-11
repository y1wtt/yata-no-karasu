use serde::{Deserialize, Serialize};
use crate::global_ip::cloudflare_ip_resolver::CloudFlareIpResolver;
use crate::global_ip::global_ip_resolver::GlobalIpResolver;

mod error;
mod global_ip;

const ENV_API_KEY: &str = "CLOUDFLARE_API_KEY";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let r_api_key: Result<String, std::env::VarError> = std::env::var(ENV_API_KEY);
    let _api_key: String = match r_api_key {
        Ok(ver) => {
            ver
        }
        Err(e) => {
            // panic!("Fail to get {} :{}", ENV_API_KEY, e);
            "".to_string()
        }
    };
    let ipResolver = CloudFlareIpResolver::new();
    let _ip = match ipResolver.get_current_ip().await {
        Ok(resp) => {
            resp
        }
        Err(e) => {
            panic!("Fail to get ip :{}", e);
        }
    };
    println!("{}", _ip);

    Ok(())
}
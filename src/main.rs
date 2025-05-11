mod current_ip;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

const ENV_API_KEY: &str = "CLOUDFLARE_API_KEY";
const DNS_ENDPOINT: &str = " https://cloudflare.com/cdn-cgi/trace";



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let r_api_key: Result<String, std::env::VarError> = std::env::var(ENV_API_KEY);
    let _api_key:String = match r_api_key {
        Ok(ver) => {
            ver
        }
        Err(e) => {
            panic!("Fail to get {} :{}", ENV_API_KEY, e);
        }
    };
    let a = current_ip::get_ip_from_cloudflare().await;
    let _ip = match a {
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
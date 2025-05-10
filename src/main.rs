use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

const ENV_API_KEY: &str = "CLOUDFLARE_API_KEY";
const DNS_ENDPOINT: &str = "https://1.1.1.1/dns-query";


#[derive(Serialize, Deserialize, Debug)]
struct DNSAnswer{
    name: String,
    r#type: i8,
    TTL: i8,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DNSResp {
    Answer: Vec<DNSAnswer>
}
async fn get_ip_from_cloudflare() -> Result<DNSResp, Box<dyn std::error::Error>> {
    let query = [
        ("name","whoami.cloudflare.com"),
        ("type","TXT")
    ];

    let uri = reqwest::Url::parse_with_params(DNS_ENDPOINT, query)?;
    let client = reqwest::Client::new();
    let resp = client.get(uri).header("accept", HeaderValue::from_static("application/dns-json")).send().await?;
    let body = resp.text().await?;

    let serialized:DNSResp = serde_json::from_str(&body)?;
    Ok(serialized)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let r_api_key: Result<String, std::env::VarError> = std::env::var(ENV_API_KEY);
    let api_key:String = match r_api_key {
        Ok(ver) => {
            ver
        }
        Err(e) => {
            panic!("Fail to get {} :{}", ENV_API_KEY, e);
        }
    };
    let a = get_ip_from_cloudflare().await;
    let ip = match a {
        Ok(resp) => {
            let ip = resp.Answer.get(0).unwrap().data.replace("\"","");
            ip
        }
        Err(e) => {
            panic!("Fail to get ip :{}", e);
        }
    };

    Ok(())
}
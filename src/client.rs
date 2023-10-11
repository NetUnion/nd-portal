use std::net::IpAddr;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, ClientBuilder};

pub(crate) fn create_client(local_ip: Option<IpAddr>, user_agent: &str) -> Result<Client, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
    ClientBuilder::new()
        .local_address(local_ip)
        .default_headers(headers)
        .build()
}

pub(crate) async fn get_ip(client: &reqwest::Client, host: &str) -> anyhow::Result<String> {
    let res = client
        .get(format!(
            "http://{}/cgi-bin/rad_user_info?callback=nd_portal&_={}",
            host,
            chrono::Local::now().timestamp_millis()
        ))
        .header(reqwest::header::USER_AGENT, crate::format::DEFAULT_UA)
        .send()
        .await?;
    let body = res.text().await?;
    let body = body.trim_start_matches("nd_portal(").trim_end_matches(')');
    let json: serde_json::Value = serde_json::from_str(body)?;
    if json["error"].as_str().unwrap() == "ok" {
        println!("Info: You have already logged in.");
    }
    Ok(json["online_ip"].as_str().unwrap().to_string())
}

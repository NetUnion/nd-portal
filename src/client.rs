use log::{trace, warn};
use std::net::IpAddr;

use network_interface::NetworkInterfaceConfig;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, ClientBuilder};

pub(crate) fn create_client(
    local_ip: Option<IpAddr>,
    user_agent: &str,
) -> Result<Client, reqwest::Error> {
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
    trace!("Get IP response: {}", body);
    let body = body.trim_start_matches("nd_portal(").trim_end_matches(')');
    let json: serde_json::Value = serde_json::from_str(body)?;
    let ip = json["online_ip"].as_str().unwrap().to_string();
    if json["error"].as_str().unwrap() == "ok" {
        warn!("IP address {} is already logged in!", ip);
    }
    Ok(ip)
}

pub(crate) fn get_ip_from_interface_name(interface: &str) -> anyhow::Result<IpAddr> {
    let interfaces = network_interface::NetworkInterface::show()?;
    let interface = interfaces
        .into_iter()
        .filter(|i| i.name == interface);
    if interface.clone().count() == 0 {
        return Err(anyhow::anyhow!("No such interface."));
    }
    let interface = interface.map(|i| i.addr).flatten().filter(|a| a.ip().is_ipv4()).collect::<Vec<_>>();
    if interface.is_empty() {
        return Err(anyhow::anyhow!("No IPv4 address found."));
    } else {
        Ok(interface[0].ip())
    }
}

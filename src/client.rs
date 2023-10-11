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

use log::{debug, error, info, warn};
use std::net::IpAddr;

use anyhow::{Context, Result};
use clap::Parser;
use client::get_ip_from_interface_name;
use format::DEFAULT_UA;
use inquire::Password;

use crate::{
    client::{create_client, get_ip},
    response::{parse_login_response, ChallengeString},
};

mod base64;
mod client;
mod config;
mod format;
mod hmac;
mod response;
mod state;
#[cfg(test)]
mod tests;
mod xencode;

#[derive(Parser, Debug)]
#[command(
    author = "fx Lingyi <pure@01fx.icu>",
    about = "A command line tool for SRun authentication",
    version = env!("CARGO_PKG_VERSION"),
)]
struct Opts {
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    username: Option<String>,
    #[arg(long)]
    password: Option<String>,
    #[arg(long)]
    ip: Option<String>,
    #[arg(long, short)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();
    if let Some(config) = opts.config {
        config_login(&config).await?;
    } else {
        let host = match opts.host {
            Some(h) => h,
            None => inquire::Text::new("Host")
                .prompt()
                .context("Failed to get host.")?,
        };
        let username = match opts.username {
            Some(u) => u,
            None => inquire::Text::new("Username")
                .prompt()
                .context("Failed to get username.")?,
        };
        let password = match opts.password {
            Some(p) => p,
            None => Password::new("Password")
                .prompt()
                .context("Failed to get password.")?,
        };
        let ip: Option<IpAddr> = match opts.ip {
            Some(i) => i
                .parse::<IpAddr>()
                .and_then(|x| Ok(Some(x)))
                .context("Failed to parse input IP address."),
            None => Ok(None),
        }?;

        login(&username, &password, &host, ip).await?;
        info!("Login successfully.");
    }
    Ok(())
}

async fn config_login(config_path: &str) -> Result<()> {
    let futures: Vec<_> = config::read_from_file(config_path)?
        .iter()
        .map(|item| -> Result<_> {
            let username = item.username.clone();
            let password = item.password.clone();
            let host = item.host.clone();
            let ip = match &item.ip {
                Some(i) => i
                    .parse::<IpAddr>()
                    .context("Failed to parse input IP address."),
                None => get_ip_from_interface_name(&item.interface.clone().unwrap()), // TODO: meaningless clone
            }?;
            Ok(tokio::spawn(async move {
                login(&username, &password, &host, Some(ip)).await
            }))
        })
        .collect();

    for future in futures {
        match future {
            Err(e) => {
                error!("Error while creating task: {}", e);
            }
            Ok(f) => match f.await? {
                Err(e) => {
                    error!("Error while executing task: {}", e);
                }
                Ok(()) => {
                    info!("Login successfully.");
                }
            },
        }
    }
    Ok(())
}

async fn login(username: &str, password: &str, host: &str, ip: Option<IpAddr>) -> Result<()> {
    let client = create_client(ip, DEFAULT_UA)?;
    let real_ip = get_ip(&client, host).await?;
    debug!(
        "Real IP address of {}: {}",
        ip.map(|x| x.to_string()).unwrap_or_else(|| "<none>".into()),
        real_ip
    );

    let state = state::PreparedState::new(host, username, password, &real_ip);
    let challenge = {
        let request = state.to_get_challenge_request(&client);
        let response = request.send().await?;
        let challenge: ChallengeString = response.text().await?.parse()?;
        challenge
    };

    let state = state.with_challenge(&challenge);
    let request = state.to_login_request(&client);
    let response = request.send().await?;
    let body = response.text().await?;
    parse_login_response(&body)?;

    Ok(())
}

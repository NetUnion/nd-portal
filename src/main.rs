use std::net::IpAddr;

use anyhow::{Context, Result};
use clap::Parser;
use format::DEFAULT_UA;
use inquire::Password;

use crate::{client::{create_client, get_ip}, response::{ChallengeString, parse_login_response}};

mod base64;
mod client;
mod format;
mod hmac;
mod response;
mod state;
#[cfg(test)]
mod tests;
mod xencode;

// clap config
// --ip(-i) <ip> : ip address, can be omitted
// --username(-u) <username> : username
// --password(-p) <password> : password

#[derive(Parser, Debug)]
#[command(
    author = "fx Lingyi <pure@01fx.icu>",
    about = "A command line tool for SRUN authentication"
)]
struct Opts {
    #[arg(long)]
    host: String,
    #[arg(long)]
    username: String,
    #[arg(long)]
    password: Option<String>,
    #[arg(long)]
    ip: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let password = match opts.password {
        Some(p) => p,
        None => Password::new("Password")
            .prompt()
            .context("Failed to get password.")?,
    };
    let input_ip: Option<IpAddr> = match opts.ip {
        Some(i) => i
            .parse::<IpAddr>()
            .and_then(|x| Ok(Some(x)))
            .context("Failed to parse input IP address."),
        None => Ok(None),
    }?;
    let client = create_client(input_ip, DEFAULT_UA)?;
    let ip = get_ip(&client, &opts.host).await?;
    let client = match input_ip {
        Some(i) => if i.to_string() != ip {
            println!("Warning: Your IP address is not the same as the one we got from the server! Using the one got from the server.");
            create_client(ip.parse::<IpAddr>()?.into(), DEFAULT_UA)?
        } else {
            client
        },
        _ => client,
    };
    let state = state::PreparedState::new(&opts.host, &opts.username, &password, &ip);
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
    println!("Login successfully.");
    Ok(())
}

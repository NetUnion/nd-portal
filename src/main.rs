use anyhow::{Context, Result};
use clap::Parser;
use format::DEFAULT_UA;
use inquire::Password;

use crate::{client::create_client, response::ChallengeString};

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let ip = get_ip(&opts.host)
        .await
        .context("Failed to get online IP.")?;
    let password = match opts.password {
        Some(p) => p,
        None => Password::new("Password")
            .prompt()
            .context("Failed to get password.")?,
    };
    let client = create_client(None, DEFAULT_UA)?;
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

async fn get_ip(host: &str) -> Result<String> {
    let res = reqwest::Client::new()
        .get(format!(
            "http://{}/cgi-bin/rad_user_info?callback=nd_portal&_={}",
            host,
            chrono::Local::now().timestamp_millis()
        ))
        .header(reqwest::header::USER_AGENT, DEFAULT_UA)
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

fn parse_login_response(body: &str) -> Result<()> {
    let body = body.trim_start_matches("nd_portal(").trim_end_matches(')');
    let json: serde_json::Value = serde_json::from_str(body)?;
    if &json["error"].as_str().unwrap() == &"ok" {
        Ok(())
    } else {
        Err(anyhow::anyhow!(json["error_msg"]
            .as_str()
            .unwrap()
            .to_string()))
    }
}

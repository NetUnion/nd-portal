use anyhow::{Context, Result};
use clap::Parser;
use format::DEFAULT_UA;
use inquire::Password;

mod base64;
mod format;
mod hmac;
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
    let challenge = get_challenge(&opts.host, &opts.username, &ip)
        .await
        .context("Failed to get challenge.")?;
    let password = match opts.password {
        Some(p) => p,
        None => Password::new("Password").prompt().context("Failed to get password.")?,
    };
    let hmd5 = hmac::hmac_md5(&challenge, &password);
    let info = format::get_info(&opts.username, &password, &ip, format::DEFAULT_AC_ID, &challenge);
    let chksum = format::get_chksum(
        &challenge,
        &opts.username,
        &hmd5,
        format::DEFAULT_AC_ID,
        &ip,
        format::DEFAULT_N,
        format::DEFAULT_RTYPE,
        &info,
    );
    login(&opts.host, &opts.username, &hmd5, &ip, &info, &chksum)
        .await
        .context("Failed to login.")?;
    println!("Login successfully.");
    Ok(())
}

async fn get_challenge(host: &str, username: &str, ip: &str) -> Result<String> {
    let res = reqwest::Client::new()
        .get(format!(
            "http://{}/cgi-bin/get_challenge?callback=nd_portal&username={}&ip={}&_={}",
            host,
            username,
            ip,
            chrono::Local::now().timestamp_millis()
        ))
        .header(reqwest::header::USER_AGENT, DEFAULT_UA)
        .send()
        .await?;
    let body = res.text().await?;
    let body = body.trim_start_matches("nd_portal(").trim_end_matches(')');
    let json: serde_json::Value = serde_json::from_str(body)?;
    Ok(json["challenge"].as_str().unwrap().to_string())
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

async fn login(
    host: &str,
    username: &str,
    hmd5: &str,
    ip: &str,
    info: &str,
    chksum: &str,
) -> Result<()> {
    let client = reqwest::Client::default();
    let res = client
        .get(format!("http://{}/cgi-bin/srun_portal", host))
        .query(&[
            ("callback", "nd_portal"),
            ("action", "login"),
            ("username", username),
            ("password", &format!("{{MD5}}{}", hmd5)),
            ("ac_id", format::DEFAULT_AC_ID),
            ("ip", ip),
            ("chksum", chksum),
            ("info", info),
            ("n", format::DEFAULT_N),
            ("type", format::DEFAULT_RTYPE),
            ("os", "Windows 10"),
            ("name", "Windows"),
            ("double_stack", "0"),
            ("_", &chrono::Local::now().timestamp_millis().to_string()),
        ])
        .header(reqwest::header::USER_AGENT, DEFAULT_UA)
        .send()
        .await?;
    let body = res.text().await?;
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

use std::str::FromStr;

use anyhow::{Result, Context};

pub(crate) struct ChallengeString(pub String);

impl FromStr for ChallengeString {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = s.trim_start_matches("nd_portal(").trim_end_matches(')');
        let json: serde_json::Value = serde_json::from_str(json).with_context(|| s.to_string())?;
        let challenge = json["challenge"].as_str().context("Failed to get challenge from JSON response.")?;
        // TODO: challenge validation
        Ok(Self(challenge.into()))
    }
}

pub(crate) fn parse_login_response(body: &str) -> Result<()> {
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

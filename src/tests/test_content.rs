use std::collections::HashMap;

use serde_qs::from_str;

use super::secret;

pub(crate) struct ChallengeRequest {
    pub username: String,
    pub ip: String,
}

pub(crate) struct ChallengeResponse {
    pub challenge: String,
    pub client_ip: String,
    pub ecode: i32,
    pub error: String,
    pub error_msg: String,
    pub expire: String,
    pub online_ip: String,
    pub res: String,
    pub srun_ver: String,
    pub st: i32,
}

pub(crate) struct LoginData {
    pub username: String,
    pub password: String,
}

pub(crate) struct LoginRequest {
    pub callback: String,
    pub action: String,
    pub username: String,
    pub password: String,
    pub ac_id: i32,
    pub ip: String,
    pub chksum: String,
    pub info: String,
    pub n: i32,
    pub rtype: i32,
    pub os: String,
    pub name: String,
    pub double_stack: i32,
}

impl From<&str> for ChallengeRequest {
    fn from(s: &str) -> Self {
        let mut s = s.split('?');
        let _ = s.next();
        let s = s.next().unwrap();
        let params: HashMap<String, String> = from_str(s).unwrap();
        ChallengeRequest {
            username: params.get("username").unwrap().to_string(),
            ip: params.get("ip").unwrap().to_string(),
        }
    }
}

impl Default for ChallengeRequest {
    fn default() -> Self {
        Self::from(secret::CHALLENGE_REQ_URL)
    }
}

impl From<&str> for ChallengeResponse {
    fn from(s: &str) -> Self {
        // 获取JSON内容
        let (_, s) = s.split_once('(').unwrap();
        let s = s.trim_end_matches(')');
        // 解析JSON
        let json: serde_json::Value = serde_json::from_str(s).unwrap();
        // 转换为结构体
        ChallengeResponse {
            challenge: json["challenge"].as_str().unwrap().to_string(),
            client_ip: json["client_ip"].as_str().unwrap().to_string(),
            ecode: json["ecode"].as_i64().unwrap() as i32,
            error: json["error"].as_str().unwrap().to_string(),
            error_msg: json["error_msg"].as_str().unwrap().to_string(),
            expire: json["expire"].as_str().unwrap().to_string(),
            online_ip: json["online_ip"].as_str().unwrap().to_string(),
            res: json["res"].as_str().unwrap().to_string(),
            srun_ver: json["srun_ver"].as_str().unwrap().to_string(),
            st: json["st"].as_i64().unwrap() as i32,
        }
    }
}

impl Default for ChallengeResponse {
    fn default() -> Self {
        Self::from(secret::CHALLENGE_RES_BODY)
    }
}

impl From<(&str, &str)> for LoginData {
    fn from(s: (&str, &str)) -> Self {
        LoginData {
            username: s.0.to_string(),
            password: s.1.to_string(),
        }
    }
}

impl Default for LoginData {
    fn default() -> Self {
        Self::from((secret::LOGIN_USERNAME, secret::LOGIN_PASSWORD))
    }
}

impl From<&str> for LoginRequest {
    fn from(s: &str) -> Self {
        let mut s = s.split('?');
        let _ = s.next();
        let s = s.next().unwrap();
        let params: HashMap<String, String> = from_str(s).unwrap();
        LoginRequest {
            callback: params.get("callback").unwrap().to_string(),
            action: params.get("action").unwrap().to_string(),
            username: params.get("username").unwrap().to_string(),
            password: params.get("password").unwrap().to_string(),
            ac_id: params.get("ac_id").unwrap().parse().unwrap(),
            ip: params.get("ip").unwrap().to_string(),
            chksum: params.get("chksum").unwrap().to_string(),
            info: params.get("info").unwrap().to_string(),
            n: params.get("n").unwrap().parse().unwrap(),
            rtype: params.get("type").unwrap().parse().unwrap(),
            os: params.get("os").unwrap().to_string(),
            name: params.get("name").unwrap().to_string(),
            double_stack: params.get("double_stack").unwrap().parse().unwrap(),
        }
    }
}

impl Default for LoginRequest {
    fn default() -> Self {
        Self::from(secret::LOGIN_REQ_URL)
    }
}

#[test]
fn test_deserialize_test_content() {
    let _ = ChallengeRequest::default();
    let _ = ChallengeResponse::default();
    let _ = LoginData::default();
    let _ = LoginRequest::default();
}
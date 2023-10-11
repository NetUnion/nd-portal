use reqwest::Client;

use crate::response::ChallengeString;

pub(crate) struct PreparedState<'a> {
    host: &'a str,
    username: &'a str,
    password: &'a str,
    ip: &'a str,
}

type RequestResult = reqwest::RequestBuilder;

impl<'a> PreparedState<'a> {
    pub(crate) fn new(host: &'a str, username: &'a str, password: &'a str, ip: &'a str) -> Self {
        Self {
            host,
            username,
            password,
            ip,
        }
    }

    pub(crate) fn to_get_challenge_request(&self, client: &Client) -> RequestResult {
        client
            .get(format!("http://{}/cgi-bin/get_challenge", self.host).as_str())
            .query(&[
                ("callback", "nd_portal"),
                ("username", self.username),
                ("ip", self.ip),
                ("double_stack", "1"),
                ("n", "200"),
                ("type", "1"),
            ])
    }

    pub(crate) fn with_challenge(self, challenge: &'a ChallengeString) -> ChallengeGotState<'a> {
        ChallengeGotState::new(self, &challenge.0)
    }
}

pub(crate) struct ChallengeGotState<'a> {
    credentials: PreparedState<'a>,
    hmac_md5: String,
    info: String,
    chksum: String,
}

impl<'a> ChallengeGotState<'a> {
    pub(crate) fn new(credentials: PreparedState<'a>, challenge: &'a str) -> Self {
        let hmac_md5 = crate::hmac::hmac_md5(challenge, credentials.password);
        let info = crate::format::get_info(
            credentials.username,
            credentials.password,
            credentials.ip,
            crate::format::DEFAULT_AC_ID,
            challenge,
        );
        let chksum = crate::format::get_chksum(
            challenge,
            credentials.username,
            &hmac_md5,
            crate::format::DEFAULT_AC_ID,
            credentials.ip,
            crate::format::DEFAULT_N,
            crate::format::DEFAULT_RTYPE,
            &info,
        );
        Self {
            credentials,
            hmac_md5,
            info,
            chksum,
        }
    }

    pub(crate) fn to_login_request(&self, client: &Client) -> RequestResult {
        client
            .get(format!("http://{}/cgi-bin/srun_portal", self.credentials.host).as_str())
            .query(&[
                ("callback", "nd_portal"),
                ("action", "login"),
                ("username", self.credentials.username),
                ("password", &format!("{{MD5}}{}", self.hmac_md5)),
                ("ac_id", crate::format::DEFAULT_AC_ID),
                ("ip", self.credentials.ip),
                ("chksum", &self.chksum),
                ("info", &self.info),
                ("n", crate::format::DEFAULT_N),
                ("type", crate::format::DEFAULT_RTYPE),
                ("os", "Windows 10"),
                ("name", "Windows"),
                ("double_stack", "0"),
            ])
    }
}

pub(crate) struct ChallengeResponse {
    pub challenge: String,
}

pub(crate) struct LoginData {
    pub username: String,
    pub password: String,
}

pub(crate) struct LoginRequest {
    pub password: String,
    pub ac_id: i32,
    pub ip: String,
    pub chksum: String,
    pub info: String,
    pub n: i32,
    pub rtype: i32,
}

impl Default for LoginData {
    fn default() -> Self {
        Self { username: "pure01fx".into(), password: "123456".into() }
    }
}

impl Default for ChallengeResponse {
    fn default() -> Self {
        Self {
            challenge: "ada4799b48321fcd0048df2a1a00a21e4cd06fc44aab824a70a5bf290a51ecaf".into(),
        }
    }
}

impl Default for LoginRequest {
    fn default() -> Self {
        Self {
            password: "{MD5}bbcc24bc429e970a9a5614f9d011467c".into(),
            ac_id: 1,
            ip: "123.123.123.123".into(),
            chksum: "42f9154f295a301c62945f1b47675732c8947112".into(),
            info: "{SRBX1}+TjUueMLJDRzZt9dyW88YRndr6Ws5eizLiRcnZJ6WVb7OlvvKW0Ia8YiyUmgOJQO3TYDWrb4gYHt7afqcn+BGHxyIoxWHYdplFgPrZldfZaMi71ykrDz49HSF0EOGNO4k/A6j+XkweZ=".into(),
            n: 200,
            rtype: 1,
        }
    }
}
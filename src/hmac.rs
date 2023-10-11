use hmac::{Mac, Hmac};

type HmacMd5 = Hmac<md5::Md5>;

pub(crate) fn hmac_md5(key: &str, data: &str) -> String {
    let mut mac = HmacMd5::new_from_slice(key.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let code = result.into_bytes();
    code.into_iter().map(|i| format!("{:02x}", i)).collect::<String>()
}

#[test]
fn test_hash_password() {
    use crate::tests::test_content::{ChallengeResponse, LoginData, LoginRequest};
    
    let challenge = ChallengeResponse::default().challenge;
    let password = LoginData::default().password;
    let hash = crate::hmac::hmac_md5(&challenge, &password);
    assert_eq!(hash, LoginRequest::default().password.trim_start_matches("{MD5}"));
}

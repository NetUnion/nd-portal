use crate::xencode::get_xencode;

pub(crate) const DEFAULT_AC_ID: &str = "1";
pub(crate) const DEFAULT_N : &str = "200";
pub(crate) const DEFAULT_RTYPE: &str = "1";
pub(crate) const DEFAULT_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

pub(crate) fn get_chksum(token: &str, username: &str, hmd5: &str, ac_id: &str, ip: &str, n: &str, rtype: &str, i: &str) -> String {
    let original = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}", 
        token, username, 
        token, hmd5, 
        token, ac_id, 
        token, ip, 
        token, n, 
        token, rtype, 
        token, i);
    // sha1 hexdigest
    use sha1::Digest;
    let mut hasher = sha1::Sha1::new();
    hasher.update(original.as_bytes());
    hasher.finalize().into_iter().map(|i| format!("{:02x}", i)).collect()
}

pub(crate) fn get_info(username: &str, password: &str, ip: &str, ac_id: &str, challenge: &str) -> String {
    let info = format!(r#"{{"username":"{}","password":"{}","ip":"{}","acid":"{}","enc_ver":"srun_bx1"}}"#, username, password, ip, ac_id);
    "{SRBX1}".to_string() + &get_xencode(&info, challenge)
}

#[test]
fn test_info() {
    use crate::tests::test_content::{ChallengeResponse, LoginData, LoginRequest};
    let username = LoginData::default().username;
    let password = LoginData::default().password;
    let hmac = crate::hmac::hmac_md5(&ChallengeResponse::default().challenge, &password);
    let ip = LoginRequest::default().ip;
    let ac_id = LoginRequest::default().ac_id.to_string();
    let info = get_info(&username, &password, &ip, &ac_id, &ChallengeResponse::default().challenge);
    assert_eq!(info, LoginRequest::default().info);
}

#[test]
fn test_chksum() {
    use crate::tests::test_content::{ChallengeResponse, LoginData, LoginRequest};
    let challenge = ChallengeResponse::default().challenge;
    let username = LoginData::default().username;
    let password = LoginData::default().password;
    let hmd5 = crate::hmac::hmac_md5(&challenge, &password);
    let ac_id = LoginRequest::default().ac_id.to_string();
    let ip = LoginRequest::default().ip;
    let n = LoginRequest::default().n.to_string();
    let rtype = LoginRequest::default().rtype.to_string();
    let info = get_info(&username, &password, &ip, &ac_id, &challenge);
    let chksum = get_chksum(&challenge, &username, &hmd5, &ac_id, &ip, &n, &rtype, &info);
    assert_eq!(chksum, LoginRequest::default().chksum);
}

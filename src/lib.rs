// 
// 
mod config;

pub use config::QiNiuConfig;


#[cfg(test)]
mod tests {

    use crypto::hmac::Hmac;
    use crypto::mac::Mac;

    use crypto::sha1::Sha1;
    use std::str::from_utf8;

    const BODY_CONTENT: &'static str = r#"bodystring"#;
    const KEY: &[u8] = b"secret_key";
    const COMPUTED_HMAC: &'static str = "97049623b0e5d20bf6beb5313d80600e3d6abe56";

    #[test]
    fn test_hmac_sha1() {
        let mut mac= Hmac::new(Sha1::new(), KEY);
        mac.input(BODY_CONTENT.as_bytes());
        let result = mac.result();
        let code = result.code();
        assert_eq!(COMPUTED_HMAC.as_bytes(), code);
        assert_eq!(COMPUTED_HMAC, from_utf8(&code).unwrap_or("failed"));
    //     assert_eq!(
    // COMPUTED_HMAC,
    // code.iter().format_with("", |byte, f| f(&format_args!("{:02x}", byte))).to_string());
    }

    #[test]
    fn test_hmac_sha1_direct() {
        let hash = hmacsha1::hmac_sha1(KEY, BODY_CONTENT.as_bytes());
        assert_eq!(COMPUTED_HMAC.as_bytes(), hash);
        assert_eq!(COMPUTED_HMAC, from_utf8(&hash).unwrap_or("failed"));
    }
}
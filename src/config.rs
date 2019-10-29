

pub struct QiNiuConfig {
    pub access_key: String,
    pub secret_key: String,
}

impl Default for QiNiuConfig {
    
    fn default() -> Self{
        QiNiuConfig {
            access_key: "".to_owned(),
            secret_key: "".to_owned()
        }
    }
}
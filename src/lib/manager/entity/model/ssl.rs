// ssl.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ssl {
    pub listen: u16,
    pub ssl_certificate: String,
    pub ssl_certificate_key: String,
    pub ssl_protocols: Vec<String>,
    pub ssl_ciphers: String,
}

// protocol.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Api {
    pub path: String,
    pub proxy_pass: String,
    pub proxy_set_header_host: String,
    pub proxy_set_header_x_real_ip: String,
    pub proxy_set_header_x_forwarded_for: String,
    pub proxy_set_header_x_forwarded_proto: String,
}

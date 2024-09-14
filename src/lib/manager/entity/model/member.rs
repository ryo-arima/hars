// member.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Member {
    pub address: String,
    pub port: u16,
    pub weight: u8,
}

// health_check.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HealthCheck {
    pub interval: u32,
    pub timeout: u32,
    pub uri: String,
}

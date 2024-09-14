// loadbalancer.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoadBalancer {
    pub algorithm: String,
    pub check_interval: u32,
    pub check_timeout: u32,
}

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    manager: MnagerConfig,
}

#[derive(Debug, Deserialize)]
struct MnagerConfig {
    store: String,
}

#[derive(Debug, Deserialize)]
struct RedisConfig {
}


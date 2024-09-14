// base.rs
use serde::Deserialize;
use crate::protocol::Api;
use crate::ssl::Ssl;

#[derive(Debug, Deserialize)]
pub struct Global {
    pub user: String,
    pub worker_processes: String,
    pub error_log: String,
    pub error_log_level: String,
    pub pid: String,
}

#[derive(Debug, Deserialize)]
pub struct Events {
    pub worker_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct LogFormat {
    pub main: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessLog {
    pub path: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpOptions {
    pub sendfile: bool,
    pub keepalive_timeout: u32,
    pub gzip: bool,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    pub path: String,
    pub root: String,
    pub index: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorPages {
    pub not_found: ErrorPage,
    pub server_error: ErrorPage,
}

#[derive(Debug, Deserialize)]
pub struct ErrorPage {
    pub path: String,
    pub root: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub listen: u16,
    pub server_name: String,
    pub location: Location,
    pub error_pages: ErrorPages,
    pub ssl: Option<Ssl>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub root: Root,
    pub api: Api,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub include: String,
    pub default_type: String,
    pub log_format: LogFormat,
    pub access_log: AccessLog,
    pub options: HttpOptions,
}

#[derive(Debug)]
pub struct Base<'a> {
    pub global: Global,
    pub events: Events,
    pub log_format: LogFormat,
    pub access_log: AccessLog,
    pub error_pages: ErrorPages,
    pub http: Http,
    pub server: Server,
    pub location: Location,
    pub ssl: Ssl,
}
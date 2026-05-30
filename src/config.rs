use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen: String,
    pub backends: Vec<String>,
    pub rate_limit: Option<RateLimitConfig>,
    pub headers: Option<HeadersConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window per IP.
    pub max_requests: u64,
    /// Window duration in seconds.
    pub window_secs: u64,
}

impl RateLimitConfig {
    pub fn window(&self) -> Duration {
        Duration::from_secs(self.window_secs)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HeadersConfig {
    /// Headers to inject into every proxied request.
    pub request: Option<Vec<(String, String)>>,
    /// Headers to inject into every response.
    pub response: Option<Vec<(String, String)>>,
}

pub fn load(path: &str) -> anyhow::Result<Config> {
    let raw = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&raw)?)
}

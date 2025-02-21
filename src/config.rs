use std::env::var;
use std::net::SocketAddr;
use anyhow::{anyhow, Context, Result};

#[derive(Clone)]
pub struct Config {
    pub smdr_socket_addr: SocketAddr,
    pub webhook_url: String,
    pub webhook_key: String,
    pub webhook_max_retries: u32,
    pub webhook_retry_delay_secs: u64,
    pub sentry_dsn: Option<String>,
    pub sentry_cron_url: Option<String>,
    pub sentry_cron_interval: u64,
}

fn get_env_var(key: &'static str) -> Result<String> {
    var(key).with_context(|| format!("Missing environment variable {}", key))
}

pub fn from_env() -> Result<Config> {

    const DEFAULT_SENTRY_CRON_INTERVAL: u64 = 180;
    const DEFAULT_MAX_RETRIES: u32 = 25;
    const DEFAULT_RETRY_DELAY_SECS: u64 = 30;

    Ok(Config {
        smdr_socket_addr: get_env_var("SMDR_ADDR")?.parse::<SocketAddr>()?,
        webhook_url: get_env_var("SMDR_WEBHOOK_URL")?,
        webhook_key: get_env_var("SMDR_WEBHOOK_KEY")?,
        webhook_max_retries: get_env_var("SMDR_WEBHOOK_MAX_RETRIES")
            .map(|v| v.parse::<u32>()
                .map_err(|e| anyhow!("Invalid SMDR_WEBHOOK_MAX_RETRIES: {}", e)))
            .unwrap_or(Ok(DEFAULT_MAX_RETRIES))?,

        webhook_retry_delay_secs: get_env_var("SMDR_WEBHOOK_RETRY_DELAY")
            .map(|v| v.parse::<u64>()
                .map_err(|e| anyhow!("Invalid SMDR_WEBHOOK_RETRY_DELAY: {}", e)))
            .unwrap_or(Ok(DEFAULT_RETRY_DELAY_SECS))?,

        sentry_dsn: get_env_var("SMDR_SENTRY_DSN").ok(),
        sentry_cron_url: get_env_var("SMDR_SENTRY_CRON_URL").ok(),
        sentry_cron_interval: get_env_var("SMDR_SENTRY_CRON_INTERVAL")
            .map(|v| v.parse::<u64>()
                .map_err(|e| anyhow!("Invalid SMDR_SENTRY_CRON_INTERVAL: {}", e)))
            .unwrap_or(Ok(DEFAULT_SENTRY_CRON_INTERVAL))?,

    })
}
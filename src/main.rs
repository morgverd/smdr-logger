use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use anyhow::{anyhow, Result};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info, warn};
use reqwest::blocking::Client;
use sentry_anyhow::capture_anyhow;
use crate::config::{Config, from_env};
use crate::socket::SMDRSocket;
use crate::types::{ActivePhoneCall, SMDRRecord};
use crate::webhooks::{sentry_cron_worker, webhook_worker};

mod types;
mod socket;
mod config;
mod webhooks;

pub struct SMDRReader {
    active: HashMap<String, ActivePhoneCall>,
    sender: Sender<ActivePhoneCall>
}
impl SMDRReader {
    pub fn new(config: &Config) -> Self {
        let (sender, receiver) = channel();
        let shared_client = Client::new();

        let webhook_client = shared_client.clone();
        let webhook_config = config.clone();

        thread::spawn(move || {
            webhook_worker(
                webhook_client,
                receiver,
                webhook_config.webhook_url,
                webhook_config.webhook_key,
                webhook_config.webhook_max_retries,
                webhook_config.webhook_retry_delay_secs
            )
        });

        let sentry_client = shared_client.clone();
        let sentry_config = config.clone();

        if let Some(sentry_cron_url) = sentry_config.sentry_cron_url {
            debug!("Spawning Sentry CRON worker...");
            thread::spawn(move || {
                sentry_cron_worker(sentry_client, sentry_cron_url, sentry_config.sentry_cron_interval);
            });
        } else {
            info!("Skipping Sentry CRON worker as there is no URL set!");
        }

        Self { active: HashMap::new(), sender }
    }

    fn finish(&mut self, record: SMDRRecord) -> Result<()> {
        let mut call = self.active
            .remove(&record.call_id)
            .unwrap_or_else(|| ActivePhoneCall::new(record.call_id.clone(), record.start.clone()));

        info!("Finished phone call #{}!", record.call_id);
        call.add_record(record);
        self.sender.send(call)
            .map_err(|e| anyhow!("Failed to queue webhook: {}", e))
    }

    pub fn ingest(&mut self, record: SMDRRecord) -> Result<()> {
        if !record.continued {
            return self.finish(record);
        }

        self.active
            .entry(record.call_id.clone())
            .or_insert_with(|| ActivePhoneCall::new(record.call_id.clone(), record.start.clone()))
            .add_record(record);

        Ok(())
    }
}

fn main() -> Result<()> {

    // Load env config.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv()?;
    let config = from_env()?;

    // Initialize Sentry guard.
    let _guard = if let Some(ref sentry_dsn) = config.sentry_dsn {
        info!("Initializing Sentry...");
        Some(sentry::init((sentry_dsn.clone(), sentry::ClientOptions {
            release: sentry::release_name!(),
            before_send: Some(Arc::new(|event| {
                error!("Sending to Sentry: {}", event.message.as_deref().unwrap_or("Unknown!"));
                Some(event)
            })),
            ..Default::default()
        })))
    } else {
        warn!("Sentry DSN is unset! Not initializing.");
        None
    };

    // Create SMDR connection.
    let mut socket = SMDRSocket::new(config.smdr_socket_addr);
    let mut reader = SMDRReader::new(&config);

    info!("Reading from SMDR...");
    loop {
        match socket.read() {
            Ok(Some(record)) => if let Err(e) = reader.ingest(record) {
                capture_anyhow(&e);
            },
            Ok(None) => continue,
            Err(e) => {
                warn!("Connection error: {}", e);
                thread::sleep(Duration::from_secs(10));
            }
        };
    }
}

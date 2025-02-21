use std::cmp::{max, min};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use anyhow::{anyhow, Result};
use log::{debug, warn};
use reqwest::blocking::Client;
use sentry::{capture_message, Level};
use crate::types::ActivePhoneCall;

fn send_webhook(
    client: &Client,
    webhook_url: &str,
    webhook_key: &str,
    call: &ActivePhoneCall
) -> Result<()> {
    let response = client.post(webhook_url)
        .header("Authorization", webhook_key)
        .json(call)
        .send()
        .map_err(|e| anyhow!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Webhook returned non-success status: {} - {}",
            response.status(),
            response.text().unwrap_or_else(|_| "Could not read response body".to_string())
        ));
    }

    Ok(())
}

pub fn webhook_worker(
    client: Client,
    receiver: Receiver<ActivePhoneCall>,
    webhook_url: String,
    webhook_key: String,
    max_retries: u32,
    retry_delay_secs: u64
) {
    while let Ok(call) = receiver.recv() {

        let client = client.clone();
        let webhook_url = webhook_url.clone();
        let webhook_key = webhook_key.clone();

        // Spawn thread to manage webhook delivery attempts.
        thread::spawn(move || {
            let mut attempts = 0;
            let mut delay = retry_delay_secs;

            loop {
                attempts += 1;
                match send_webhook(&client, &webhook_url, &webhook_key, &call) {
                    Ok(_) => {
                        debug!("Successfully sent webhook for call ID: {}", call.id());
                        break;
                    }
                    Err(e) => {
                        warn!("Webhook attempt {} failed for call ID {}: {}", attempts, call.id(), e);
                        if attempts >= max_retries {
                            capture_message(&*format!("Max retries reached for call ID {}. Giving up.", call.id()), Level::Error);
                            break;
                        }

                        // Exponential backoff with 1 hour cap.
                        thread::sleep(Duration::from_secs(delay));
                        delay = min(delay * 2, 3600);
                    }
                }
            }
        });
    }
}

pub fn sentry_cron_worker(
    client: Client,
    sentry_url: String,
    sentry_interval: u64
) {
    let success_duration = Duration::from_secs(sentry_interval);
    let error_duration = Duration::from_secs(max(5, sentry_interval / 2));

    loop {

        match client.get(&sentry_url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    debug!("Sentry CRON request submitted successfully!");
                    thread::sleep(success_duration);
                } else {

                    // No point in capturing a message here, eventually Sentry will
                    // detect the missing CRON updates and trigger an alert itself.
                    warn!("Failed to send Sentry CRON request!");
                    thread::sleep(error_duration);
                }
            },
            Err(e) => {
                capture_message(format!("Sentry CRON request error: {e}").as_str(), Level::Warning);
                thread::sleep(error_duration);
            }
        }
    }
}
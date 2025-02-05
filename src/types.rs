use std::time::{Duration, Instant};
use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
pub struct ActivePhoneCall {
    start: String,
    records: Vec<SMDRRecord>
}
impl ActivePhoneCall {
    pub fn new(start: String) -> Self {
        Self { start, records: Vec::new() }
    }

    pub fn add_record(&mut self, record: SMDRRecord) {
        self.records.push(record)
    }
}

#[derive(Debug, Clone)]
pub struct SMDRRecord {
    pub start: String,
    pub duration: Duration,
    pub ring: u8,
    pub caller: String,
    pub direction: String,
    pub called: String,
    pub dialled: String,
    pub account: String,
    pub is_internal: bool,
    pub call_id: String,
    pub continued: bool,
    pub party_1_device: String,
    pub party_1_name: String
}
impl SMDRRecord {
    pub fn from_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 13 {
            return Err(anyhow!("Insufficient parts in record!"));
        }

        let duration_parts: Vec<&str> = parts[1].split(':').collect();
        let duration = if duration_parts.len() == 3 {
            match (
                duration_parts[0].parse::<u64>().ok(), // h
                duration_parts[1].parse::<u64>().ok(), // m
                duration_parts[2].parse::<u64>().ok(), // s
            ) {
                (Some(h), Some(m), Some(s)) => Duration::new(h * 3600 + m * 60 + s, 0),
                _ => return Err(anyhow!("Failed to parse duration!")),
            }
        } else {
            return Err(anyhow!("Invalid duration format!"));
        };

        Ok(SMDRRecord {
            start: parts[0].to_string(),
            duration,
            ring: parts[2].parse().context("Failed to parse ring ")?,
            caller: parts[3].to_string(),
            direction: parts[4].to_string(),
            called: parts[5].to_string(),
            dialled: parts[6].to_string(),
            account: parts[7].to_string(),
            is_internal: parts[8] == "1",
            call_id: parts[9].to_string(),
            continued: parts[10] == "1",
            party_1_device: parts[11].to_string(),
            party_1_name: parts[12].to_string()
        })
    }
}
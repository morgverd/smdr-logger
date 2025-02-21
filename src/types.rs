use std::time::Duration;
use anyhow::{anyhow, Context, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct ActivePhoneCall {
    call_id: String,
    start: String,
    caller: Option<SMDRCaller>,
    records: Vec<ReducedSMDRRecord>
}
impl ActivePhoneCall {
    pub fn new(call_id: String, start: String) -> Self {
        Self { call_id, start, caller: None, records: Vec::new() }
    }

    pub fn add_record(&mut self, record: SMDRRecord) {
        if self.caller.is_none() {

            // If the caller is unset, get the info from the first record as it doesn't change.
            let (simplified, shared) = ReducedSMDRRecord::split(record);
            self.caller = Some(shared);
            self.records.push(simplified);
        } else {
            self.records.push(ReducedSMDRRecord::reduce(record));
        }
    }

    pub fn id(&self) -> &String {
        &self.call_id
    }
}

#[derive(Serialize)]
pub struct SMDRCaller {
    pub dialled: String,
    pub caller: String,
    pub party_2_device: String,
    pub party_2_name: String
}

#[derive(Serialize)]
pub struct ReducedSMDRRecord {
    pub duration: u64,
    pub ring: u8,
    pub direction: String,
    pub called: String,
    pub account: String,
    pub is_internal: bool,
    pub continued: bool,
    pub party_1_device: String,
    pub party_1_name: String,
    pub hold_time: u64,
    pub park_time: u64
}
impl ReducedSMDRRecord {
    pub fn split(record: SMDRRecord) -> (Self, SMDRCaller) {
        let simplified = Self::from(&record);
        let caller = SMDRCaller {
            dialled: record.dialled,
            caller: record.caller,
            party_2_device: record.party_2_device,
            party_2_name: record.party_2_name,
        };

        (simplified, caller)
    }

    pub fn reduce(record: SMDRRecord) -> Self {
        Self::from(&record)
    }
}
impl From<&SMDRRecord> for ReducedSMDRRecord {
    fn from(record: &SMDRRecord) -> Self {
        Self {
            duration: record.duration,
            ring: record.ring,
            direction: record.direction.clone(),
            called: record.called.clone(),
            account: record.account.clone(),
            is_internal: record.is_internal,
            continued: record.continued,
            party_1_device: record.party_1_device.clone(),
            party_1_name: record.party_1_name.clone(),
            hold_time: record.hold_time,
            park_time: record.park_time,
        }
    }
}

pub struct SMDRRecord {
    pub start: String,
    pub duration: u64,
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
    pub party_1_name: String,
    pub party_2_device: String,
    pub party_2_name: String,
    pub hold_time: u64,
    pub park_time: u64
}
impl SMDRRecord {
    pub fn from_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 17 {
            return Err(anyhow!("Insufficient parts in record: expected 17, got {}", parts.len()));
        }

        // Parse duration in HH:MM:SS format
        let duration = parse_duration(parts[1])?;
        Ok(Self {
            start: parts[0].to_string(),
            duration: duration.as_secs(),
            ring: parts[2].parse().context("Failed to parse ring time")?,
            caller: parts[3].to_string(),
            direction: parts[4].to_string(),
            called: parts[5].to_string(),
            dialled: parts[6].to_string(),
            account: parts[7].to_string(),
            is_internal: parts[8] == "1",
            call_id: parts[9].to_string(),
            continued: parts[10] == "1",
            party_1_device: parts[11].to_string(),
            party_1_name: parts[12].to_string(),
            party_2_device: parts[13].to_string(),
            party_2_name: parts[14].to_string(),
            hold_time: parts[15].parse().context("Failed to parse hold time")?,
            park_time: parts[16].parse().context("Failed to parse park time")?
        })
    }
}

fn parse_duration(duration_str: &str) -> Result<Duration> {
    let parts: Vec<&str> = duration_str.split(':').collect();

    if parts.len() != 3 {
        return Err(anyhow!("Invalid duration format: expected HH:MM:SS"));
    }

    let hours = parts[0].parse::<u64>()
        .context("Failed to parse hours in duration")?;
    let minutes = parts[1].parse::<u64>()
        .context("Failed to parse minutes in duration")?;
    let seconds = parts[2].parse::<u64>()
        .context("Failed to parse seconds in duration")?;

    Ok(Duration::new(hours * 3600 + minutes * 60 + seconds, 0))
}
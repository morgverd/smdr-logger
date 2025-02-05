use std::collections::HashMap;
use anyhow::{Context, Result};
use crate::types::{ActivePhoneCall, SMDRRecord};

pub struct SMDRReader {
    active: HashMap<String, ActivePhoneCall>
}
impl SMDRReader {
    pub fn new() -> Self {
        Self { active: HashMap::new() }
    }

    fn finish(&mut self, record: SMDRRecord) -> Result<()> {
        let mut call = self.active
            .remove(&record.call_id)
            .unwrap_or_else(|| ActivePhoneCall::new(record.start.clone()));

        call.add_record(record);
        println!("{call:#?}");

        Ok(())
    }

    pub fn ingest(&mut self, record: SMDRRecord) -> Result<()> {
        if !record.continued {
            return self.finish(record);
        }

        self.active
            .entry(record.call_id.clone())
            .or_insert_with(|| ActivePhoneCall::new(record.start.clone()))
            .add_record(record);

        Ok(())
    }
}
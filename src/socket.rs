use std::net::{SocketAddr, TcpStream};
use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};
use crate::types::SMDRRecord;

pub struct SMDRSocket {
    address: SocketAddr,
    reader: Option<BufReader<TcpStream>>
}

impl SMDRSocket {
    pub fn new(address: SocketAddr) -> Self {
        SMDRSocket { address, reader: None }
    }

    fn connect(&mut self) -> Result<()> {
        let stream = TcpStream::connect(self.address)?;
        self.reader = Some(BufReader::new(stream));
        Ok(())
    }

    pub(crate) fn read(&mut self) -> Result<Option<SMDRRecord>> {
        if self.reader.is_none() {
            self.connect()?;
        }

        let reader = self.reader.as_mut().unwrap();
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {
                self.reader = None;
                return Ok(None);
            }
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return Ok(None);
                }
                Ok(Some(SMDRRecord::from_line(trimmed).context("Failed to parse SMDRRecord!")?))
            }
            Err(e) => {
                self.reader = None;
                Err(e.into())
            }
        }
    }
}
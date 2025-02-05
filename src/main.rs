use std::thread;
use std::time::Duration;
use crate::reader::SMDRReader;
use anyhow::Result;
use crate::socket::SMDRSocket;

mod reader;
mod types;
mod socket;

fn main() -> Result<()> {

    let mut socket = SMDRSocket::new("192.168.1.50:8088");
    let mut reader = SMDRReader::new();

    loop {
        match socket.read() {
            Ok(Some(record)) => reader.ingest(record)?,
            Ok(None) => continue,
            Err(e) => {
                eprintln!("Connection error: {}", e);
                thread::sleep(Duration::from_secs(10));
            }
        }
    }
}

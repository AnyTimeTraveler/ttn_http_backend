use crate::{Configuration, downlink};

pub fn handle_packet(from: &str, port: u8, _counter: u32, data: &[u8], cfg: &Configuration) {
    println!("Message received: {:?}", String::from_utf8_lossy(data));
    downlink(String::from(from), port, data, cfg).expect("Error sending response");
}
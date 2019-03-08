extern crate hex;

use std::collections::HashMap;

use chrono::offset::TimeZone;
use chrono::Utc;
use hex::decode;

pub struct Section {
    entries: HashMap<String, SectionValue>
}

pub enum SectionValue {
    U32(u32),
    U64(u64),
    STRING(String),
    BYTES([u8]),
    LIST(Vec<SectionValue>),
    SECTION(Section),
}


impl Section {
    fn new() -> Section {
        Section {
            entries: HashMap::new()
        }
    }

    fn add(mut self, key: String, value: SectionValue) -> Section {
        self.entries.insert(key, value);
        self
    }


    fn handshakeRequest() {
        let mut node_data = Section::new();
        node_data.add(String::from("local_time"), SectionValue::U64(Utc.timestamp().timestamp() as u64));
        node_data.add(String::from("myport"), SectionValue::U32(0));

        //TODO hex convert something
        //mainnet
        let network_id = Hex::decode("1230f171610441611731008216a1a110");
        node_data.add(String::from("network_id"), SectionValue::BYTES(network_id));
    }
}
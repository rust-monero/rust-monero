extern crate hex;
extern crate rand;

use std::collections::HashMap;

use chrono::offset::TimeZone;
use chrono::Utc;
use hex::*;

use self::rand::Rng;

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


    fn handshakeRequest() -> Section {
        let mut node_data = Section::new();
        node_data.add(String::from("local_time"), SectionValue::U64(Utc.timestamp().timestamp() as u64));
        node_data.add(String::from("myport"), SectionValue::U32(0));

        //TODO hex convert something
        //mainnet
        let network_id = Hex::decode("1230f171610441611731008216a1a110").unwrap();
        node_data.add(String::from("network_id"), SectionValue::BYTES(network_id));

        let mut rng = rand::thread_rng();
        let peer_id = rng.gen::<u64>();
        node_data.add(String::from("peer_id"), SectionValue::U64(peer_id));


        //payload_data
        let mut payload_data = Section::new();
        payload_data.add(String::from("cumulative_difficulty"), SectionValue::U64(1));
        payload_data.add(String::from("current_height"), SectionValue::U64(1));


        let genesis_hash = Hex::decode("418015bb9ae982a1975da7d79277c2705727a56894ba0fb246adaabb1f4632e3").unwrap();
        payload_data.add(String::from("top_id"), SectionValue::BYTES(genesis_hash));
        payload_data.add(String::from("top_version"), SectionValue::BYTES([1 as byte]));

        let mut section = Section::new();
        section.add(String::from("node_data"), SectionValue::SECTION(node_data));
        section.add(String::from("payload_data"), SectionValue::SECTION(payload_data));
        return section;
    }
}
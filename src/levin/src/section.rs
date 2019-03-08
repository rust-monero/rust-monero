use std::collections::HashMap;

pub struct Section {
    entries: HashMap<String, SectionValue>
}

pub enum SectionValue {
    STRING(String),
    BYTES([u8]),
    LIST(Vec<SectionValue>),
}


impl Section {

}
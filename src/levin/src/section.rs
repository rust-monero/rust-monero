extern crate bytes;
extern crate hex;
extern crate rand;

use std::collections::HashMap;

use bytes::{Buf, BufMut, BytesMut, Bytes};
use chrono::Utc;
use rand::Rng;

use crate::{LevinError, raw_size};
use crate::*;

// constants copied from monero p2p & epee

const PORTABLE_STORAGE_SIGNATUREA: u32 = 0x01011101;
const PORTABLE_STORAGE_SIGNATUREB: u32 = 0x01020101;

const PORTABLE_STORAGE_FORMAT_VER: u8 = 1;


// do not let string be so big
const MAX_STRING_LEN_POSSIBLE: u64 = 2000000000;

// data types
const SERIALIZE_TYPE_INT64: u8 = 1;
const SERIALIZE_TYPE_INT32: u8 = 2;
const SERIALIZE_TYPE_INT16: u8 = 3;
const SERIALIZE_TYPE_INT8: u8 = 4;
const SERIALIZE_TYPE_UINT64: u8 = 5;
const SERIALIZE_TYPE_UINT32: u8 = 6;
const SERIALIZE_TYPE_UINT16: u8 = 7;
const SERIALIZE_TYPE_UINT8: u8 = 8;
const SERIALIZE_TYPE_DOUBLE: u8 = 9;
const SERIALIZE_TYPE_STRING: u8 = 10;
const SERIALIZE_TYPE_BOOL: u8 = 11;
const SERIALIZE_TYPE_OBJECT: u8 = 12;
const SERIALIZE_TYPE_ARRAY: u8 = 13;

const SERIALIZE_FLAG_ARRAY: u8 = 0x80;


pub enum SectionValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Double(f64),
    Bool(bool),
    Bytes(Vec<u8>),
    List(Vec<SectionValue>),
    Section(Section),
}

impl SectionValue {
    fn read(buf: &mut Buf) -> Result<SectionValue, LevinError> {
        let serialize_type = buf.get_u8();
        SectionValue::read_with_type(buf, serialize_type)
    }

    fn read_with_type(buf: &mut Buf, serialize_type: u8) -> Result<SectionValue, LevinError> {
        let entry = match serialize_type {
            SERIALIZE_TYPE_INT8 => {
                ensure_eof!(buf, 1);
                SectionValue::I8(buf.get_i8())
            }
            SERIALIZE_TYPE_INT16 => {
                ensure_eof!(buf, 2);
                SectionValue::I16(buf.get_i16_le())
            }
            SERIALIZE_TYPE_INT32 => {
                ensure_eof!(buf, 4);
                SectionValue::I32(buf.get_i32_le())
            }
            SERIALIZE_TYPE_INT64 => {
                ensure_eof!(buf, 8);
                SectionValue::I64(buf.get_i64_le())
            }
            SERIALIZE_TYPE_UINT8 => {
                ensure_eof!(buf, 1);
                SectionValue::U8(buf.get_u8())
            }
            SERIALIZE_TYPE_UINT16 => {
                ensure_eof!(buf, 2);
                SectionValue::U16(buf.get_u16_le())
            }
            SERIALIZE_TYPE_UINT32 => {
                ensure_eof!(buf, 4);
                SectionValue::U32(buf.get_u32_le())
            }
            SERIALIZE_TYPE_UINT64 => {
                ensure_eof!(buf, 8);
                SectionValue::U64(buf.get_u64_le())
            }
            SERIALIZE_TYPE_DOUBLE => {
                ensure_eof!(buf, 8);
                SectionValue::Double(buf.get_f64_le())
            }
            SERIALIZE_TYPE_STRING => {
                let b = read_buf::<Buf>(buf)?;
                SectionValue::Bytes(b)
            }
            SERIALIZE_TYPE_BOOL => {
                ensure_eof!(buf, 1);
                SectionValue::Bool(buf.get_u8() != 0)
            }
            SERIALIZE_TYPE_OBJECT => {
                SectionValue::read(buf)?
            }
            SERIALIZE_TYPE_ARRAY => {
                ensure_eof!(buf, 1);
                let serialize_type = buf.get_u8();
                if serialize_type & SERIALIZE_FLAG_ARRAY != SERIALIZE_FLAG_ARRAY {
                    panic!();
                }
                SectionValue::read_list(buf, serialize_type)?
            },
            _ => {
                return Err(LevinError::InvalidSerializeType(serialize_type));
            }
        };
        Ok(entry)
    }

    fn read_list(buf: &mut Buf, serialize_type: u8) -> Result<SectionValue, LevinError> {
        let origin_type = serialize_type;
        if serialize_type & SERIALIZE_FLAG_ARRAY != SERIALIZE_FLAG_ARRAY {
            return Err(LevinError::ErrorArrayType(serialize_type));
        } else {
            serialize_type &= !SERIALIZE_FLAG_ARRAY;
        }
        let size = raw_size::read(buf)?;

        let mut list: Vec<SectionValue> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(SectionValue::read_with_type(buf, serialize_type)?)
        }
        Ok(SectionValue::List(list))
    }
    //copy from xmr
    fn write(&self, buf: &mut BytesMut) {
        match self {
            SectionValue::U8(v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_UINT8);
                buf.put_u8(*v);
            }
            SectionValue::U16(v) => {
                buf.reserve(3);
                buf.put_u8(SERIALIZE_TYPE_UINT16);
                buf.put_u16_le(*v);
            }
            SectionValue::U32(v) => {
                buf.reserve(5);
                buf.put_u8(SERIALIZE_TYPE_UINT32);
                buf.put_u32_le(*v);
            }
            SectionValue::U64(v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_UINT64);
                buf.put_u64_le(*v)
            }
            SectionValue::I8(v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_INT8);
                buf.put_i8(*v);
            }
            SectionValue::I16(v) => {
                buf.reserve(3);
                buf.put_u8(SERIALIZE_TYPE_INT16);
                buf.put_i16_le(*v);
            }
            SectionValue::I32(v) => {
                buf.reserve(5);
                buf.put_u8(SERIALIZE_TYPE_INT32);
                buf.put_i32_le(*v);
            }
            SectionValue::I64(v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_INT64);
                buf.put_i64_le(*v);
            }
            SectionValue::Double(v) => {
                buf.reserve(9);
                buf.put_u8(SERIALIZE_TYPE_DOUBLE);
                buf.put_f64_le(*v);
            }
            SectionValue::Bool(v) => {
                buf.reserve(2);
                buf.put_u8(SERIALIZE_TYPE_BOOL);
                buf.put_u8(if *v == false { 0 } else { 1 });
            }
            SectionValue::Bytes(v) => {
                buf.reserve(1);
                buf.put_u8(SERIALIZE_TYPE_STRING);
                write_buf(buf, v)
            }
            SectionValue::List(v) => {
                //TODO
//                buf.reserve(1);
//                buf.put_u8(SERIALIZE_TYPE_ARRAY);
//                Array::write(buf, v)
            }
            SectionValue::Section(v) => {
                buf.reserve(1);
                buf.put_u8(SERIALIZE_TYPE_OBJECT);
                Section::write(v, buf);
            }
        }
    }
}


pub struct Section {
    pub entries: HashMap<String, SectionValue>
}

impl Section {
    pub fn new() -> Section {
        Section {
            entries: HashMap::new()
        }
    }

    pub fn add(&mut self, key: String, value: SectionValue) {
        self.entries.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&SectionValue> {
        self.entries.get(key)
    }

    pub fn write(&self, buf: &mut BytesMut) {
        raw_size::write(buf, self.entries.len());
        for (name, entry) in self.entries.iter() {
            write_name(buf, name);
            entry.write(buf);
        }
    }

    fn handshake_request() -> Section {
        let mut node_data = Section::new();
        node_data.add(String::from("local_time"), SectionValue::U64(Utc::now().timestamp_millis() as u64));
        node_data.add(String::from("my_port"), SectionValue::U32(0));

        //TODO hex convert something
        //mainnet
        let network_id = hex::decode("1230f171610441611731008216a1a110").unwrap();
        node_data.add(String::from("network_id"), SectionValue::Bytes(network_id));

        let mut rng = rand::thread_rng();
        let peer_id = rng.gen::<u64>();
        node_data.add(String::from("peer_id"), SectionValue::U64(peer_id));


        //payload_data
        let mut payload_data = Section::new();
        payload_data.add(String::from("cumulative_difficulty"), SectionValue::U64(1));
        payload_data.add(String::from("current_height"), SectionValue::U64(1));


        let genesis_hash = hex::decode("418015bb9ae982a1975da7d79277c2705727a56894ba0fb246adaabb1f4632e3").unwrap();
        payload_data.add(String::from("top_id"), SectionValue::Bytes(genesis_hash));
        payload_data.add(String::from("top_version"), SectionValue::Bytes(vec!(1 as u8)));

        let mut section = Section::new();
        section.add(String::from("node_data"), SectionValue::Section(node_data));
        section.add(String::from("payload_data"), SectionValue::Section(payload_data));
        return section;
    }
}


fn read_name<B: Buf>(buf: &mut B) -> Result<String, LevinError> {
    ensure_eof!(buf, 1);
    let length = buf.get_u8() as usize;
    ensure_eof!(buf, length);

    let s = String::from_utf8_lossy(&buf.bytes()[..length]).into_owned();
    buf.advance(length);
    Ok(s)
}

fn read_buf<B: Buf>(buf: &mut B) -> Result<Vec<u8>, LevinError> {
    let length = raw_size::read(buf)?;
    ensure_eof!(buf, length);

    let mut b = Vec::with_capacity(length);
    b.extend_from_slice(&buf.bytes()[..length]);
    buf.advance(length);
    Ok(b)
}

fn write_buf(buf: &mut BytesMut, b: &Vec<u8>) {
    raw_size::write(buf, b.len());

    buf.reserve(b.len());
    buf.put(b.as_slice());
}

fn write_name(buf: &mut BytesMut, name: &str) {
    buf.reserve(name.as_bytes().len() + 1);
    buf.put_u8(name.as_bytes().len() as u8);
    buf.put(name.as_bytes());
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::section::{Section, SectionValue};

    #[test]
    fn it_works() {
        let s = Section::handshake_request();
        assert_eq!(false, s.entries.is_empty());

        let v = s.get(&String::from("node_data")).unwrap();

        match v {
            &SectionValue::Section(ref p) => {
                println!("success");
            }
            _ => {
                println!("error");
            }
        }

        match v {
            SectionValue::Section(_) => println!("success"),
            _ => println!("failed")
        }
    }
}
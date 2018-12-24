pub enum DeviceMode{
    NONE,
    TRANSACTION_CREATE_REAL,
    TRANSACTION_CREATE_FAKE,
    TRANSACTION_PARSE
}

pub enum DeviceType {
    SOFTWARE,
    LEDGER,
    TREZOR
}

impl DeviceType {
    fn value(&self) -> i32 {
        match self {
            DeviceType::SOFTWARE => 0,
            DeviceType::LEDGER => 1,
            DeviceType::TREZOR => 2
        }
    }
}

pub enum DeviceProtocol {
    PROTOCOL_DEFAULT,
    PROTOCOL_PROXY,     // Originally defined by Ledger
    PROTOCOL_COLD,      // Originally defined by Trezor
}

pub struct Device {
    name: String,
    mode: DeviceMode

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

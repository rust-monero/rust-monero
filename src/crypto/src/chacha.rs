pub const CHACHA_KEY_SIZE: usize = 32;
pub const CHACHA_IV_SIZE:usize = 8;

pub struct ChaChaIV {
    data: [u8; CHACHA_IV_SIZE]
}
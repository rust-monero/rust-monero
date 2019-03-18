pub const HASH_SIZE: usize = 32;

pub struct Hash(pub [u8; HASH_SIZE]);

impl Default for Hash {
    fn default() -> Self {
        Hash([0; 32])
    }
}

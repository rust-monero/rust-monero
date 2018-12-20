use xmr_crypto::hash::Hash;

pub struct TxInToScript {
    prev: Hash,
    prevout: u64,
    sigset: Vec<u8>
}
use xmr_crypto::crypto::KeyImage;

pub struct TxInToKey {
    amount: u64,
    key_offsets: Vec<u64>,
    k_image: KeyImage
}
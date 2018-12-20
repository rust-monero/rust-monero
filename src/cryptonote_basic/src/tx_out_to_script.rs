use xmr_crypto::crypto::PublicKey;

pub struct TxOutToScript {
    keys: Vec<PublicKey>,
    script: Vec<u8>
}
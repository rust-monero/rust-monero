use xmr_crypto::crypto::PublicKey;

struct TxOutToScript {
    keys: Vec<PublicKey>,
    script: Vec<u8>
}
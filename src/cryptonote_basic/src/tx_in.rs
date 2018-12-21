use xmr_crypto::hash::Hash;
use crate::tx_out::TxOutToScriptHash;
use xmr_crypto::crypto::KeyImage;

pub struct TxInGen {
    height: u64,
}

pub struct TxInToScript {
    prev: Hash,
    prevout: u64,
    sigset: Vec<u8>
}

pub struct TxInToScriptHash {
    prev: Hash,
    prevout: u64,
    script: TxOutToScriptHash,
    sigset: Vec<u8>
}

pub struct TxInToKey {
    amount: u64,
    key_offsets: Vec<u64>,
    k_image: KeyImage
}

pub enum TxIn {
    TxInGen(TxInGen),
    TxInToScript(TxInToScript),
    TxInToScriptHash(TxInToScriptHash),
    TxInToKey(TxInToKey)
}
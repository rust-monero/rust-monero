use xmr_crypto::hash::Hash;
use crate::tx_out_to_script_hash::TxOutToScriptHash;

pub struct TxInToScriptHash {
    prev: Hash,
    prevout: u64,
    script: TxOutToScriptHash,
    sigset: Vec<u8>
}
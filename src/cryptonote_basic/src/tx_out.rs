use xmr_crypto::crypto::PublicKey;
use xmr_crypto::hash::Hash;

pub struct TxOutToScript {
    keys: Vec<PublicKey>,
    script: Vec<u8>
}

pub struct TxOutToScriptHash {
    hash: Hash
}

pub struct TxOutToKey {
    key: PublicKey
}

pub enum TxOutTarget{
    TxOutToScript(TxOutToScript),
    TxOutToScriptHash(TxOutToScriptHash),
    TxOutToKey(TxOutToKey)
}

pub struct TxOut {
    amount: u64,
    target: TxOutTarget
}
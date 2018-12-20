use crate::tx_in_gen::TxInGen;
use crate::tx_in_to_script::TxInToScript;
use crate::tx_in_to_script_hash::TxInToScriptHash;
use crate::tx_in_to_key::TxInToKey;

pub enum TxIn {
    TxInGen(TxInGen),
    TxInToScript(TxInToScript),
    TxInToScriptHash(TxInToScriptHash),
    TxInToKey(TxInToKey)
}
use crate::tx_out_to_script::TxOutToScript;
use crate::tx_out_to_script_hash::TxOutToScriptHash;
use crate::tx_out_to_key::TxOutToKey;

pub enum TxOutTarget{
    TxOutToScript(TxOutToScript),
    TxOutToScriptHash(TxOutToScriptHash),
    TxOutToKey(TxOutToKey)
}
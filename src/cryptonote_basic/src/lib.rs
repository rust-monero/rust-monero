extern crate xmr_crypto;

use xmr_crypto::crypto::Signature;
use xmr_crypto::hash::Hash;

//use crate::tx_in_gen::TxInGen;
//use crate::tx_in_to_key::TxInToKey;
//use crate::tx_in_to_script::TxInToScript;
//use crate::tx_in_to_script_hash::TxInToScriptHash;
//use crate::tx_out_to_key::TxOutToKey;
//use crate::tx_out_to_script::TxOutToScript;
//use crate::tx_out_to_script_hash::TxOutToScriptHash;

pub mod tx_out_to_script;
pub mod tx_out_to_script_hash;
pub mod tx_out_to_key;
pub mod tx_out_target;

pub mod tx_in_gen;
pub mod tx_in_to_script;
pub mod tx_in_to_script_hash;
pub mod tx_in_to_key;
pub mod tx_in;

pub struct RingSignature(pub Vec<Signature>);


extern crate xmr_crypto;

pub use tx_in::TxIn;
pub use tx_in_gen::TxInGen;
pub use tx_in_to_key::TxInToKey;
pub use tx_in_to_script::TxInToScript;
pub use tx_in_to_script_hash::TxInToScriptHash;
pub use tx_out_target::TxOutTarget;
pub use tx_out_to_key::TxOutToKey;
pub use tx_out_to_script::TxOutToScript;
pub use tx_out_to_script_hash::TxOutToScriptHash;
use xmr_crypto::crypto::Signature;
use xmr_crypto::hash::Hash;

mod tx_out_to_script;
mod tx_out_to_script_hash;
mod tx_out_to_key;
mod tx_out_target;

mod tx_in_gen;
mod tx_in_to_script;
mod tx_in_to_script_hash;
mod tx_in_to_key;
mod tx_in;

pub struct RingSignature(pub Vec<Signature>);


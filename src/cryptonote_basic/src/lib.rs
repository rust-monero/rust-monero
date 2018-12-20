extern crate xmr_crypto;

use xmr_crypto::crypto::Signature;
use xmr_crypto::hash::Hash;

pub mod tx_out_to_script;
pub mod tx_out_to_script_hash;
pub mod tx_out_to_key;

pub struct RingSignature(pub Vec<Signature>);


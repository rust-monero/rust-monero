extern crate xmr_crypto;

use xmr_crypto::crypto::Signature;

pub mod tx_out_to_script;

pub struct RingSignature(pub Vec<Signature>);


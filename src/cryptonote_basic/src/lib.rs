extern crate xmr_crypto;

use xmr_crypto::crypto::Signature;
use xmr_crypto::hash::Hash;

mod tx_out;

mod tx_in;

mod transaction_prefix;
mod transaction;

pub struct RingSignature(pub Vec<Signature>);


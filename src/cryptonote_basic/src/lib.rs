extern crate xmr_crypto;

use xmr_crypto::crypto::Signature;

mod tx_out;
mod tx_in;
mod transaction_prefix;
mod transaction;
mod block;
mod account;
mod difficulty;
mod hard_fork;

pub struct RingSignature(pub Vec<Signature>);


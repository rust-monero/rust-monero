extern crate crypto;

use crypto::crypto::Signature;

pub mod tx_out;
pub mod tx_in;
pub mod transaction_prefix;
pub mod transaction;
pub mod block;
pub mod account;
pub mod difficulty;
pub mod hard_fork;
pub mod miner;
pub mod subaddress_index;
pub mod tx_extra;
pub mod verification_context;

pub struct RingSignature(pub Vec<Signature>);

pub struct BlobData(pub String);


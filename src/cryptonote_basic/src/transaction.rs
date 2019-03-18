use crate::transaction_prefix::TransactionPrefix;
use crypto::crypto::Signature;
use crypto::hash::Hash;
use std::sync::atomic::AtomicBool;

pub struct Transaction {
    prefix: TransactionPrefix,
    hash_valid: AtomicBool,
    blob_size_valid: AtomicBool,

    signatures: Vec<Vec<Signature>>,
    //TODO  ringct
    //rct::rctSig rct_signatures;
    hash: Hash,
    blob_size: usize,
}

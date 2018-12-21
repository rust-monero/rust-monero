use crate::transaction_prefix::TransactionPrefix;
use std::sync::atomic::AtomicBool;
use xmr_crypto::crypto::Signature;

pub struct Transaction {
    prefix: TransactionPrefix,
    hash_valid: AtomicBool,
    blob_size_valid: AtomicBool,

    signatures: Vec<Vec<Signature>>,

}
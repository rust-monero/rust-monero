use std::sync::atomic::AtomicBool;

use xmr_crypto::crypto::PublicKey;
use xmr_crypto::hash::Hash;

use crate::transaction::Transaction;
use xmr_crypto::crypto::SecretKey;

pub struct BlockHeader {
    major_version: u8,
    minor_version: u8,
    timestamp: u64,
    prev_id: Hash,
    nonce: u32
}

pub struct Block {
    block_header: BlockHeader,
    hash_valid: AtomicBool,
    miner_tx: Transaction,
    tx_hashes: Vec<Hash>,
    //hash cash
    hash: Hash,
}

pub struct AccountPublicAddress {
    spendPublicKey: PublicKey,
    vewPublicKey: PublicKey
}

pub struct KeyPair {
    public_key: PublicKey,
    secret_key: SecretKey
}
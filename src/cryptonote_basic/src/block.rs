use xmr_crypto::hash::Hash;
use std::sync::atomic::AtomicBool;
use crate::transaction::Transaction;

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
    hash: Hash
}
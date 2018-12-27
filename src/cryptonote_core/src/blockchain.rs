use cryptonote_basic::transaction::Transaction;
use cryptonote_basic::block::Block;
use cryptonote_basic::difficulty::DifficultyType;
use db::blockchain_db::BlockChainDB;
use crate::tx_pool::TxMemoryPool;
use std::collections::HashMap;
use crypto::hash::Hash;
use std::iter::Map;

enum BlockchainDbSyncMode {
    // user didn't specify, use db_async
    DB_DEFAULTSYNC,
    // handle syncing calls instead of the backing db, synchronously
    DB_SYNC,
    // handle syncing calls instead of the backing db, asynchronously
    DB_ASYNC,
    // Leave syncing up to the backing db (safest, but slowest because of disk I/O)
    DB_NOSYNC,
}

struct TransactionChainEntry {
    tx: Transaction,
    keeper_block_height: u64,
    blob_size: usize,
    global_output_indexes: Vec<u64>
}

struct BlockExtendedInfo {
    bl: Block,
    height: u64,
    block_cumulative_weight: usize,
    cumulative_difficulty: DifficultyType,
    already_generated_coins: u64
}

pub struct Blockchain {
    db: Box<BlockChainDB>,
    tx_pool: Box<TxMemoryPool>,
    //mutable epee::critical_section m_blockchain_lock; // TODO: add here reader/writer lock
    current_block_cumul_weight_limit: usize,
    current_block_cumul_weight_median: usize,
    //TODO

}
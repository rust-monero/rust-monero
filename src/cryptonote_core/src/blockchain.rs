use std::collections::HashMap;
use std::iter::Map;

use checkpoints::Checkpoints;
use crypto::crypto::KeyImage;
use crypto::hash::Hash;
use cryptonote_basic::block::Block;
use cryptonote_basic::difficulty::DifficultyType;
use cryptonote_basic::transaction::Transaction;
use db::blockchain_db::BlockChainDB;
use db::blockchain_db::OutputData;

use crate::tx_pool::TxMemoryPool;
use cryptonote_basic::hard_fork::HardFork;

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
    global_output_indexes: Vec<u64>,
}

struct BlockExtendedInfo {
    bl: Block,
    height: u64,
    block_cumulative_weight: usize,
    cumulative_difficulty: DifficultyType,
    already_generated_coins: u64,
}

pub struct Blockchain {
    db: Box<BlockChainDB>,
    tx_pool: Box<TxMemoryPool>,
    //mutable epee::critical_section m_blockchain_lock; // TODO: add here reader/writer lock
    current_block_cumul_weight_limit: usize,
    current_block_cumul_weight_median: usize,
    scan_table: HashMap<Hash, HashMap<KeyImage, Vec<OutputData>>>,
    blocks_longhash_table: HashMap<Hash, Hash>,
    check_txin_table: HashMap<Hash, HashMap<KeyImage, bool>>,
    blocks_hash_of_hashes: Vec<Hash>,
    blocks_hash_check: Vec<Hash>,
    blocks_txs_check: Vec<Hash>,
    db_sync_mode: BlockchainDbSyncMode,
    fast_sync: bool,
    show_time_stats: bool,
    db_default_sync: bool,
    db_sync_on_blocks: bool,

    db_sync_threshold: u64,
    max_prepare_blocks_threads: u64,
    fake_pow_calc_time: u64,
    fake_scan_time: u64,
    sync_counter: u64,
    bytes_to_sync: u64,
    timestamps: Vec<u64>,
    difficulties: Vec<DifficultyType>,
    timestamps_and_difficulties_height: u64,
    //TODO
    //epee::critical_section m_difficulty_lock;
    difficulty_for_next_block_top_hash: Hash,
    difficulty_for_next_block: DifficultyType,


    //TODO
    //boost::asio::io_service m_async_service;
    //boost::thread_group m_async_pool;
    //std::unique_ptr<boost::asio::io_service::work> m_async_work_idle;

    // all alternative chains  crypto::hash -> block_extended_info
    alternative_chains: HashMap<Hash, BlockExtendedInfo>,
    // some invalid blocks // crypto::hash -> block_extended_info
    invalid_blocks: HashMap<Hash, BlockExtendedInfo>,

    checkpoints: Checkpoints,
    enforce_dns_checkpoints: bool,

    hard_fork: HardFork,
    //TODO
    //nettype
}
use crypto::crypto::PublicKey;
use crypto::hash::Hash;
use cryptonote_basic::block::Block;
use cryptonote_basic::difficulty::DifficultyType;
use cryptonote_basic::transaction::Transaction;
use cryptonote_basic::hard_fork::HardFork;

pub struct OutputData {
    pubkey: PublicKey,
    unlock_time: u64,
    height: u64,
//    commitment: rct:key;
}

pub struct TxData {
    pub tx_id: u64,
    pub unlock_time: u64,
    pub block_id: u64,
}

pub struct TxpoolTxMeta {
    max_used_block_id: Hash,
    last_failed_id: Hash,
    weight: u64,
    fee: u64,
    max_used_block_height: u64,
    last_failed_height: u64,
    receive_time: u64,
    last_relayed_time: u64,
    // 112 bytes
    kept_by_block: u8,
    relayed: u8,
    do_not_relay: u8,
    //default 1
    double_spend_seen: u8,
    //default 7
    bf_padding: u8,
    padding: [u8; 76], // till 192 bytes
}

impl Default for TxpoolTxMeta {
    fn default() -> Self {
        TxpoolTxMeta {
            max_used_block_id: Hash::default(),
            last_failed_id: Hash::default(),
            weight: 0,
            fee: 0,
            max_used_block_height: 0,
            last_failed_height: 0,
            receive_time: 0,
            last_relayed_time: 0,
            kept_by_block: 0,
            relayed: 0,
            do_not_relay: 0,
            double_spend_seen: 1,
            bf_padding: 7,
            padding: [0; 76],
        }
    }
}

pub const DFB_SAFE: i32 = 1;
pub const DBF_FAST: i32 = 2;
pub const DBF_FASTEST: i32 = 4;
pub const DBF_RDONLY: i32 = 8;
pub const DBF_SALVAGE: i32 = 0x10;

pub struct BlockChainDBInfo {
    pub folder: String,
    //performance metric
    pub num_calls: u64,
    pub time_blk_hash: u64,
    pub time_add_block1: u64,
    pub time_add_transaction: u64,

    pub time_tx_exists: u64,
    pub time_commit1: u64,
    pub auto_remove_logs: bool,
    pub hardFork: Option<HardFork>,
}

pub trait BlockChainDB {
    /**
     * @brief store the transaction and its metadata
     *
     * The subclass implementing this will add the specified transaction data
     * to its backing store.  This includes only the transaction blob itself
     * and the other data passed here, not the separate outputs of the
     * transaction.
     *
     * It returns a tx ID, which is a mapping from the tx_hash. The tx ID
     * is used in #add_tx_amount_output_indices().
     *
     * If any of this cannot be done, the subclass should throw the corresponding
     * subclass of DB_EXCEPTION
     *
     * @param blk_hash the hash of the block containing the transaction
     * @param tx the transaction to be added
     * @param tx_hash the hash of the transaction
     * @param tx_prunable_hash the hash of the prunable part of the transaction
     * @return the transaction ID
     */
    fn add_block(&self, blk: &Block, block_weight: usize,
                 cumulative_difficulty: DifficultyType, coins_generated: u64, num_rct_outs: u64,
                 blk_hash: &Hash);

    /**
     * @brief remove data about the top block
     *
     * The subclass implementing this will remove the block data from the top
     * block in the chain.  The data to be removed is that which was added in
     * BlockchainDB::add_block(const block& blk, size_t block_weight, const difficulty_type& cumulative_difficulty, const uint64_t& coins_generated, const crypto::hash& blk_hash)
     *
     * If any of this cannot be done, the subclass should throw the corresponding
     * subclass of DB_EXCEPTION
     */
    fn remove_block(&self);

    /**
     * @brief store the transaction and its metadata
     *
     * The subclass implementing this will add the specified transaction data
     * to its backing store.  This includes only the transaction blob itself
     * and the other data passed here, not the separate outputs of the
     * transaction.
     *
     * It returns a tx ID, which is a mapping from the tx_hash. The tx ID
     * is used in #add_tx_amount_output_indices().
     *
     * If any of this cannot be done, the subclass should throw the corresponding
     * subclass of DB_EXCEPTION
     *
     * @param blk_hash the hash of the block containing the transaction
     * @param tx the transaction to be added
     * @param tx_hash the hash of the transaction
     * @param tx_prunable_hash the hash of the prunable part of the transaction
     * @return the transaction ID
     */
    fn add_transaction_data(&self, blk_hash: &Hash, tx: &Transaction, tx_hash: &Hash, tx_prunable_hash: &Hash) -> u64;

    /**
     * @brief remove data about a transaction
     *
     * The subclass implementing this will remove the transaction data
     * for the passed transaction.  The data to be removed was added in
     * add_transaction_data().  Additionally, current subclasses have behavior
     * which requires the transaction itself as a parameter here.  Future
     * implementations should note that this parameter is subject to be removed
     * at a later time.
     *
     * If any of this cannot be done, the subclass should throw the corresponding
     * subclass of DB_EXCEPTION
     *
     * @param tx_hash the hash of the transaction to be removed
     * @param tx the transaction
     */
    fn remove_transaction_data(&self, tx_hash: &Hash, tx: &Transaction);
}
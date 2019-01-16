use std::sync::Arc;
use std::sync::atomic::AtomicPtr;
use std::thread::Thread;

use lmdb::Database;
use lmdb::Environment;
use lmdb::RwCursor;
use lmdb::RwTransaction;

pub struct MdbTxnCursors<'txn> {
    pub txc_blocks: RwCursor<'txn>,
    pub txc_block_heights: RwCursor<'txn>,
    pub txc_block_info: RwCursor<'txn>,

    pub txc_output_txs: RwCursor<'txn>,
    pub txc_output_amounts: RwCursor<'txn>,

    pub txc_txs: RwCursor<'txn>,
    pub txc_txs_pruned: RwCursor<'txn>,
    pub txc_txs_prunable: RwCursor<'txn>,
    pub txc_txs_prunable_hash: RwCursor<'txn>,
    pub txc_tx_indices: RwCursor<'txn>,
    pub txc_tx_outputs: RwCursor<'txn>,

    pub txc_spent_keys: RwCursor<'txn>,

    pub txc_txpool_meta: RwCursor<'txn>,
    pub txc_txpool_blob: RwCursor<'txn>,
    pub txc_hf_versions: RwCursor<'txn>,
}

pub struct MdbRflags {
    pub rf_txn: bool,
    pub rf_blocks: bool,
    pub rf_block_heights: bool,
    pub rf_block_info: bool,
    pub rf_output_txs: bool,
    pub rf_output_amounts: bool,
    pub rf_txs: bool,
    pub rf_txs_pruned: bool,
    pub rf_txs_prunable: bool,
    pub rf_txs_prunable_hash: bool,
    pub rf_tx_indices: bool,
    pub rf_tx_outputs: bool,
    pub rf_spent_keys: bool,
    pub rf_txpool_meta: bool,
    pub rf_txpool_blob: bool,
    pub rf_hf_versions: bool,
}

pub struct MdbThreadInfo<'env, 'txn> {
    pub ti_rtxn: RwTransaction<'env>,
    pub ti_rcursors: MdbTxnCursors<'txn>,
    pub ti_rflags: MdbRflags,
}

pub struct MdbTxnSafe<'env, 'txn> {
    tinfo: MdbThreadInfo<'env, 'txn>,
    txn: RwTransaction<'env>,
    batch_txn: bool,
    check: bool,

}

impl<'env, 'txn> MdbTxnSafe<'env, 'txn> {}

pub struct BlockchainLMDB<'env, 'txn> {
    pub env: Environment,

    pub blocks: Database,
    pub block_heights: Database,
    pub block_info: Database,

    pub txs: Database,
    pub txs_pruned: Database,
    pub txs_prunable: Database,
    pub txs_prunable_hash: Database,
    pub tx_indices: Database,
    pub tx_outputs: Database,

    pub output_txs: Database,
    pub output_amounts: Database,

    pub spent_keys: Database,

    pub txpool_meta: Database,
    pub txpool_blob: Database,

    pub hf_starting_heights: Database,
    pub hf_versions: Database,

    pub properties: Database,

    cum_size: u64,
    cum_count: u32,
    folder: String,
    write_txn: MdbTxnSafe<'env, 'txn>,
    write_batch_txn: MdbTxnSafe<'env, 'txn>,
    //  boost::thread::id m_writer;

    batch_transactions: bool,
    batch_active: bool,
    wcursors: MdbTxnCursors<'txn>,
    //  mutable boost::thread_specific_ptr<mdb_threadinfo> m_tinfo;
}


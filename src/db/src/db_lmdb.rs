use std::fs;
use std::fs::File;
use std::os::raw::c_uint;
use std::path::Path;
use std::sync::atomic::AtomicPtr;
use std::sync::Arc;
use std::thread::Thread;

use lmdb::Cursor;
use lmdb::Database;
use lmdb::DatabaseFlags;
use lmdb::Environment;
use lmdb::EnvironmentFlags;
use lmdb::RwCursor;
use lmdb::RwTransaction;
use lmdb::Transaction;
use lmdb_sys::*;

use crypto::hash::Hash;
use cryptonote_config::CRYPTONOTE_BLOCKCHAINDATA_FILENAME;
use cryptonote_config::CRYPTONOTE_BLOCKCHAINDATA_LOCK_FILENAME;

use crate::blockchain_db::BlockChainDBInfo;
use crate::blockchain_db::DBF_FAST;
use crate::blockchain_db::DBF_FASTEST;
use crate::blockchain_db::DBF_RDONLY;
use crate::blockchain_db::DBF_SALVAGE;

/* DB schema:
 *
 * Table            Key          Data
 * -----            ---          ----
 * blocks           block ID     block blob
 * block_heights    block hash   block height
 * block_info       block ID     {block metadata}
 *
 * txs_pruned       txn ID       pruned txn blob
 * txs_prunable     txn ID       prunable txn blob
 * txs_prunable_hash txn ID      prunable txn hash
 * tx_indices       txn hash     {txn ID, metadata}
 * tx_outputs       txn ID       [txn amount output indices]
 *
 * output_txs       output ID    {txn hash, local index}
 * output_amounts   amount       [{amount output index, metadata}...]
 *
 * spent_keys       input hash   -
 *
 * txpool_meta      txn hash     txn metadata
 * txpool_blob      txn hash     txn blob
 *
 * Note: where the data items are of uniform size, DUPFIXED tables have
 * been used to save space. In most of these cases, a dummy "zerokval"
 * key is used when accessing the table; the Key listed above will be
 * attached as a prefix on the Data to serve as the DUPSORT key.
 * (DUPFIXED saves 8 bytes per record.)
 *
 * The output_amounts table doesn't use a dummy key, but uses DUPSORT.
 */
const LMDB_BLOCKS: &str = "blocks";
const LMDB_BLOCK_HEIGHTS: &str = "block_heights";
const LMDB_BLOCK_INFO: &str = "block_info";

const LMDB_TXS: &str = "txs";
const LMDB_TXS_PRUNED: &str = "txs_pruned";
const LMDB_TXS_PRUNABLE: &str = "txs_prunable";
const LMDB_TXS_PRUNABLE_HASH: &str = "txs_prunable_hash";
const LMDB_TX_INDICES: &str = "tx_indices";
const LMDB_TX_OUTPUTS: &str = "tx_outputs";

const LMDB_OUTPUT_TXS: &str = "output_txs";
const LMDB_OUTPUT_AMOUNTS: &str = "output_amounts";
const LMDB_SPENT_KEYS: &str = "spent_keys";

const LMDB_TXPOOL_META: &str = "txpool_meta";
const LMDB_TXPOOL_BLOB: &str = "txpool_blob";

const LMDB_HF_STARTING_HEIGHTS: &str = "hf_starting_heights";
const LMDB_HF_VERSIONS: &str = "hf_versions";

const LMDB_PROPERTIES: &str = "properties";

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

//pub struct MdbThreadInfo<'env, 'txn> {
//    pub ti_rtxn: RwTransaction<'env>,
//    pub ti_rcursors: MdbTxnCursors<'txn>,
//    pub ti_rflags: MdbRflags,
//}
//
//pub struct MdbTxnSafe<'env, 'txn> {
//    tinfo: MdbThreadInfo<'env, 'txn>,
//    txn: RwTransaction<'env>,
//    batch_txn: bool,
//    check: bool,
//}

pub struct BlockchainLMDB<'env> {
    pub db: BlockChainDBInfo,

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
    write_txn: Option<RwTransaction<'env>>,
    write_batch_txn: Option<RwTransaction<'env>>,
    //  boost::thread::id m_writer;
    batch_transactions: bool,
    batch_active: bool,
    //    wcursors: Option<MdbTxnCursors<'txn>>, //may not need this
    //  mutable boost::thread_specific_ptr<mdb_threadinfo> m_tinfo;
}

impl<'env> BlockchainLMDB<'env> {
    pub fn open(filename: &str, db_flags: i32) -> BlockchainLMDB {
        let mut mdb_flags = EnvironmentFlags::NO_READAHEAD;
        let db_path = Path::new(filename);
        if db_path.exists() {
            if !db_path.is_dir() {
                panic!(
                    "LMDB needs a directory path, but a file was passed, filename = {}",
                    filename
                );
            }
        } else {
            match fs::create_dir_all(db_path) {
                Err(_) => panic!("Failed to create directory {}", filename),
                Ok(_) => info!("create file success"),
            }
        }
        let parent_path = db_path.parent().unwrap();
        if parent_path
            .join(CRYPTONOTE_BLOCKCHAINDATA_FILENAME)
            .exists()
            || parent_path
                .join(CRYPTONOTE_BLOCKCHAINDATA_LOCK_FILENAME)
                .exists()
        {
            error!(
                "Found existing LMDB files in {}",
                parent_path.to_str().unwrap()
            );
            error!(
                "Move {} and/or {} to {}, or delete them, and then restart",
                CRYPTONOTE_BLOCKCHAINDATA_FILENAME,
                CRYPTONOTE_BLOCKCHAINDATA_LOCK_FILENAME,
                filename
            );
            panic!("Database could not be opened");
        }
        if db_flags & DBF_FAST > 0 {
            mdb_flags = mdb_flags | EnvironmentFlags::NO_SYNC;
        } else if db_flags & DBF_FASTEST > 0 {
            mdb_flags = mdb_flags
                | EnvironmentFlags::NO_SYNC
                | EnvironmentFlags::WRITE_MAP
                | EnvironmentFlags::MAP_ASYNC;
        } else if db_flags & DBF_RDONLY > 0 {
            mdb_flags = mdb_flags | EnvironmentFlags::READ_ONLY;
        } else if db_flags & DBF_SALVAGE > 0 {
            //TODO update lmdb version
            //            mdb_flags = mdb_flags | EnvironmentFlags::MDB_PREVSNAPSHOT;
        }

        let mut env = Environment::new()
            .set_max_dbs(20)
            .set_max_readers(126) //TODO calculate from cpu core nums
            .set_flags(mdb_flags)
            .open(db_path)
            .expect("Failed to create lmdb environment");

        //TODO resize

        let mut txn = env
            .begin_rw_txn()
            .expect("Failed to create a transaction for the db");

        let blocks = unsafe {
            txn.create_db(Some(LMDB_BLOCKS), DatabaseFlags::INTEGER_KEY)
                .expect("Failed to open db handle for blocks")
        };
        let block_info = unsafe {
            txn.create_db(
                Some(LMDB_BLOCK_INFO),
                DatabaseFlags::INTEGER_KEY | DatabaseFlags::DUP_SORT | DatabaseFlags::DUP_FIXED,
            )
            .expect("Failed to open db handle for block_info")
        };
        let block_heights = unsafe {
            txn.create_db(
                Some(LMDB_BLOCK_HEIGHTS),
                DatabaseFlags::INTEGER_KEY | DatabaseFlags::DUP_SORT | DatabaseFlags::DUP_FIXED,
            )
            .expect("Failed to open db handle for block_heights")
        };
        let txs = unsafe {
            txn.create_db(Some(LMDB_TXS), DatabaseFlags::INTEGER_KEY)
                .expect("Failed to open db handle for txs")
        };
        let txs_pruned = unsafe {
            txn.create_db(Some(LMDB_TXS_PRUNED), DatabaseFlags::INTEGER_KEY)
                .expect("Failed to open db handle for txs_pruned")
        };
        let txs_prunable = unsafe {
            txn.create_db(Some(LMDB_TXS_PRUNABLE), DatabaseFlags::INTEGER_KEY)
                .expect("Failed to open db handle for txs_prunable")
        };
        let txs_prunable_hash = unsafe {
            txn.create_db(Some(LMDB_TXS_PRUNABLE_HASH), DatabaseFlags::INTEGER_KEY)
                .expect("Failed to open db handle for txs_prunable_hash")
        };
        let tx_indices = unsafe {
            txn.create_db(
                Some(LMDB_TX_INDICES),
                DatabaseFlags::INTEGER_KEY | DatabaseFlags::DUP_SORT | DatabaseFlags::DUP_FIXED,
            )
            .expect("Failed to open db handle for tx_indices")
        };
        let tx_outputs = unsafe {
            txn.create_db(Some(LMDB_TX_OUTPUTS), DatabaseFlags::INTEGER_KEY)
                .expect("Failed to open db handle for tx_outputs")
        };
        let output_txs = unsafe {
            txn.create_db(
                Some(LMDB_OUTPUT_TXS),
                DatabaseFlags::INTEGER_KEY | DatabaseFlags::DUP_SORT | DatabaseFlags::DUP_FIXED,
            )
            .expect("Failed to open db handle for output_txs")
        };
        let output_amounts = unsafe {
            txn.create_db(
                Some(LMDB_OUTPUT_AMOUNTS),
                DatabaseFlags::INTEGER_KEY | DatabaseFlags::DUP_SORT | DatabaseFlags::DUP_FIXED,
            )
            .expect("Failed to open db handle for output_amounts")
        };
        let spent_keys = unsafe {
            txn.create_db(
                Some(LMDB_SPENT_KEYS),
                DatabaseFlags::INTEGER_KEY | DatabaseFlags::DUP_SORT | DatabaseFlags::DUP_FIXED,
            )
            .expect("Failed to open db handle for spent_keys")
        };
        let txpool_meta = unsafe {
            txn.create_db(Some(LMDB_TXPOOL_META), DatabaseFlags::empty())
                .expect("Failed to open db handle for txpool_meta")
        };
        let txpool_blob = unsafe {
            txn.create_db(Some(LMDB_TXPOOL_BLOB), DatabaseFlags::empty())
                .expect("Failed to open db handle for txpool_blob")
        };
        let hf_starting_heights = unsafe {
            txn.create_db(Some(LMDB_HF_STARTING_HEIGHTS), DatabaseFlags::empty())
                .expect("Failed to open db handle for hf_starting_heights")
        };
        let hf_versions = unsafe {
            txn.create_db(Some(LMDB_HF_VERSIONS), DatabaseFlags::INTEGER_KEY)
                .expect("Failed to open db handle for hf_versions")
        };
        let properties = unsafe {
            txn.create_db(Some(LMDB_PROPERTIES), DatabaseFlags::empty())
                .expect("Failed to open db handle for properties")
        };

        //        let txc_blocks = txn.open_rw_cursor(blocks).unwrap();
        //        let txc_block_info = txn.open_rw_cursor(block_info).unwrap();
        //        let txc_block_heights = txn.open_rw_cursor(block_heights).unwrap();
        //        let txc_txs = txn.open_rw_cursor(txs).unwrap();
        //        let txc_txs_pruned = txn.open_rw_cursor(txs_pruned).unwrap();
        //        let txc_txs_prunable = txn.open_rw_cursor(txs_prunable).unwrap();
        //        let txc_txs_prunable_hash = txn.open_rw_cursor(txs_prunable_hash).unwrap();
        //        let txc_tx_indices = txn.open_rw_cursor(tx_indices).unwrap();
        //        let txc_tx_outputs = txn.open_rw_cursor(tx_outputs).unwrap();
        //        let txc_output_txs = txn.open_rw_cursor(output_txs).unwrap();
        //        let txc_output_amounts = txn.open_rw_cursor(output_amounts).unwrap();
        //        let txc_spent_keys = txn.open_rw_cursor(spent_keys).unwrap();
        //        let txc_txpool_meta = txn.open_rw_cursor(txpool_meta).unwrap();
        //        let txc_txpool_blob = txn.open_rw_cursor(txpool_blob).unwrap();
        //        let txc_hf_versions = txn.open_rw_cursor(hf_versions).unwrap();
        //
        //        let cursors = MdbTxnCursors {
        //            txc_blocks,
        //            txc_block_info,
        //            txc_block_heights,
        //            txc_txs,
        //            txc_txs_pruned,
        //            txc_txs_prunable,
        //            txc_txs_prunable_hash,
        //            txc_tx_indices,
        //            txc_tx_outputs,
        //            txc_output_txs,
        //            txc_output_amounts,
        //            txc_spent_keys,
        //            txc_txpool_meta,
        //            txc_txpool_blob,
        //            txc_hf_versions,
        //        };
        let t = txn.commit();

        let mut db = BlockchainLMDB {
            db: BlockChainDBInfo {
                folder: "".to_string(),
                num_calls: 0,
                time_blk_hash: 0,
                time_add_block1: 0,
                time_add_transaction: 0,
                time_tx_exists: 0,
                time_commit1: 0,
                auto_remove_logs: false,
                hardFork: None,
            },
            env,
            blocks,
            block_info,
            block_heights,
            txs,
            txs_pruned,
            txs_prunable,
            txs_prunable_hash,
            tx_indices,
            tx_outputs,
            output_txs,
            output_amounts,
            spent_keys,
            txpool_meta,
            txpool_blob,
            hf_starting_heights,
            hf_versions,
            properties,

            cum_size: 0,
            cum_count: 0,
            folder: String::from(filename),
            write_txn: None,
            write_batch_txn: None,
            batch_transactions: false,
            batch_active: false,
        };
        db
    }

    fn block_exists(self, h: &Hash, height: u64) -> Option<u64> {
        let txn = self
            .env
            .begin_ro_txn()
            .expect("get read only transaction failed when check block exists");
        let cursor = txn.open_ro_cursor(self.block_heights).unwrap();
        let result = cursor.get(Some(&h.0[..]), None, MDB_GET_BOTH);
        if result.is_ok() {
            let t = result.unwrap();

            Some(1)
        } else {
            None
        }
    }
}

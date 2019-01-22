use std::fs;
use std::fs::File;
use std::os::raw::c_uint;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicPtr;
use std::thread::Thread;

use lmdb::Database;
use lmdb::Environment;
use lmdb::EnvironmentFlags;
use lmdb::RwCursor;
use lmdb::RwTransaction;
use lmdb::ffi::*;

use cryptonote_config::CRYPTONOTE_BLOCKCHAINDATA_FILENAME;
use cryptonote_config::CRYPTONOTE_BLOCKCHAINDATA_LOCK_FILENAME;

use crate::blockchain_db::BlockChainDBInfo;
use crate::blockchain_db::DBF_FAST;
use crate::blockchain_db::DBF_FASTEST;
use crate::blockchain_db::DBF_RDONLY;
use crate::blockchain_db::DBF_SALVAGE;

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
    write_txn: MdbTxnSafe<'env, 'txn>,
    write_batch_txn: MdbTxnSafe<'env, 'txn>,
    //  boost::thread::id m_writer;

    batch_transactions: bool,
    batch_active: bool,
    wcursors: MdbTxnCursors<'txn>,
    //  mutable boost::thread_specific_ptr<mdb_threadinfo> m_tinfo;
}

impl<'env, 'txn> BlockchainLMDB<'env, 'txn> {
    fn open(&mut self, filename: &str, db_flags: i32) {
        let mut mdb_flags = EnvironmentFlags::NO_READAHEAD;
        let db_path = Path::new(filename);
        if db_path.exists() {
            if !db_path.is_dir() {
                panic!("LMDB needs a directory path, but a file was passed, filename = {}", filename);
            }
        } else {
            match fs::create_dir_all(db_path) {
                Err(_) => panic!("Failed to create directory {}", filename),
                Ok(_) => info!("create file success")
            }
        }
        let parent_path = db_path.parent().unwrap();
        if parent_path.join(CRYPTONOTE_BLOCKCHAINDATA_FILENAME).exists() ||
            parent_path.join(CRYPTONOTE_BLOCKCHAINDATA_LOCK_FILENAME).exists() {
            error!("Found existing LMDB files in {}", parent_path.to_str().unwrap());
            error!("Move {} and/or {} to {}, or delete them, and then restart",
                   CRYPTONOTE_BLOCKCHAINDATA_FILENAME, CRYPTONOTE_BLOCKCHAINDATA_LOCK_FILENAME, filename);
            panic!("Database could not be opened");
        }
        self.db.folder = String::from(filename);

        if db_flags & DBF_FAST > 0 {
            mdb_flags = mdb_flags | EnvironmentFlags::NO_SYNC;
        } else if db_flags & DBF_FASTEST > 0 {
            mdb_flags = mdb_flags | EnvironmentFlags::NO_SYNC | EnvironmentFlags::WRITE_MAP | EnvironmentFlags::MAP_ASYNC;
        } else if db_flags & DBF_RDONLY > 0 {
            mdb_flags = mdb_flags | EnvironmentFlags::READ_ONLY;
        } else if db_flags & DBF_SALVAGE > 0 {
            //TODO update lmdb version
//            mdb_flags = mdb_flags | EnvironmentFlags::MDB_PREVSNAPSHOT;
        }


        self.env = Environment::new()
            .set_max_dbs(20)
            .set_max_readers(126) //TODO calculate from cpu core nums
            .set_flags(mdb_flags)
            .open(db_path)
            .expect("Failed to create lmdb environment");

        let database = self.env.open_db(None)
            .expect("open db failed!");

//        let mut mei: MDB_envinfo = MDB_envinfo {
//            me_mapaddr: libc::c_void,
//            me_mapsize: 0,
//            me_last_pgno: 0,
//            me_last_txnid: 0,
//            me_maxreaders: 0,
//            me_numreaders: 0,
//        };
    }
}


use crate::db_lmdb::BlockchainLMDB;
use crate::blockchain_db::BlockChainDBInfo;
use lmdb::Environment;
use crate::db_lmdb::MdbTxnCursors;

#[test]
fn it_works() {
    info!("test1");
    warn!("test2 {}", "monero");
}

#[test]
fn lmdb_open() {
}
use crate::blockchain_db::BlockChainDBInfo;
use crate::db_lmdb::BlockchainLMDB;
use crate::db_lmdb::MdbTxnCursors;
use lmdb::Cursor;
use lmdb::Environment;
use lmdb::RwCursor;
use lmdb::Transaction;
use lmdb_sys::MDB_SET;

//use lmdb::ffi::MDB_FIRST;
//lmdb_sys::MDB_FIRST;

#[test]
fn it_works() {
    info!("test1");
    warn!("test2 {}", "monero");
}

#[test]
fn lmdb_open() {
    let db = BlockchainLMDB::open("/Users/line/.bitmonero/lmdb", 2);
    let tx = db.env.begin_ro_txn().unwrap();
    let c = tx.open_ro_cursor(db.blocks).unwrap();
    let d = c.get(Some(&b"key2"[..]), None, MDB_SET);
    assert_eq!(false, d.is_ok());
}

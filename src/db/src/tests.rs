use crate::db_lmdb::BlockchainLMDB;
use crate::blockchain_db::BlockChainDBInfo;
use lmdb::Environment;
use crate::db_lmdb::MdbTxnCursors;
use lmdb::Transaction;
use lmdb::RwCursor;
use lmdb::Cursor;

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
    let d = c.get(Some(&b"key2"[..]), None, 0);
    assert_eq!((None, &b""[..]), d.unwrap());
}
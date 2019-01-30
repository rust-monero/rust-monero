use crypto::crypto::PublicKey;
use crypto::hash::Hash;
use cryptonote_basic::difficulty::DifficultyType;

use crate::blockchain_db::TxData;
use crate::blockchain_db::OutputData;

pub struct MdbBlockInfoOld {
    height: u64,
    timestamp: u64,
    coins: u64,
    weight: u64,
    diff: DifficultyType,
    hash: Hash,
}


pub struct MdbBlockInfo {
    pub height: u64,
    pub timestamp: u64,
    pub coins: u64,
    pub weight: u64,
    pub diff: DifficultyType,
    pub hash: Hash,
    pub cumRct: u64,
}

pub struct BlockHeight {
    pub hash: Hash,
    pub height: u64,
}

pub struct TxIndex {
    pub key: Hash,
    pub data: TxData,
}

pub struct PreRctOutputData {
    //the output's public key (for spend verification)
    pub pub_key: PublicKey,
    //the output's unlock time (or height)
    pub unlock_time: u64,
    //the height of the block which created the output
    pub height: u64,
}

pub struct PreRctOutKey {
    pub amount_index: u64,
    pub output_id: u64,
    pub data: PreRctOutputData
}

pub struct OutKey {
    pub amount_index: u64,
    pub output_id: u64,
    pub data: OutputData
}

pub struct OutTx {
    pub output_id: u64,
    pub tx_hash: Hash,
    pub local_index: u64
}






use crypto::crypto::PublicKey;
use crypto::hash::Hash;

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
            padding: [0; 76]
        }
    }
}


pub trait BlockChainDB {}
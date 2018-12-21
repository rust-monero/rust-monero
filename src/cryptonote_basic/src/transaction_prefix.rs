use crate::TxIn;
use crate::tx_out::TxOut;

pub struct TransactionPrefix {
    version: u8,
    unlock_time: u64,
    vin: Vec<TxIn>,
    vout: Vec<TxOut>,
    extra: Vec<u8>
}
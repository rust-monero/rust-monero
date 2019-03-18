use crate::tx_in::TxIn;
use crate::tx_out::TxOut;

pub struct TransactionPrefix {
    version: usize,
    unlock_time: u64,
    vin: Vec<TxIn>,
    vout: Vec<TxOut>,
    extra: Vec<u8>,
}

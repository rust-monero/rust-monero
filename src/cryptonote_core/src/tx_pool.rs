use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use tokio::timer::Interval;

use crypto::crypto::KeyImage;
use crypto::hash::Hash;
use std::sync::atomic::AtomicPtr;

pub struct TxByFeeAndReceiveTimeEntry((f64, u64), Hash);

pub struct SortedTxContainer(BTreeSet<TxByFeeAndReceiveTimeEntry>);

pub struct KeyImagesContainer(HashMap<KeyImage, HashSet<Hash>>);

pub struct TxMemoryPool {
    spent_key_images: KeyImagesContainer,
    remove_stuck_tx_interval: Interval,
    txs_by_fee_and_receive_time: SortedTxContainer,
    cookie: AtomicPtr<u64>,
    timed_out_transactions: HashSet<Hash>,
//    blockChain: Blockchain
}
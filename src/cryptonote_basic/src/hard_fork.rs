use db::blockchain_db::BlockChainDB;
use std::collections::vec_deque::VecDeque;
use std::sync::Mutex;
use std::sync::Arc;

const DEFAULT_ORIGINAL_VERSION_TILL_HEIGHT: u64 = 0;
const DEFAULT_FORKED_TIME: i64 = 31557600;
const DEFAULT_UPDATE_TIME: i64 = 31557600 / 2;
const DEFAULT_WINDOW_SIZE: u64 = 10080;
const DEFAULT_THRESHOLD_PERCENT: u64 = 80;

pub enum HardForkState {
    LikelyForked,
    UpdateNeeded,
    Ready,
}

struct Params {
    version: u8,
    threshold: u8,
    height: u64,
    time: i64
}

pub struct HardFork {
    //TODO with sized
    db: Box<BlockChainDB>,
    forked_time: i64,
    update_time: i64,
    window_size: u64,
    default_threshold_percent: u8,
    original_version: u8,
    original_version_till_height: u64,
    heights: Vec<Params>,
    versions: VecDeque<u8>,
    last_versions:[i32; 256],
    current_fork_index: u32,

    //TODO  lock
    //mutable epee::critical_section lock;
}

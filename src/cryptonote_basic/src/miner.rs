use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicPtr;
use std::sync::Mutex;
use std::thread::Thread;

use tokio::timer::Interval;

use crate::BlobData;
use crate::block::AccountPublicAddress;
use crate::block::Block;
use crate::difficulty::DifficultyType;

const BACKGROUND_MINING_DEFAULT_IDLE_THRESHOLD_PERCENTAGE: u8 = 90;
const BACKGROUND_MINING_MIN_IDLE_THRESHOLD_PERCENTAGE: u8 = 50;
const BACKGROUND_MINING_MAX_IDLE_THRESHOLD_PERCENTAGE: u8 = 99;
const BACKGROUND_MINING_DEFAULT_MIN_IDLE_INTERVAL_IN_SECONDS: u16 = 10;
const BACKGROUND_MINING_MIN_MIN_IDLE_INTERVAL_IN_SECONDS: u16 = 10;
const BACKGROUND_MINING_MAX_MIN_IDLE_INTERVAL_IN_SECONDS: u16 = 3600;
const BACKGROUND_MINING_DEFAULT_MINING_TARGET_PERCENTAGE: u8 = 40;
const BACKGROUND_MINING_MIN_MINING_TARGET_PERCENTAGE: u8 = 5;
const BACKGROUND_MINING_MAX_MINING_TARGET_PERCENTAGE: u8 = 50;
const BACKGROUND_MINING_MINER_MONITOR_INVERVAL_IN_SECONDS: u8 = 10;
// ramp up
const BACKGROUND_MINING_DEFAULT_MINER_EXTRA_SLEEP_MILLIS: u64 = 400;
const BACKGROUND_MINING_MIN_MINER_EXTRA_SLEEP_MILLIS: u64 = 5;

struct MinerConfig {
    current_extra_message_index: u64
}

pub trait MinerHandler {
    fn handle_block_found(&self, b: &Block) -> bool;
    fn get_block_template(&self, b: &Block, adr: AccountPublicAddress, diffic: &DifficultyType,
                          height: u64, expected_reward: u64, ex_nonce: BlobData) -> bool;
}

struct Miner {
    stop: AtomicPtr<u32>,
    //TODO lock
    //epee::critical_section m_template_lock;
    template: Block,
    template_no: AtomicPtr<u32>,
    starter_nonce: AtomicPtr<u32>,
    diffic: DifficultyType,
    height: u64,
    thread_index: AtomicPtr<u32>,
    threads_total: AtomicPtr<u32>,
    pausers_count: AtomicPtr<i32>,
    //TODO epee::critical_section m_miners_count_lock;
    threads: Vec<Thread>,
    //TODO
    //epee::critical_section m_threads_lock;
    //TODO
    //i_miner_handler* m_phandler;
    phandler: Box<MinerHandler>,
    mine_address: AccountPublicAddress,
    update_block_template_interval: Interval,
    update_merge_hr_interval: Interval,
    extra_messages: Vec<BlobData>,
    config: MinerConfig,
    config_folder_path: String,
    last_hr_merge_time: AtomicPtr<u64>,
    hashes: AtomicPtr<u64>,
    current_hash_rate: AtomicPtr<u64>,
    //TODO
    //epee::critical_section m_last_hash_rates_lock;
    m_last_hash_rates: Vec<u64>,
    do_print_hashrate: bool,
    do_mining: bool,
    is_background_mining_enabled: AtomicBool,
    ignore_battery: bool,
    //TODO
    //boost::mutex m_is_background_mining_enabled_mutex;
    //boost::condition_variable m_is_background_mining_enabled_cond;
    is_background_mining_started: AtomicBool,
    //boost::condition_variable m_is_background_mining_started_cond;
    //boost::thread m_background_mining_thread;
    background_mining_thread: Thread,
    min_idle_seconds: u64,
    idle_threshold: u64,
    mining_target: u64,
    miner_extra_sleep: AtomicPtr<u64>,

}
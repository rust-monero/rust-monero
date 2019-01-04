pub const CRYPTONOTE_DNS_TIMEOUT_MS: u32 = 20000;

pub const CRYPTONOTE_MAX_BLOCK_NUMBER: u64 = 500000000;
// block header blob limit, never used!
pub const CRYPTONOTE_MAX_BLOCK_SIZE: u64 = 500000000;
//size of block (bytes) that is the maximum that miners will produce
pub const CRYPTONOTE_GETBLOCKTEMPLATE_MAX_BLOCK_SIZE: u64 = 196608;
pub const CRYPTONOTE_MAX_TX_SIZE: u64 = 1000000000;
pub const CRYPTONOTE_PUBLIC_ADDRESS_TEXTBLOB_VER: u64 = 0;
pub const CRYPTONOTE_MINED_MONEY_UNLOCK_WINDOW: u64 = 60;
pub const CURRENT_TRANSACTION_VERSION: u64 = 2;
pub const CURRENT_BLOCK_MAJOR_VERSION: u64 = 1;
pub const CURRENT_BLOCK_MINOR_VERSION: u64 = 0;
pub const CRYPTONOTE_BLOCK_FUTURE_TIME_LIMIT: u64 = 60 * 60 * 2;
pub const CRYPTONOTE_DEFAULT_TX_SPENDABLE_AGE: u64 = 10;

pub const BLOCKCHAIN_TIMESTAMP_CHECK_WINDOW: u64 = 10;

// MONEY_SUPPLY - total number coins to be generated
pub const MONEY_SUPPLY: i64 = -1;
pub const EMISSION_SPEED_FACTOR_PER_MINUTE: u64 = 20;
pub const FINAL_SUBSIDY_PER_MINUTE: u64 = 300000000000; //3 * pow(10, 11)

pub const CRYPTONOTE_REWARD_BLOCKS_WINDOW: u64 = 100;
//size of block (bytes) after which reward for block calculated using block size
pub const CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V2: u64 = 60000;
//size of block (bytes) after which reward for block calculated using block size - before first fork
pub const CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V1: u64 = 20000;
//size of block (bytes) after which reward for block calculated using block size - second change, from v5
pub const CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V5: u64 = 300000;
pub const CRYPTONOTE_COINBASE_BLOB_RESERVED_SIZE: u64 = 600;
pub const CRYPTONOTE_DISPLAY_DECIMAL_POINT: u64 = 12;

// COIN - number of smallest units in one coin

pub const COIN: u64 = 1000000000000;// pow(10, 12)

pub const FEE_PER_KB_OLD: u64 = 10000000000;//pow(10, 10)

pub const FEE_PER_KB: u64 = 2000000000;// 2 * pow(10, 9)

pub const FEE_PER_BYTE: u64 = 300000;
pub const DYNAMIC_FEE_PER_KB_BASE_FEE: u64 = 2000000000;// 2 * pow(10,9)

pub const DYNAMIC_FEE_PER_KB_BASE_BLOCK_REWARD: u64 = 10000000000000;// 10 * pow(10,12)

pub const DYNAMIC_FEE_PER_KB_BASE_FEE_V5: u64 = 2000000000 * CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V2 / CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V5;
pub const DYNAMIC_FEE_REFERENCE_TRANSACTION_WEIGHT: u64 = 3000;

pub const ORPHANED_BLOCKS_MAX_COUNT: u64 = 100;


pub const DIFFICULTY_TARGET_V2: u64 = 120;  // seconds

pub const DIFFICULTY_TARGET_V1: u64 = 60;  // seconds - before first fork

pub const DIFFICULTY_WINDOW: u64 = 720; // blocks

pub const DIFFICULTY_LAG: u64 = 15;  // !!!

pub const DIFFICULTY_CUT: u64 = 60;  // timestamps to cut after sorting

pub const DIFFICULTY_BLOCKS_COUNT: u64 = DIFFICULTY_WINDOW + DIFFICULTY_LAG;


pub const CRYPTONOTE_LOCKED_TX_ALLOWED_DELTA_SECONDS_V1: u64 = DIFFICULTY_TARGET_V1 * CRYPTONOTE_LOCKED_TX_ALLOWED_DELTA_BLOCKS;
pub const CRYPTONOTE_LOCKED_TX_ALLOWED_DELTA_SECONDS_V2: u64 = DIFFICULTY_TARGET_V2 * CRYPTONOTE_LOCKED_TX_ALLOWED_DELTA_BLOCKS;
pub const CRYPTONOTE_LOCKED_TX_ALLOWED_DELTA_BLOCKS: u64 = 1;


pub const DIFFICULTY_BLOCKS_ESTIMATE_TIMESPAN: u64 = DIFFICULTY_TARGET_V1; //just alias; used by tests


pub const BLOCKS_IDS_SYNCHRONIZING_DEFAULT_COUNT: u64 = 10000;  //by default, blocks ids count in synchronizing

pub const BLOCKS_SYNCHRONIZING_DEFAULT_COUNT_PRE_V4: u64 = 100; //by default, blocks count in blocks downloading

pub const BLOCKS_SYNCHRONIZING_DEFAULT_COUNT: u64 = 20;  //by default, blocks count in blocks downloading

pub const CRYPTONOTE_MEMPOOL_TX_LIVETIME: u64 = (86400 * 3); //seconds, three days

pub const CRYPTONOTE_MEMPOOL_TX_FROM_ALT_BLOCK_LIVETIME: u64 = 604800; //seconds, one week

pub const COMMAND_RPC_GET_BLOCKS_FAST_MAX_COUNT: u64 = 1000;

pub const P2P_LOCAL_WHITE_PEERLIST_LIMIT: u64 = 1000;
pub const P2P_LOCAL_GRAY_PEERLIST_LIMIT: u64 = 5000;

pub const P2P_DEFAULT_CONNECTIONS_COUNT: u64 = 8;
pub const P2P_DEFAULT_HANDSHAKE_INTERVAL: u64 = 60; //secondes

pub const P2P_DEFAULT_PACKET_MAX_SIZE: u64 = 50000000;  //50000000 bytes maximum packet size

pub const P2P_DEFAULT_PEERS_IN_HANDSHAKE: u64 = 250;
pub const P2P_DEFAULT_CONNECTION_TIMEOUT: u64 = 5000; //5 seconds

pub const P2P_DEFAULT_PING_CONNECTION_TIMEOUT: u64 = 2000; //2 seconds

pub const P2P_DEFAULT_INVOKE_TIMEOUT: u64 = 60 * 2 * 1000;  //2 minutes

pub const P2P_DEFAULT_HANDSHAKE_INVOKE_TIMEOUT: u64 = 5000; //5 seconds

pub const P2P_DEFAULT_WHITELIST_CONNECTIONS_PERCENT: u64 = 70;
pub const P2P_DEFAULT_ANCHOR_CONNECTIONS_COUNT: u64 = 2;
pub const P2P_DEFAULT_LIMIT_RATE_UP: u64 = 2048; // kB/s

pub const P2P_DEFAULT_LIMIT_RATE_DOWN: u64 = 8192; // kB/s

pub const P2P_FAILED_ADDR_FORGET_SECONDS: u64 = (60 * 60);  //1 hour

pub const P2P_IP_BLOCKTIME: u64 = (60 * 60 * 24);  //24 hour

pub const P2P_IP_FAILS_BEFORE_BLOCK: u64 = 10;
pub const P2P_IDLE_CONNECTION_KILL_INTERVAL: u64 = (5 * 60); //5 minutes

pub const P2P_SUPPORT_FLAG_FLUFFY_BLOCKS: u64 = 0x01;
pub const P2P_SUPPORT_FLAGS: u64 = P2P_SUPPORT_FLAG_FLUFFY_BLOCKS;

pub const CRYPTONOTE_NAME: &str = "bitmonero";
pub const CRYPTONOTE_POOLDATA_FILENAME: &str = "poolstate.bin";
pub const CRYPTONOTE_BLOCKCHAINDATA_FILENAME: &str = "data.mdb";
pub const CRYPTONOTE_BLOCKCHAINDATA_LOCK_FILENAME: &str = "lock.mdb";
pub const P2P_NET_DATA_FILENAME: &str = "p2pstate.bin";
pub const MINER_CONFIG_FILE_NAME: &str = "miner_conf.json";

pub const THREAD_STACK_SIZE: u64 = 5 * 1024 * 1024;

pub const HF_VERSION_DYNAMIC_FEE: u64 = 4;
pub const HF_VERSION_MIN_MIXIN_4: u64 = 6;
pub const HF_VERSION_MIN_MIXIN_6: u64 = 7;
pub const HF_VERSION_MIN_MIXIN_10: u64 = 8;
pub const HF_VERSION_ENFORCE_RCT: u64 = 6;
pub const HF_VERSION_PER_BYTE_FEE: u64 = 8;

pub const PER_KB_FEE_QUANTIZATION_DECIMALS: u64 = 8;

pub const HASH_OF_HASHES_STEP: u64 = 256;

pub const DEFAULT_TXPOOL_MAX_WEIGHT: u64 = 648000000; // 3 days at 300000, in bytes

pub const BULLETPROOF_MAX_OUTPUTS: u64 = 16;

// New constants are intended to go here
pub const DEFAULT_FEE_ATOMIC_XMR_PER_KB: u64 = 500; // Just a placeholder!  Change me!

pub const FEE_CALCULATION_MAX_RETRIES: u8 = 10;
pub const DEFAULT_DUST_THRESHOLD: u64 = 2000000000; // 2 * pow(10, 9)

pub const BASE_REWARD_CLAMP_THRESHOLD: u64 = 100000000;
pub const P2P_REMOTE_DEBUG_TRUSTED_PUB_KEY: &str = "0000000000000000000000000000000000000000000000000000000000000000";

pub enum NetworkType {
    MAINNET = 0,
    TESTNET,
    STAGENET,
    FAKECHAIN,
    UNDEFINED = 255,
}

pub struct Config {
    pub cryptonote_public_address_base58_prefix: u64,
    pub cryptonote_public_integrated_address_base58_prefix: u64,
    pub cryptonote_public_subaddress_base58_prefix: u64,
    pub p2p_default_port: u16,
    pub rpc_default_port: u16,
    pub zmq_rpc_default_port: u16,
    // Bender's nightmare
    pub network_id: [u8; 16],
    pub genesis_tx: &'static str,
    pub genesis_nonce: u32,
}

pub const MAINNET: Config = Config {
    cryptonote_public_address_base58_prefix: 18,
    cryptonote_public_integrated_address_base58_prefix: 19,
    cryptonote_public_subaddress_base58_prefix: 42,
    p2p_default_port: 18080,
    rpc_default_port: 18081,
    zmq_rpc_default_port: 18082,
    network_id: [0x12, 0x30, 0xF1, 0x71, 0x61, 0x04, 0x41, 0x61, 0x17, 0x31, 0x00, 0x82, 0x16, 0xA1, 0xA1, 0x10],
    genesis_tx: "013c01ff0001ffffffffffff03029b2e4c0281c0b02e7c53291a94d1d0cbff8883f8024f5142ee494ffbbd08807121017767aafcde9be00dcfd098715ebcf7f410daebc582fda69d24a28e9d0bc890d1",
    genesis_nonce: 10000,
};

pub const TESTNET: Config = Config {
    cryptonote_public_address_base58_prefix: 53,
    cryptonote_public_integrated_address_base58_prefix: 54,
    cryptonote_public_subaddress_base58_prefix: 63,
    p2p_default_port: 28080,
    rpc_default_port: 28081,
    zmq_rpc_default_port: 28082,
    network_id: [0x12, 0x30, 0xF1, 0x71, 0x61, 0x04, 0x41, 0x61, 0x17, 0x31, 0x00, 0x82, 0x16, 0xA1, 0xA1, 0x11],
    genesis_tx: "013c01ff0001ffffffffffff03029b2e4c0281c0b02e7c53291a94d1d0cbff8883f8024f5142ee494ffbbd08807121017767aafcde9be00dcfd098715ebcf7f410daebc582fda69d24a28e9d0bc890d1",
    genesis_nonce: 10001,
};

pub const STAGENET: Config = Config {
    cryptonote_public_address_base58_prefix: 24,
    cryptonote_public_integrated_address_base58_prefix: 25,
    cryptonote_public_subaddress_base58_prefix: 36,
    p2p_default_port: 38080,
    rpc_default_port: 38081,
    zmq_rpc_default_port: 38082,
    network_id: [0x12, 0x30, 0xF1, 0x71, 0x61, 0x04, 0x41, 0x61, 0x17, 0x31, 0x00, 0x82, 0x16, 0xA1, 0xA1, 0x12],
    genesis_tx: "013c01ff0001ffffffffffff0302df5d56da0c7d643ddd1ce61901c7bdc5fb1738bfe39fbe69c28a3a7032729c0f2101168d0c4ca86fb55a4cf6a36d31431be1c53a3bd7411bb24e8832410289fa6f3b",
    genesis_nonce: 10002,
};
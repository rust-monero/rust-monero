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
pub const MONEY_SUPPLY: u64 = -1;
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
pub const FEE_PER_KB_OLD: u64 = 10000000000; //pow(10, 10)
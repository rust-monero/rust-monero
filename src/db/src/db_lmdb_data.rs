use cryptonote_basic::difficulty::DifficultyType;

pub struct MdbBlockInfoOld {
    height: u64,
    timestamp: u64,
    coins: u64,
    weight: u64,
    diff: DifficultyType,
}
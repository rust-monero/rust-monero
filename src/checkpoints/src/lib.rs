use crypto::hash::Hash;
use std::collections::HashMap;

const JSON_HASH_FILE_NAME: &str = "checkpoints.json";

pub struct Checkpoints {
    pub points: HashMap<u64, Hash>,
}

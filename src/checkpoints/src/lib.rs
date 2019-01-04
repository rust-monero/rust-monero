use std::collections::HashMap;
use crypto::hash::Hash;

const JSON_HASH_FILE_NAME: &str = "checkpoints.json";

pub struct Checkpoints {
    pub points: HashMap<u64, Hash>
}

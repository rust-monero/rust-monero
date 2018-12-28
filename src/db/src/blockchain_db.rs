use crypto::crypto::PublicKey;

struct OutputData {
    pubkey: PublicKey,
    unlock_time: u64,
    height: u64,
//    commitment: rct:key;
}


pub trait BlockChainDB {
}
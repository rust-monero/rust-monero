use crypto::crypto::PublicKey;
use crypto::hash::Hash;

const TX_EXTRA_PADDING_MAX_COUNT: u32 = 255;
const TX_EXTRA_NONCE_MAX_COUNT: u32 = 255;

const TX_EXTRA_TAG_PADDING: u32 = 0x00;
const TX_EXTRA_TAG_PUBKEY: u32 = 0x01;
const TX_EXTRA_NONCE: u32 = 0x02;
const TX_EXTRA_MERGE_MINING_TAG: u32 = 0x03;
const TX_EXTRA_TAG_ADDITIONAL_PUBKEYS: u32 = 0x04;
const TX_EXTRA_MYSTERIOUS_MINERGATE_TAG: u32 = 0xDE;

const TX_EXTRA_NONCE_PAYMENT_ID: u32 = 0x00;
const TX_EXTRA_NONCE_ENCRYPTED_PAYMENT_ID: u32 = 0x01;

pub struct TxExtraPadding {
    size: u64,
}

pub struct TxExtraPubKey {
    pub_key: PublicKey
}

pub struct TxExtraNonce {
    nonce: String
}

pub struct TxExtraMergeMiningTag {
    depth: u64,
    merkle_root: Hash,
}

pub struct TxExtraAdditionalPubKeys {
    data: Vec<PublicKey>
}

pub struct TxExtraMysteriousMinergate {
    data: String
}

pub enum TxExtraField {
    TxExtraPadding(TxExtraPadding),
    TxExtraPubKey(TxExtraPubKey),
    TxExtraNonce(TxExtraNonce),
    TxExtraMergeMiningTag(TxExtraMergeMiningTag),
    TxExtraAdditionalPubKeys(TxExtraAdditionalPubKeys),
    TxExtraMysteriousMinergate(TxExtraMysteriousMinergate)
}
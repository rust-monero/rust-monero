use crate::block::AccountPublicAddress;
use xmr_crypto::crypto::SecretKey;
use xmr_crypto::chacha::ChaChaIV;
use xmr_device::Device;

pub struct AccountKeys {
    account_address: AccountPublicAddress,
    spend_secret_key: SecretKey,
    vew_secret_key: SecretKey,
    multisig_keys: Vec<SecretKey>,
    device: Device,
    m_encryption_iv: ChaChaIV
}

pub struct AccountBase {
    keys: AccountKeys,
    creation_timestamp: u64
}
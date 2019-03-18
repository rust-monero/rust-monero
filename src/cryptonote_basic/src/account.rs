use crate::block::AccountPublicAddress;
use crypto::chacha::ChaChaIV;
use crypto::crypto::SecretKey;
use device::Device;

pub struct AccountKeys {
    account_address: AccountPublicAddress,
    spend_secret_key: SecretKey,
    vew_secret_key: SecretKey,
    multisig_keys: Vec<SecretKey>,
    device: Device,
    m_encryption_iv: ChaChaIV,
}

pub struct AccountBase {
    keys: AccountKeys,
    creation_timestamp: u64,
}

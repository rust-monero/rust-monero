use crate::block::AccountPublicAddress;
use xmr_crypto::crypto::SecretKey;

pub struct AccountKeys {
    account_address: AccountPublicAddress,
    spend_secret_key: SecretKey,
    vew_secret_key: SecretKey,
    multisig_keys: Vec<SecretKey>,
    //TODO hw::device
    //hw::device *m_device = &hw::get_device("default");

}
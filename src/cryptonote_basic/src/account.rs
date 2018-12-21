use crate::block::AccountPublicAddress;
use xmr_crypto::crypto::SecretKey;

pub struct AccountKeys {
    accountAddress: AccountPublicAddress,
    spendSecretKey: SecretKey,
    vewSecretKey: SecretKey,
    multisig_keys: Vec<SecretKey>,
    //TODO hw::device
    //hw::device *m_device = &hw::get_device("default");

}
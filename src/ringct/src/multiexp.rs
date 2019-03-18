use crate::rct_types::Key;
use crypto::crypto_ops::ge_p3;

pub struct MultiexpData {
    scalar: Key,
    point: ge_p3,
}

//ringct/multiexp.h 56
struct StrausCachedData;
struct PippengerCachedData;

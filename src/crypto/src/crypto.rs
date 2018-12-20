pub struct EcPoint(pub [u8; 32]);

pub struct EcScalar(pub [u8; 32]);

pub struct PublicKey {
    ec_point: EcPoint
}
//TODO
pub struct SecretKey {

}

pub struct PublicKeyV {
    keys: Vec<PublicKey>,
    rows: u32
}

pub struct SecretKeyV {
    keys: Vec<SecretKey>,
    rows: u32
}

pub struct PublicKeyM {
    cols: u32,
    rows: u32,
    column_vectors: Vec<SecretKeyV>
}

pub struct KeyDerivation {
    ec_point: EcPoint
}
pub struct KeyImage {
    ec_point: EcPoint
}
pub struct Signature {
    c: EcScalar,
    r: EcScalar,
}
//TODO
//POD_CLASS key_derivation: ec_point {
//friend class crypto_ops;
//};
//
//POD_CLASS key_image: ec_point {
//friend class crypto_ops;
//};
//
//POD_CLASS signature {
//ec_scalar c, r;
//friend class crypto_ops;
//};


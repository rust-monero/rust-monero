use crypto::crypto_ops::ge_dsmp;

const ATOMS: usize = 64;

pub struct Key {
    bytes: [char; 32]
}

impl Key {
    fn new(data: [u8; 32]) -> Key {
        let mut tmp: [char; 32] = [0 as char; 32];
        for (i, v) in data.iter().enumerate() {
            tmp[i] = *v as char;
        }
        let k = Key {
            bytes: tmp
        };
        k
    }
}

//vector of keys
pub struct KeyV(Vec<Key>);

//matrix of keys (indexed by column first)
pub struct KeyM(Vec<KeyV>);

pub struct CtKey {
    pub dest: Key,
    //C here if public
    pub mask: Key,
}

pub struct CtKeyV(Vec<CtKey>);

pub struct CtKeyM(Vec<CtKeyV>);

#[allow(non_snake_case)]
pub struct MultisigKLRki {
    K: Key,
    L: Key,
    R: Key,
    ki: Key,
}

pub struct MultisigOut {
    c: Vec<Key>
}

pub struct EcdhTuple {
    mask: Key,
    amount: Key,
    sender_pk: Key,
}

pub struct XmrAmount(u64);

pub struct Bits([u32; ATOMS]);

pub struct Key64([Key; 64]);

pub struct BoroSig {
    s0: Key64,
    s1: Key64,
    ee: Key,
}

pub struct GeDsmp {
    //crypto-ops.h
    k: ge_dsmp
}

#[allow(non_snake_case)]
pub struct MgSig {
    ss: KeyM,
    cc: Key,
    II: KeyV,
}

#[allow(non_snake_case)]
pub struct RangeSig {
    asig: BoroSig,
    Ci: Key64,
}

#[allow(non_snake_case)]
pub struct Bulletproof {
    V: KeyV,
    A: Key,
    S: Key,
    T1: Key,
    T2: Key,
    taux: Key,
    mu: Key,
    L: KeyV,
    R: KeyV,
    a: Key,
    b: Key,
    t: Key,
}

pub enum RCTType {
    RCTTypeNull,
    RCTTypeFull,
    RCTTypeSimple,
    RCTTypeBulletproof,
}

pub enum RangeProofType {
    RangeProofBorromean,
    RangeProofBulletproof,
    RangeProofMultiOutputBulletproof,
    RangeProofPaddedBulletproof,
}

pub struct RCTSigBase {
    _type: u8,
    message: Key,
    //the set of all pubkeys / copy
    mix_ring: CtKeyM,
    //pairs that you mix with
    //C - for simple rct
    pseudo_outs: KeyV,
    ecdh_info: Vec<EcdhTuple>,
    out_pk: CtKeyV,
    txn_fee: XmrAmount,
}

#[allow(non_snake_case)]
pub struct RCTSigPrunable {
    rangeSigs: Vec<RangeSig>,
    bulletproofs: Vec<Bulletproof>,
    MGs: Vec<MgSig>,
    pseudo_outs: KeyV,
}


pub struct RCTSig {
    p: RCTSigPrunable
}

pub const H: Key = Key::new([0x8b, 0x65, 0x59, 0x70, 0x15, 0x37, 0x99, 0xaf, 0x2a, 0xea, 0xdc, 0x9f, 0xf1, 0xad, 0xd0, 0xea, 0x6c, 0x72, 0x51, 0xd5, 0x41, 0x54, 0xcf, 0xa9, 0x2c, 0x17, 0x3a, 0x0d, 0xd3, 0x9c, 0x1f, 0x94]);

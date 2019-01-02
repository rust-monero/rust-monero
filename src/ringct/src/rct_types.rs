use crypto::crypto_ops::ge_dsmp;

const ATOMS: usize = 64;

pub struct Key {
    bytes: [char; 32]
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

pub const H: Key = Key {
    bytes: [0x8b as char, 0x65 as char, 0x59 as char, 0x70 as char, 0x15 as char, 0x37 as char, 0x99 as char, 0xaf as char,
        0x2a as char, 0xea as char, 0xdc as char, 0x9f as char, 0xf1 as char, 0xad as char, 0xd0 as char, 0xea as char,
        0x6c as char, 0x72 as char, 0x51 as char, 0xd5 as char, 0x41 as char, 0x54 as char, 0xcf as char, 0xa9 as char,
        0x2c as char, 0x17 as char, 0x3a as char, 0x0d as char, 0xd3 as char, 0x9c as char, 0x1f as char, 0x94 as char]
};

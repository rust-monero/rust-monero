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

pub struct MultisigKLRki {
    K: Key,
    L: Key,
    R: Key,
    ki: Key
}

pub struct MultisigOut {
    c: Vec<Key>
}

pub struct EcdhTuple {
    mask: Key,
    amount: Key,
    senderPk: Key
}

pub struct XmrAmount(u64);

pub struct bits([u32; ATOMS]);

pub struct Key64([Key; 64]);

pub struct BoroSig {
    s0: Key64,
    s1: Key64,
    ee: Key
}

pub struct GeDsmp {
    //crypto-ops.h
     k: ge_dsmp
}

pub struct MgSig {
    ss: KeyM,
    cc: Key,
    II: keyV
}

pub struct RangeSig {
    asig: BoroSig,
    Ci: Key64,
}

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


const ATOMS: usize = 64;

struct Key {
    bytes: [char; 32]
}

//vector of keys
struct KeyV(Vec<Key>);
//matrix of keys (indexed by column first)
struct KeyM(Vec<KeyV>);

struct CtKey {
    dest: Key,
    //C here if public
    mask: Key,
}

struct CtKeyV(Vec<CtKey>);
struct CtKeyM(Vec<CtKeyV>);

struct MultisigKLRki {
    K: Key,
    L: Key,
    R: Key,
    ki: Key
}

struct MultisigOut {
    c: Vec<Key>
}

struct EcdhTuple {
    mask: Key,
    amount: Key,
    senderPk: Key
}

struct XmrAmount(u64);

struct bits([u32; ATOMS]);

struct Key64([Key; 64]);

struct BoroSig {
    s0: Key64,
    s1: Key64,
    ee: Key
}

struct GeDsmp {
    //crypto-ops.h
    //ge_dsmp k;
}


pub struct TxVerificationContext {
    should_be_relayed: bool,
    //bad tx, should drop connection
    verifivation_failed: bool,
    //the transaction is related with an alternative blockchain
    verifivation_impossible: bool,
    added_to_pool: bool,
    low_mixin: bool,
    double_spend: bool,
    invalid_input: bool,
    invalid_output: bool,
    too_big: bool,
    overspend: bool,
    fee_too_low: bool,
    not_rct: bool,
}

pub struct BlockVerificationContext {
    added_to_main_chain: bool,
    //bad block, should drop connection
    verifivation_failed: bool,
    marked_as_orphaned: bool,
    already_exists: bool,
    partial_block_reward: bool,
}

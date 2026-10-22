use std::{any::Any, sync::Arc, time::Duration};

use bitcoin::{
    Amount, BlockHash, CompactTarget, ScriptBuf, Sequence, Transaction, TxMerkleNode, TxOut,
    absolute, block, transaction,
};

use crate::Fut;

pub struct TemplateToken(Arc<dyn Any + Send + Sync>);

impl TemplateToken {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }
}

pub struct Coinbase {
    pub version: transaction::Version,
    pub sequence: Sequence,
    pub script_sig_prefix: ScriptBuf,
    pub witness: Option<[u8; 32]>,
    pub reward_remaining: Amount,
    pub required_outputs: Vec<TxOut>,
    pub lock_time: absolute::LockTime,
}

pub struct Template {
    pub prev_hash: BlockHash,
    pub height: u64,
    pub version: block::Version,
    pub bits: CompactTarget,
    pub min_time: u32,
    pub cur_time: u32,
    pub coinbase: Coinbase,
    pub coinbase_merkle_path: Vec<TxMerkleNode>,
    pub token: TemplateToken,
}

pub struct TemplateTx {
    pub tx: Transaction,
    pub fee: Amount,
    pub sigops: u32,
}

pub struct Solution {
    pub version: block::Version,
    pub time: u32,
    pub nonce: u32,
    pub coinbase: Transaction,
}

pub struct WaitOptions {
    pub timeout: Option<Duration>,
    pub fee_threshold: Option<Amount>,
}

pub trait TemplateSource {
    fn template(&self) -> impl Fut<Template>;
    fn transactions(&self, tmpl: &Template) -> impl Fut<Vec<TemplateTx>>;
    fn wait_next(&self, prev: &Template, opts: WaitOptions) -> impl Fut<Option<Template>>;
    fn submit_solution(&self, tmpl: &Template, sol: &Solution) -> impl Fut<()>;
}

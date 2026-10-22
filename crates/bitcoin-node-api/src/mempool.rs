use crate::Fut;
use bitcoin::{Amount, Txid, Weight};

pub struct MempoolEntry {
    pub vsize: u32,
    pub weight: Weight,
    pub fee: Amount,
    pub time: u64,
    pub height: u64,
    pub ancestors: u32,
    pub descendants: u32,
}

pub trait MempoolSource {
    fn txids(&self) -> impl Fut<Vec<Txid>>;
    fn entry(&self, txid: Txid) -> impl Fut<Option<MempoolEntry>>;
}

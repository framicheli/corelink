use bitcoin::{Amount, FeeRate, Transaction, Txid};

use crate::Fut;

pub struct MempoolAccept {
    pub txid: Txid,
    pub allowed: bool,
    pub reject_reason: Option<String>,
}

pub struct PackageResult {
    pub txid: Txid,
    pub outcome: PackageOutcome,
}

pub enum PackageOutcome {
    Accepted { fee: Amount },
    AlreadyInMempool,
    Rejected(String),
}

pub enum FeeMode {
    Conservative,
    Economical,
}

pub struct FeeTarget {
    pub blocks: u16,
    pub mode: FeeMode,
}

pub trait Broadcaster {
    fn broadcast(&self, tx: &Transaction) -> impl Fut<Txid>;
    fn broadcast_package(&self, txs: &[Transaction]) -> impl Fut<Vec<PackageResult>>; // Only from Bitcoin Core >= 28
    fn test_accept(&self, txs: &[Transaction]) -> impl Fut<Vec<MempoolAccept>>;
}

pub trait FeeEstimator {
    // `None` when node has no estimate yet, like a fresh node or regtest
    fn estimate_fee(&self, target: FeeTarget) -> impl Fut<Option<FeeRate>>;
}

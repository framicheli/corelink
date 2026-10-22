use crate::Fut;
use bitcoin::{Amount, Block, BlockHash, OutPoint, ScriptBuf, Transaction, Txid, block::Header};

pub struct Tip {
    pub height: u64,
    pub hash: BlockHash,
}

pub struct Utxo {
    pub value: Amount,
    pub script_pubkey: ScriptBuf,
    pub confirmations: u32,
    pub coinbase: bool,
}

pub trait ChainSource {
    fn tip(&self) -> impl Fut<Tip>;
    fn block_hash(&self, height: u64) -> impl Fut<BlockHash>;
    fn header(&self, hash: BlockHash) -> impl Fut<Header>;
    fn block(&self, hash: BlockHash) -> impl Fut<Block>;
    fn transaction(&self, txid: Txid, block: Option<BlockHash>) -> impl Fut<Transaction>;
    fn utxo(&self, outpoint: OutPoint, include_mempool: bool) -> impl Fut<Option<Utxo>>;
}

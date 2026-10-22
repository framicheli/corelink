use bitcoin::{BlockHash, Txid};
use futures_core::Stream;

use crate::{ApiResult, Fut};

pub enum NodeEvent {
    BlockConnected(BlockHash),
    BlockDisconnected(BlockHash),
    TxAdded(Txid),
    TxRemoved(Txid),
    Lagged,
}

pub trait EventSource {
    type EventStream: Stream<Item = ApiResult<NodeEvent>> + Send + Unpin;

    fn subscribe(&self) -> impl Fut<Self::EventStream>;
}

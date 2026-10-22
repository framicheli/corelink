use std::future::Future;

pub mod chain;
pub mod error;
pub mod event;
pub mod mempool;
pub mod mining;
pub mod tx;

pub type ApiResult<T> = core::result::Result<T, crate::error::NodeAPIError>;

pub trait Fut<T>: Future<Output = ApiResult<T>> + Send {}

impl<T, F> Fut<T> for F where F: Future<Output = ApiResult<T>> + Send {}

pub mod erc20;

use thiserror::Error;
use web3::types::{Address, Bytes, H256, TransactionParameters};
use crate::engine::Engine;
use async_trait::async_trait;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug,Error)]
pub enum Error {
    #[error("address from_str error")]
    ErrAddress(#[from] hex::FromHexError),
    #[error("engine error")]
    ErrEngine(#[from] crate::engine::Error),
    #[error("abi error")]
    ErrAbi(#[from] crate::abi::Error),
}

#[async_trait]
pub trait ContractLiving<T> {
    fn engine(&self) -> &Engine;
    fn contract(&self) -> &Address;
    fn abi(&self) -> &T;

    async fn call_data(&self, data: Bytes) -> Result<Bytes> {
        Ok(self.engine().call_transaction(self.contract().clone(), data).await?)
    }

    async fn send_data(&self, data: Bytes, private_key: &str) -> Result<(H256, TransactionParameters)> {
        Ok(self.engine().send_transaction_by_data(self.contract().clone(), data, private_key).await?)
    }
}


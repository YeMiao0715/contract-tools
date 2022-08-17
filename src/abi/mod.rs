use web3::ethabi::Token;
use web3::types::{Bytes};
use thiserror::Error;

pub mod erc20;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug,Error)]
pub enum Error {
    #[error("eth abi error")]
    ErrEthAbi(#[from] web3::ethabi::Error),
    #[error("address from_str error")]
    ErrAddress(#[from] hex::FromHexError),
    #[error("contract error")]
    ErrWeb3Contract(#[from] web3::contract::Error),
}

pub trait ContractAbi {
    fn abi(&self) -> &web3::ethabi::Contract;

    fn method(&self, name: &str, tokens: &[Token]) -> Result<Bytes> {
        let f = self.abi().function(name)?;
        let b = f.encode_input(tokens)?;
        Ok(Bytes(b))
    }

    fn decode_method(&self, name: &str, data: Bytes) -> Result<Vec<Token>> {
        let f = self.abi().function(name)?;
        let res = f.decode_output(&data.0)?;
        Ok(res)
    }
}
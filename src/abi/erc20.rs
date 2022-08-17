use web3::contract::tokens::{Detokenize, Tokenize};
use crate::abi::{ContractAbi, Result};
use web3::ethabi::{Token};
use web3::types::{Address, Bytes, U256};

const ERC20_ABI_JSON: &str = "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"}],\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\"}],\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\"}],\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"msgSender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]";

pub trait Erc20ContractAbi: ContractAbi {
    fn name(&self) -> Result<Bytes> {
        self.method("name", &().into_tokens())
    }

    fn decode_name(&self, data: Bytes) -> Result<String> {
        let tokens = self.decode_method("name", data)?;
        Ok(String::from_tokens(tokens)?)
    }

    fn symbol(&self) -> Result<Bytes> {
        self.method("symbol", &().into_tokens())
    }

    fn decode_symbol(&self, data: Bytes) ->Result<String> {
        let tokens = self.decode_method("symbol", data)?;
        Ok(String::from_tokens(tokens)?)
    }

    fn decimals(&self) -> Result<Bytes> {
        self.method("decimals", &().into_tokens())
    }

    fn decode_decimals(&self, data: Bytes) ->Result<U256> {
        let tokens = self.decode_method("decimals", data)?;
        Ok(U256::from_tokens(tokens)?)
    }

    fn total_supply(&self) -> Result<Bytes> {
        self.method("totalSupply", &().into_tokens())
    }

    fn decode_total_supply(&self, data: Bytes) -> Result<U256> {
        let tokens = self.decode_method("totalSupply", data)?;
        Ok(U256::from_tokens(tokens)?)
    }

    fn balance_of(&self, address: Address) -> Result<Bytes> {
        self.method("balanceOf", &address.into_tokens())
    }

    fn decode_balance_of(&self, data: Bytes) -> Result<U256> {
        let tokens = self.decode_method("balanceOf", data)?;
        Ok(U256::from_tokens(tokens)?)
    }

    fn transfer(&self, to: Address, amount: U256) -> Result<Bytes> {
        self.method("transfer", &(to, amount).into_tokens())
    }

    fn allowance(&self, owner: Address, spender: Address) -> Result<Bytes> {
        self.method("allowance", &(owner, spender).into_tokens())
    }

    fn decode_allowance(&self, data: Bytes) -> Result<U256> {
        let tokens = self.decode_method("allowance", data)?;
        Ok(U256::from_tokens(tokens)?)
    }

    fn approve(&self, spender: Address, amount: U256) -> Result<Bytes> {
        self.method("allowance", &(spender, amount).into_tokens())
    }

    fn transfer_from(&self, from: Address, to: Address, amount: U256) -> Result<Bytes> {
        self.method("transferFrom", &(from, to, amount).into_tokens())
    }

    fn increase_allowance(&self, spender: Address, added_value: U256) -> Result<Bytes> {
        self.method("increaseAllowance", &(spender, added_value).into_tokens())
    }

    fn decrease_allowance(&self, spender: Address, subtracted_value: U256) -> Result<Bytes> {
        self.method("decreaseAllowance", &(spender, subtracted_value).into_tokens())
    }
}


pub struct Erc20Abi {
    abi: web3::ethabi::Contract
}

impl Erc20Abi {
    pub fn new() -> Self {
        Self {
            abi: web3::ethabi::Contract::load(ERC20_ABI_JSON.as_bytes()).expect("ERC20_ABI_JSON is error")
        }
    }
}

impl ContractAbi for Erc20Abi {
    fn abi(&self) -> &web3::ethabi::Contract {
        &self.abi
    }
}

impl Erc20ContractAbi for Erc20Abi {}

#[cfg(test)]
mod tests{
    use web3::ethabi::Address;
    use crate::abi::erc20::{Erc20Abi, Erc20ContractAbi};

    #[test]
    fn erc20_abi() {
        let erc20_abi = Erc20Abi::new();
        let data = erc20_abi.name().expect("balance_of err");
        println!("{:?}", hex::encode(&data.0));
        let data = erc20_abi.symbol().expect("symbol err");
        println!("{:?}", hex::encode(&data.0));
        let data = erc20_abi.decimals().expect("decimals err");
        println!("{:?}", hex::encode(&data.0));
        let data = erc20_abi.total_supply().expect("total_supply err");
        println!("{:?}", hex::encode(&data.0));
        let data = erc20_abi.balance_of(Address::random()).expect("balance_of err");
        println!("{:?}", hex::encode(&data.0));
    }

}
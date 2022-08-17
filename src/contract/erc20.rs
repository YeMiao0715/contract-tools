use web3::types::{Address, Bytes, H256, TransactionParameters, U256};
use crate::abi::erc20::{Erc20Abi, Erc20ContractAbi};
use crate::contract::ContractLiving;
use crate::engine::Engine;
use async_trait::async_trait;
use crate::contract::Result;

#[async_trait]
pub trait Erc20Contract<T: Erc20ContractAbi>: ContractLiving<T> {
    async fn name(&self) -> Result<String> {
        let data = self.abi().name()?;
        let data = self.call_data(data).await?;
        Ok(self.abi().decode_name(data)?)
    }

    async fn symbol(&self) -> Result<String> {
        let data = self.abi().symbol()?;
        let data = self.call_data(data).await?;
        Ok(self.abi().decode_name(data)?)
    }

    async fn decimals(&self) -> Result<U256> {
        let data = self.abi().decimals()?;
        let data = self.call_data(data).await?;
        Ok(self.abi().decode_decimals(data)?)
    }

    async fn total_supply(&self) -> Result<U256> {
        let data = self.abi().total_supply()?;
        let data = self.call_data(data).await?;
        Ok(self.abi().decode_total_supply(data)?)
    }

    async fn balance_of(&self, account: Address) -> Result<U256> {
        let data = self.abi().balance_of(account)?;
        let data = self.call_data(data).await?;
        Ok(self.abi().decode_balance_of(data)?)
    }

    async fn transfer(&self, to: Address, amount: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        let data = self.abi().transfer(to, amount)?;
        Ok(self.send_data(data, private_key).await?)
    }

    async fn allowance(&self, owner: Address, spender: Address) -> Result<U256> {
        let data = self.abi().allowance(owner,spender)?;
        let data = self.call_data(data).await?;
        Ok(self.abi().decode_allowance(data)?)
    }

    async fn approve(&self, spender: Address, amount: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        let data = self.abi().approve(spender, amount)?;
        Ok(self.send_data(data, private_key).await?)
    }

    async fn transfer_from(&self, from: Address, to: Address, amount: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        let data = self.abi().transfer_from(from, to, amount)?;
        Ok(self.send_data(data, private_key).await?)
    }

    async fn increase_allowance(&self, spender: Address, added_value: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        let data = self.abi().increase_allowance(spender, added_value)?;
        Ok(self.send_data(data, private_key).await?)
    }

    async fn decrease_allowance(&self, spender: Address, subtracted_value: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        let data = self.abi().decrease_allowance(spender, subtracted_value)?;
        Ok(self.send_data(data, private_key).await?)
    }
}

pub struct Erc20 {
    engine: Engine,
    contract: Address,
    abi: Erc20Abi,
}

impl Erc20 {
    pub fn new(engine: Engine, contract: Address) -> Self {
        Self {engine, contract, abi: Erc20Abi::new()}
    }
}

impl ContractLiving<Erc20Abi> for Erc20 {
    fn engine(&self) -> &Engine {
        &self.engine
    }

    fn contract(&self) -> &Address {
        &self.contract
    }

    fn abi(&self) -> &Erc20Abi {
        &self.abi
    }
}

impl Erc20Contract<Erc20Abi> for Erc20 {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use web3::types::{Address, U256};
    use crate::contract::ContractLiving;
    use crate::contract::erc20::{Erc20, Erc20Contract};
    use crate::engine::Engine;

    const PRIVATE_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    #[tokio::test]
    async fn erc20() {
        let engine = Engine::new("http:/127.0.0.1:8545").unwrap();
        let contract = Address::from_str("0x2F5c897956c9a512FFDEC3846fCB8C9e7b7989A3").unwrap();
        let erc20 = Erc20::new(engine,contract);
        let name = erc20.name().await.unwrap();
        println!("name: {}", name);
        let symbol = erc20.symbol().await.unwrap();
        println!("symbol: {}", symbol);
        let total_supply = erc20.total_supply().await.unwrap();
        println!("total_supply: {}", total_supply);
        let to = Address::random();
        let balance_of = erc20.balance_of(to).await.unwrap();
        println!("balance_of: {}", balance_of);
        let (hash, _) = erc20.transfer(to,U256::exp10(20), PRIVATE_KEY).await.unwrap();
        println!("hash: {}", hash);
        let receipt = erc20.engine().wait_transaction(hash).await.unwrap();
        println!("hash status: {}", receipt.status.unwrap());
        let balance_of = erc20.balance_of(to).await.unwrap();
        println!("balance_of: {}", balance_of);
        assert_eq!(balance_of, U256::exp10(20))
    }

}
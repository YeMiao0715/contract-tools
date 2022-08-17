use std::ops::{Mul};
use std::str::FromStr;
use std::time::Duration;
use secp256k1::SecretKey;
use web3::signing::{Key, SecretKeyRef};
use web3::transports::{Http};
use web3::types::{Address, H256, U256, U64, Bytes, CallRequest, TransactionParameters, TransactionReceipt};
use thiserror::Error;
use web3::Web3;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("web3 error")]
    ErrWeb3(#[from] web3::Error),
    #[error("secp256k1 error")]
    ErrSecp256k1(#[from] secp256k1::Error),
    #[error("from hex error")]
    ErrHexDecode(#[from] hex::FromHexError),
}


#[derive(Debug, Clone)]
pub struct Engine {
    web3: Web3<Http>,
    transaction_type: Option<U64>,
    gas_price: Option<U256>
}

impl Engine {

    pub fn new(rpc: &'static str) -> Result<Self> {
        let client = Http::new(rpc)?;
        let w3 = Web3::new(client);
        Ok(Self{
            web3: w3,
            transaction_type: Some(U64::from(2)),
            gas_price: Some(U256::from(5).mul(U256::exp10(9)))
        })
    }

    pub fn from_transaction_type(rpc: &'static str, t: i32) -> Result<Self> {
        let client = Http::new(rpc)?;
        let w3 = Web3::new(client);
        Ok(Self{
            web3: w3,
            transaction_type: Some(U64::from(t)),
            gas_price: Some(U256::from(5).mul(U256::exp10(9)))
        })
    }

    pub fn web3(&self) -> &Web3<Http> {
        &self.web3
    }

    pub fn parse_private_key_to_address(&self, private_key: &str) -> Result<(Address, SecretKey)> {
        let private_key = if private_key.contains("0x") {
            &private_key[2..]
        }else {
            private_key
        };
        let private_key = SecretKey::from_str(private_key)?;
        let from = SecretKeyRef::from(&private_key).address();
        Ok((from, private_key))
    }

    async fn make_transaction(&self, from: Address, to: Address, value: Option<U256>, data: Option<Bytes>, nonce: Option<U256>) -> Result<TransactionParameters> {
        let transaction_type = self.transaction_type.clone();

        let gas_price = self.gas_price.clone();

        let gas = self.web3.eth().estimate_gas(CallRequest{
            from: Some(from),
            to: Some(to),
            gas: None,
            gas_price,
            value,
            data: data.clone(),
            transaction_type,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None
        }, None).await?;

        let chain_id = self.web3.eth().chain_id().await?;

        Ok(TransactionParameters{
            to: Some(to),
            gas,
            gas_price,
            value: match value {
                Some(t) => t,
                None => U256::zero(),
            },
            data: match data {
                Some(t) => t,
                None => Bytes::default(),
            },
            nonce: match nonce {
                Some(t) => Some(t),
                None => {
                    let transaction_count = self.web3.eth().transaction_count(from, None).await?;
                    Some(transaction_count)
                },
            },
            transaction_type,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            chain_id: Some(chain_id.as_u64()),
        })
    }

    async fn send_transaction(&self, to: Address, value: Option<U256>, data: Option<Bytes>, nonce: Option<U256>, private_key: &str) -> Result<(H256, TransactionParameters)> {
        let (from, private_key) = self.parse_private_key_to_address(private_key)?;
        let tx = self.make_transaction(from, to, value, data, nonce).await?;
        let signed = self.web3.accounts().sign_transaction(tx.clone(), &private_key).await?;
        let result = self.web3.eth().send_raw_transaction(signed.raw_transaction).await?;
        Ok((result, tx))
    }

    pub async fn send_transaction_by_value(&self, to: Address, value: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        self.send_transaction(to, Some(value), None, None, private_key).await
    }

    pub async fn send_transaction_by_value_with_nonce(&self, to: Address, value: U256, nonce: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        self.send_transaction(to, Some(value), None, Some(nonce), private_key).await
    }

    pub async fn send_transaction_by_data(&self, to: Address, data: Bytes, private_key: &str) -> Result<(H256, TransactionParameters)> {
        self.send_transaction(to, None, Some(data), None, private_key).await
    }

    pub async fn send_transaction_by_data_with_nonce(&self, to: Address, data: Bytes, nonce: U256, private_key: &str) -> Result<(H256, TransactionParameters)> {
        self.send_transaction(to, None, Some(data), Some(nonce), private_key).await
    }

    pub async fn call_transaction(&self, contract: Address, data: Bytes) -> Result<Bytes>{
        let data = self.web3.eth().call(CallRequest{
            from: None,
            to: Some(contract),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
            transaction_type: self.transaction_type.clone(),
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None
        }, None).await?;
        Ok(data)
    }

    pub async fn wait_transaction(&self, hash: H256) -> Result<TransactionReceipt> {
        loop {
            match self.web3().eth().transaction_receipt(hash).await? {
                Some(receipt) => {
                    return Ok(receipt);
                }
                None => {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Div;
    use std::str::FromStr;
    use web3::types::{Address, TransactionParameters, U256};
    use secp256k1::{SecretKey};
    use web3::signing::{Key, SecretKeyRef};
    use crate::engine::Engine;

    const PRIVATE_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    #[tokio::test]
    async fn test_new_engine() {
        let private_key = SecretKey::from_str(PRIVATE_KEY).expect("解析私钥失败");
        let from = SecretKeyRef::from(&private_key).address();
        println!("{}", from)
    }

    #[tokio::test]
    async fn address_of_extended_private_key() -> web3::Result {
        // let prvk = hex::decode("43e82a4e3480804654f0bedff00a6721f1720d4f4489fc1c76771e79e2ceef8e").expect("解析私钥失败");
        let prvk = SecretKey::from_str(PRIVATE_KEY).expect("私钥解析失败");
        // println!("{:?}", prvk.address());
        // secret_key.public_key()
        // let secp = Secp256k1::new();
        // println!("{}", secret_key.public_key(&secp).to_string());
        // let key = SecretKeyRef::from(&secret_key);

        // let private_key = SigningKey::from_bytes(prvk.as_slice()).expect("解析私钥失败");
        //

        let to = Address::from_str("0x7EB8f3364B5F2BDe169b198d2DB41903f575522a").unwrap();

        let client = web3::transports::Http::new("http://127.0.0.1:8545").expect("rpc client error");
        let w3 = web3::Web3::new(client);

        let tx_object = TransactionParameters {
            to: Some(to),
            value: U256::exp10(15), //0.1 eth
            ..Default::default()
        };

        let signed = w3.accounts().sign_transaction(tx_object, &prvk).await?;
        println!("{}", signed.message_hash);
        // SecretKeyRef::new("asd");

        // Send the tx to infura
        let result = w3.eth().send_raw_transaction(signed.raw_transaction).await?;

        println!("Tx succeeded with hash: {}", result);

        // private_key.public_key();
        // let e = private_key.public_key().to_encoded_point(false);
        // let from = Address::from
        // println!("{}", from.to_string())
        Ok(())
    }

    #[tokio::test]
    async fn send_transaction_by_value() -> () {
        let e = Engine::new("http://127.0.0.1:8545").unwrap();
        let to = Address::from_str("0x7EB8f3364B5F2BDe169b198d2DB41903f575522a").unwrap();
        let hash = e.send_transaction_by_value(
            to,
            U256::exp10(15),
            PRIVATE_KEY
        ).await.unwrap();

        println!("hash is {}",hash.0)
    }

    #[tokio::test]
    async fn send_transaction_by_value_with_nonce() -> () {
        let e = Engine::new("http://127.0.0.1:8545").unwrap();
        let to = Address::from_str("0x7EB8f3364B5F2BDe169b198d2DB41903f575522a").unwrap();

        let private_key = PRIVATE_KEY;
        let count = {
            let (from, _) = e.parse_private_key_to_address(private_key).unwrap();
            println!("{:?}", from);
            let count = e.web3.eth().transaction_count(from,None).await.unwrap();
            println!("count {}", count);
            count
        };


        let hash = e.send_transaction_by_value_with_nonce(
            to,
            U256::exp10(15),
            count,
            private_key
        ).await.unwrap();

        println!("hash is {:?}",hash)
    }

    #[tokio::test]
    async fn get_account_balance() {
        let e = Engine::new("http://127.0.0.1:8545").unwrap();
        let address = Address::from_str("0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199").unwrap();
        let balance = e.web3().eth().balance(address, None).await.unwrap();
        println!("{}",balance.div(U256::exp10(18)));

    }
}
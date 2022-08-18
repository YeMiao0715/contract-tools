use web3::types::{AccessList, Address, Bytes, H256, SignedTransaction, TransactionParameters, U256, U64};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tx {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Bytes,
    pub gas: U256,
    pub gas_price: Option<U256>,
    pub nonce: Option<U256>,
    pub chain_id: Option<u64>,
    pub transaction_type: Option<U64>,
    pub access_list: Option<AccessList>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,

    pub hash: Option<H256>,
    pub message_hash: Option<H256>,
    pub v: Option<u64>,
    pub r: Option<H256>,
    pub s: Option<H256>,
}

impl Tx {

    pub fn from_transaction_parameters(tx: TransactionParameters) -> Self {
        Self {
            from: None,
            nonce: tx.nonce,
            to: tx.to,
            gas: tx.gas,
            gas_price: tx.gas_price,
            value: tx.value,
            data: tx.data,
            chain_id: tx.chain_id,
            transaction_type: tx.transaction_type,
            access_list: tx.access_list,
            max_fee_per_gas: tx.max_fee_per_gas,
            max_priority_fee_per_gas: tx.max_priority_fee_per_gas,

            hash: None,
            message_hash: None,
            v: None,
            r: None,
            s: None
        }
    }

    pub fn set_from(&mut self, from: Address) {
        self.from = Some(from)
    }

    pub fn set_signed(&mut self, signed: SignedTransaction) {
        self.hash = Some(signed.transaction_hash);
        self.message_hash = Some(signed.message_hash);
        self.s = Some(signed.s);
        self.v = Some(signed.v);
        self.r = Some(signed.r);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
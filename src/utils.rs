use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::json;
use web3::signing::{keccak256};
use web3::types::{Address, H256, TransactionParameters};

pub trait AddressUtils {
    // 将地址转换为checksum_address
    fn checksum_address(&self) -> String;
}

impl AddressUtils for Address {
    fn checksum_address(&self) -> String {
        // 0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199
        let lowercase_address = hex::encode(self);
        let hash_address = hex::encode(keccak256(lowercase_address.as_bytes()));
        let mut checksum_address = "0x".to_string();
        for c in 0..40 {
            let ch = match &hash_address[c..=c] {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" => {
                    lowercase_address[c..=c].to_lowercase()
                }
                _ => lowercase_address.to_string()[c..=c].to_uppercase(),
            };
            checksum_address.push_str(&ch);
        }
        checksum_address
    }
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for Address {
    fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

impl ToHex for H256  {
    fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use web3::types::{Address, H256};
    use crate::utils::ToHex;
    use super::AddressUtils;

    #[test]
    fn address_utils() {
        let addr = Address::from_str("0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199").unwrap();
        println!("{}", addr.to_hex());
        println!("{}", addr.checksum_address())
    }

    #[test]
    fn to_hex() {
        let hash = H256::random();
        println!("{}", hash.to_hex())
    }

}
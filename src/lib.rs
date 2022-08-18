pub mod engine;
pub mod utils;
pub mod abi;
pub mod contract;
pub mod tx;

#[cfg(test)]
mod tests {
    use crate::engine::Engine;

    #[tokio::test]
    async fn new_engine() {
        let e = Engine::new("http://127.0.0.1:8545").unwrap();
    }

    #[tokio::test]
    async fn erc20() {

    }

}
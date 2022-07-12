use std::time::Duration;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub struct SolanaWallet {
    rpc_client: RpcClient,
    pub_key: Pubkey
}

// TODO: Get rids of unwrap and improve API.

impl SolanaWallet {
    pub fn new(address: &str) -> Self {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let rpc_timeout = Duration::from_secs(30);
        let confirm_transaction_initial_timeout = Duration::from_secs(5);
        let commitment_config = CommitmentConfig::confirmed();
        let rpc_client = RpcClient::new_with_timeouts_and_commitment(rpc_url, rpc_timeout, commitment_config, confirm_transaction_initial_timeout);
        let pub_key = Pubkey::from_str(address).unwrap();
        Self { rpc_client, pub_key }
    }

    pub fn get_balance(&self) -> f64 {
        let lamports = self.rpc_client.get_balance(&self.pub_key).unwrap() as f64;
        let sol = lamports * 0.000000001;
        sol
    }
}
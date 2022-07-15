use std::time::Duration;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_client::rpc_response::RpcKeyedAccount;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use super::constants::*;


pub struct SolanaWallet {
    rpc_client: RpcClient,
    pub_key: Pubkey
}

pub struct UserWallet {
    pub address: String,
    pub rpc: String
}



// TODO: Get rid of unwrap and improve API.
// TODO Map response as Human Redable - get_token_accounts

impl SolanaWallet {
    pub fn new(user: UserWallet) -> Self {
        let rpc_timeout = Duration::from_secs(30);
        let confirm_transaction_initial_timeout = Duration::from_secs(5);
        let commitment_config = CommitmentConfig::confirmed();
        let rpc_client = RpcClient::new_with_timeouts_and_commitment(user.rpc.as_str(), rpc_timeout, commitment_config, confirm_transaction_initial_timeout);
        let pub_key = Pubkey::from_str(user.address.as_str()).unwrap();
        Self { rpc_client, pub_key }
    }

    pub fn get_balance(&self) -> u64 {
        let lamports = self.rpc_client.get_balance(&self.pub_key).unwrap() as u64;
        let sol = lamports / LAMPORTS_PER_SOL;
        sol
    }

    pub fn get_token_accounts(&self) -> Vec<RpcKeyedAccount> {
       let accounts  = self.rpc_client.get_token_accounts_by_owner(&self.pub_key, TokenAccountsFilter::ProgramId(Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap())).unwrap();
        accounts
    }
}

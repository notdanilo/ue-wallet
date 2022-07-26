use std::error::Error;
use super::constants::*;
use super::parser::*;
use std::time::Duration;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_client::rpc_response::RpcKeyedAccount;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_client::client_error::ClientError;
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
use solana_sdk::account::ReadableAccount;
use anyhow::{anyhow, Result as AnyResult};
use serde_json::Value;


pub struct SolanaWallet {
    rpc_client: RpcClient,
    pub_key: Pubkey
}

pub struct UserWallet {
    pub address: String,
    pub rpc: String
}

// TODO: Get rid of unwrap and improve API.

impl SolanaWallet {
    pub fn new(user: UserWallet) -> Self {
        let rpc_timeout = Duration::from_secs(30);
        let confirm_transaction_initial_timeout = Duration::from_secs(5);
        let commitment_config = CommitmentConfig::confirmed();
        let rpc_client = RpcClient::new_with_timeouts_and_commitment(user.rpc.as_str(), rpc_timeout, commitment_config, confirm_transaction_initial_timeout);
        let pub_key = Pubkey::from_str(user.address.as_str()).unwrap();
        Self { rpc_client, pub_key }
    }

    pub fn get_balance(&self) -> Result<u64, ClientError>  {
        let lamports = self.rpc_client.get_balance(&self.pub_key).unwrap() as u64;
        let balance = lamports / LAMPORTS_PER_SOL;
        return Ok(balance);
    }

    pub fn get_token_accounts(&self) -> Result<Vec<TokenBalance>, ClientError> {
       let accounts  = self.rpc_client.get_token_accounts_by_owner(
           &self.pub_key,
           TokenAccountsFilter::ProgramId(Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap())
            )?
           .iter()
           .map(|token_account| -> TokenBalance {
               parse_token_account(token_account,&self.pub_key)
            })
           .collect();
        return Ok(accounts);
    }

    pub fn get_nft_accounts(&self) -> AnyResult<Value,DecodeError> {
       let accounts = self.get_token_accounts().unwrap();

        let filtered_accounts: Vec<String> =  accounts
            .into_iter()
            .filter(|s| s.amount == "1" && s.decimal == "0")
            .map(|s| s.token_address)
            .collect();

         let metadata_accounts = filtered_accounts.iter().map(|z| decode(&self.rpc_client,&z).unwrap()).collect();
        Ok(metadata_accounts)
    }
}

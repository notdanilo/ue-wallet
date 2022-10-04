use super::constants::*;
use super::parser::*;

use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_client::client_error::ClientError;

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use anyhow::Result as AnyResult;
use serde_json::Value;
use serde::{Serialize, Deserialize};

use std::time::Duration;
use std::str::FromStr;

// use solana_client::rpc_response::RpcKeyedAccount;
// use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
// use solana_sdk::account::ReadableAccount;



pub struct SolanaWallet {
    rpc_client: RpcClient,
    pub_key: Pubkey
}

pub struct UserWallet {
    pub address: String,
    pub rpc: String
}

// TODO: Remove this
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub image: String
}

// TODO: Get rid of unwrap/expects and improve API.
impl SolanaWallet {
    pub fn new(user: UserWallet) -> Self {
        let rpc_timeout = Duration::from_secs(60);
        let confirm_transaction_initial_timeout = Duration::from_secs(60);
        let commitment_config = CommitmentConfig::confirmed();
        let rpc_client = RpcClient::new_with_timeouts_and_commitment(user.rpc.as_str(), rpc_timeout, commitment_config, confirm_transaction_initial_timeout);
        let pub_key = Pubkey::from_str(user.address.as_str()).expect("Failed to get public key from string");
        Self { rpc_client, pub_key }
    }

    pub fn get_address(&self) -> String {
        self.pub_key.to_string()
    }

    pub fn get_balance(&self) -> Result<f64, ClientError>  {
        let lamports = self.rpc_client.get_balance(&self.pub_key).expect("Failed to get balance") as u64;
        let balance = lamports as f64 / LAMPORTS_PER_SOL as f64;
        return Ok(balance);
    }

    pub fn get_token_accounts(&self) -> Result<Vec<TokenBalance>, ClientError> {
       let accounts  = self.rpc_client.get_token_accounts_by_owner(
           &self.pub_key,
           TokenAccountsFilter::ProgramId(Pubkey::from_str(TOKEN_PROGRAM_ID).expect("Failed to get token program public key from string."))
            )?
           .iter()
           .map(|token_account| -> TokenBalance {
               parse_token_account(token_account,&self.pub_key)
            })
           .collect();
        return Ok(accounts);
    }

    // TODO: Implement metaplex deserializer
    pub fn get_nft_accounts(&self) -> AnyResult<Vec<Value>,DecodeError> {
       let accounts = self.get_token_accounts().expect("Failed to get token accounts.");

        let filtered_accounts: Vec<String> =  accounts
            .into_iter()
            .filter(|s| s.amount == "1" && s.decimal == "0")
            .map(|s| s.token_address)
            .collect();

         let metadata_accounts = filtered_accounts
             .iter()
             .map(|z| decode(&self.rpc_client,&z).expect("Failed to decode."))
             .collect();
        Ok(metadata_accounts)
    }

    // TODO: Remove this function.
    pub fn get_nft_uris(&self) -> AnyResult<Vec<String>, DecodeError> {
        self
            .get_nft_accounts()
            .map(|v| v
                .iter()
                .filter_map(|value| value.get("nft_data"))
                .filter_map(|value| value.get("uri"))
                .filter_map(|value| value.as_str())
                .map(String::from)
                .map(|metadata_uri|
                    reqwest::blocking::get(metadata_uri)
                        .and_then(|response| response.json::<Metadata>())
                        .map(|metadata| metadata.image)
                        .unwrap_or_default()
                )
                .collect()
            )
    }
}

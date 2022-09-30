use std::str::FromStr;
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_response::RpcKeyedAccount;
use solana_account_decoder::{parse_token::TokenAccountType, UiAccountData};
use mpl_token_metadata::state::CollectionDetails;
use mpl_token_metadata::state::{Key, Metadata, TokenStandard, UseMethod};
use solana_client::rpc_client::RpcClient;
use solana_client::client_error::ClientErrorKind;
use thiserror::Error;
use retry::{delay::Exponential, retry};
use borsh::BorshDeserialize;
use serde_json::{json, Value};
use anyhow::Result as AnyResult;


#[derive(Serialize, Debug)]
pub struct TokenBalance {
    pub token_address: String,
    pub token_account: String,
    pub amount: String,
    pub decimal: String,
    pub owner_pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct JSONCreator {
    pub address: String,
    pub verified: bool,
    pub share: u8,
}

#[derive(Debug, Serialize)]
pub struct JSONCollection {
    pub verified: bool,
    pub key: String,
}

#[derive(Debug, Serialize)]
pub enum JSONCollectionDetails {
    V1 { size: u64 },
}

#[derive(Debug, Serialize)]
pub struct JSONUses {
    pub use_method: String,
    pub remaining: u64,
    pub total: u64,
}

pub fn parse_token_account(account: &RpcKeyedAccount, owner: &Pubkey) -> TokenBalance {
    if let UiAccountData::Json(parsed_account) = account.account.data.clone() {
        match serde_json::from_value(parsed_account.parsed) {
            Ok(TokenAccountType::Account(ui_token_account)) => {
                let mint = ui_token_account.mint.clone();
                return TokenBalance {
                    token_address: mint,
                    token_account: account.pubkey.clone(),
                    amount: ui_token_account.token_amount.ui_amount_string,
                    decimal: ui_token_account.token_amount.decimals.to_string(),
                    owner_pubkey: owner.to_string(),
                };
            }
            Ok(_) => panic!("unsupported account type"),
            Err(err) => panic!("Error while parsing account {:?}", err),
        }
    } else {
        panic!("Failed to parse account")
    }
}

pub fn decode(client: &RpcClient, mint_account: &str) -> AnyResult<Value,DecodeError> {
    let pubkey = match Pubkey::from_str(mint_account) {
        Ok(pubkey) => pubkey,
        Err(_) => return Err(DecodeError::PubkeyParseFailed(mint_account.to_string())),
    };
    let metadata_pda = get_metadata(pubkey);

    let account_data = match retry(
        Exponential::from_millis_with_factor(250, 2.0).take(3),
        || client.get_account_data(&metadata_pda),
    ) {
        Ok(data) => data,
        Err(err) => {
            return Err(DecodeError::NetworkError(err.to_string()));
        }
    };

    let metadata: Metadata = match Metadata::deserialize(&mut account_data.as_slice()) {
        Ok(m) => m,
        Err(err) => return Err(DecodeError::DecodeMetadataFailed(err.to_string())),
    };

    let json_metadata = decode_to_json(metadata).unwrap();
    Ok(json_metadata)
}

fn decode_to_json(metadata: Metadata) -> AnyResult<Value> {
    let mut creators: Vec<JSONCreator> = Vec::new();

    if let Some(c) = metadata.data.creators {
        creators = c
            .iter()
            .map(|c| JSONCreator {
                address: c.address.to_string(),
                verified: c.verified,
                share: c.share,
            })
            .collect::<Vec<JSONCreator>>();
    }

    let data_json = json!({
        "name": metadata.data.name.trim_matches(char::from(0)),
        "symbol": metadata.data.symbol.trim_matches(char::from(0)),
        "seller_fee_basis_points": metadata.data.seller_fee_basis_points,
        "uri": metadata.data.uri.trim_matches(char::from(0)),
        "creators": creators,
    });

    let mut token_standard: Option<String> = None;
    if let Some(ts) = metadata.token_standard {
        token_standard = Some(parse_token_standard(ts))
    }

    let mut collection: Option<JSONCollection> = None;
    if let Some(c) = metadata.collection {
        collection = Some(JSONCollection {
            verified: c.verified,
            key: c.key.to_string(),
        })
    }

    let mut collection_details: Option<JSONCollectionDetails> = None;
    if let Some(details) = metadata.collection_details {
        match details {
            CollectionDetails::V1 { size } => {
                collection_details = Some(JSONCollectionDetails::V1 { size })
            }
        }
    }

    let mut uses: Option<JSONUses> = None;
    if let Some(u) = metadata.uses {
        uses = Some(JSONUses {
            use_method: parse_use_method(u.use_method),
            remaining: u.remaining,
            total: u.total,
        })
    }

    let json_metadata = json!({
        "key": parse_key(metadata.key),
        "update_authority": metadata.update_authority.to_string(),
        "mint_account": metadata.mint.to_string(),
        "nft_data": data_json,
        "primary_sale_happened": metadata.primary_sale_happened,
        "is_mutable": metadata.is_mutable,
        "edition_nonce": metadata.edition_nonce,
        "token_standard": token_standard,
        "collection": collection,
        "uses": uses,
        "collection_details": collection_details,
    });

    Ok(json_metadata)
}

pub fn get_metadata(mint: Pubkey) -> Pubkey {
    let (metadata, _) = Pubkey::find_program_address(
        &[
            mpl_token_metadata::state::PREFIX.as_bytes(),
            mpl_token_metadata::id().as_ref(),
            mint.as_ref(),
        ],
        &mpl_token_metadata::id(),
    );
    metadata
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("no account data found")]
    MissingAccount(String),

    #[error("failed to get account data")]
    ClientError(ClientErrorKind),

    #[error("network request failed after three attempts: ensure you used a valid address and check the state of the Solana cluster")]
    NetworkError(String),

    #[error("failed to parse string into Pubkey")]
    PubkeyParseFailed(String),

    #[error("failed to decode metadata")]
    DecodeMetadataFailed(String),
}

fn parse_key(key: Key) -> String {
    match key {
        Key::Uninitialized => String::from("Uninitialized"),
        Key::EditionV1 => String::from("EditionV1"),
        Key::MasterEditionV1 => String::from("MasterEditionV1"),
        Key::ReservationListV1 => String::from("ReservationListV1"),
        Key::MetadataV1 => String::from("MetadataV1"),
        Key::ReservationListV2 => String::from("ReservationListV2"),
        Key::MasterEditionV2 => String::from("MasterEditionV2"),
        Key::EditionMarker => String::from("EditionMarker"),
        Key::UseAuthorityRecord => String::from("UseAuthorityRecord"),
        Key::CollectionAuthorityRecord => String::from("CollectionAuthorityRecord"),
    }
}

fn parse_token_standard(token_standard: TokenStandard) -> String {
    match token_standard {
        TokenStandard::NonFungible => String::from("NonFungible"),
        TokenStandard::FungibleAsset => String::from("FungibleAsset"),
        TokenStandard::Fungible => String::from("Fungible"),
        TokenStandard::NonFungibleEdition => String::from("NonFungibleEdition"),
    }
}

fn parse_use_method(use_method: UseMethod) -> String {
    match use_method {
        UseMethod::Burn => String::from("Burn"),
        UseMethod::Single => String::from("Single"),
        UseMethod::Multiple => String::from("Multiple"),
    }
}
pub mod solana;
pub mod constants;
pub mod parser;

pub mod ffi;

#[cfg(test)]
mod tests {
    use crate::solana::{SolanaWallet, UserWallet};


    #[test]
    fn client() {
        dotenv::dotenv().unwrap();
        let user_wallet = UserWallet {
            address: std::env::var("SOLANA_WALLET").unwrap().to_string(),
            rpc :std::env::var("RPC_URL").unwrap().to_string()
        };
        let wallet = SolanaWallet::new(user_wallet);
        println!("{:?} SOL", wallet.get_balance().unwrap());
    }

    #[test]
    fn get_token_accounts() {
        dotenv::dotenv().unwrap();
        let user_wallet = UserWallet {
            address: std::env::var("SOLANA_WALLET").unwrap().to_string(),
            rpc :std::env::var("RPC_URL").unwrap().to_string()
        };
        let wallet = SolanaWallet::new(user_wallet);
        println!("{:?}", wallet.get_token_accounts().unwrap());
    }

    #[test]
    fn get_nft_token_accounts() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().unwrap();
        let user_wallet = UserWallet {
            address: std::env::var("SOLANA_WALLET").unwrap().to_string(),
            rpc :std::env::var("RPC_URL").unwrap().to_string()
        };
        let wallet = SolanaWallet::new(user_wallet);
        let nfts = wallet.get_nft_accounts()?
            .iter()
            .map(|value| value.get("nft_data").unwrap().get("uri").unwrap().as_str().unwrap().into())
            .collect::<Vec<String>>();
        println!("{:#?}", nfts);
        Ok(())
    }

    #[test]
    fn get_nft_uris() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().unwrap();
        let user_wallet = UserWallet {
            address: std::env::var("SOLANA_WALLET").unwrap().to_string(),
            rpc :std::env::var("RPC_URL").unwrap().to_string()
        };
        let wallet = SolanaWallet::new(user_wallet);
        let nfts = wallet.get_nft_uris()?;
        println!("{:#?}", nfts);
        Ok(())
    }
}

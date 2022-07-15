pub mod solana;
pub mod constants;

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
        println!("{} SOL", wallet.get_balance());
    }

    #[test]
    fn get_token_accounts() {
        dotenv::dotenv().unwrap();
        let user_wallet = UserWallet {
            address: std::env::var("SOLANA_WALLET").unwrap().to_string(),
            rpc :std::env::var("RPC_URL").unwrap().to_string()
        };
        let wallet = SolanaWallet::new(user_wallet);
        println!("{:?}", wallet.get_token_accounts());
    }
}

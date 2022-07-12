pub mod solana;

#[cfg(test)]
mod tests {
    use crate::solana::SolanaWallet;

    #[test]
    fn client() {
        dotenv::dotenv().unwrap();
        let address = std::env::var("SOLANA_WALLET").unwrap();
        let wallet = SolanaWallet::new(&address);
        println!("{} SOL", wallet.get_balance());
    }
}

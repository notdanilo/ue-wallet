use crate::solana::{SolanaWallet, UserWallet};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn wallet_new(address: *const c_char) -> *mut SolanaWallet {
    let address = unsafe {
        CStr::from_ptr(address).to_owned().into_string().unwrap()
    };
    let rpc = "https://api.mainnet-beta.solana.com".into();
    let user_wallet = UserWallet { address, rpc };
    Box::into_raw(Box::new(SolanaWallet::new(user_wallet)))
}

#[no_mangle]
pub extern "C" fn wallet_get_address(wallet: *mut SolanaWallet) -> *mut CString {
    let address = unsafe {
        wallet
            .as_ref()
            .map(|wallet| wallet.get_address())
            .unwrap_or_default()
    };
    Box::into_raw(Box::new(CString::new(address).expect("Failed to convert address.")))
}

#[no_mangle]
pub extern "C" fn wallet_destroy(solana_wallet: *mut SolanaWallet) {
    unsafe {
        if !solana_wallet.is_null() {
            Box::from_raw(solana_wallet);
        }
    }
}

#[no_mangle]
pub extern "C" fn wallet_get_balance(solana_wallet: *mut SolanaWallet) -> f64 {
    unsafe {
        solana_wallet
            .as_ref()
            .map(|wallet| wallet.get_balance().unwrap_or_default())
            .unwrap_or_default()
    }
}

#[no_mangle]
pub extern "C" fn wallet_list_nfts(solana_wallet: *mut SolanaWallet) -> *mut Vec<*mut CString> {
    let uris = unsafe {
        solana_wallet
            .as_ref()
            .map(|wallet| {
                wallet.get_nft_uris()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| Box::into_raw(Box::new(CString::new(s).expect("Failed to convert URI."))))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    };
    Box::into_raw(Box::new(uris))
}

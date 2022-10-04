// Fill out your copyright notice in the Description page of Project Settings.


#include "Wallet.h"
#include <iostream>

UWallet::~UWallet() {
	if (Manager && Manager->WalletDestroy) Manager->WalletDestroy(Instance);
}

UWallet* UWallet::CreateWallet(UWalletManager* Manager, const FString& Address) {
	UWallet* Wallet = NewObject<UWallet>(Manager);
	Wallet->Manager = Manager;
	Wallet->Instance = Manager->WalletNew ? Manager->WalletNew(TCHAR_TO_ANSI(*Address)) : 0;
	return Wallet;
}

FString UWallet::GetAddress() const {
	void* rstring = Manager->WalletGetAddress(Instance);
	FString Address = FString(Manager->StringAsCStr(rstring));
	Manager->StringDestroy(rstring);
	return Address;
}

TArray<UNFT*> UWallet::ListNFTs() const {
	TArray<UNFT*> nfts;
	void* uri_vector = Manager->WalletListNFTs(Instance);
	int length = Manager->VectorLength(uri_vector);
	void** uris = Manager->VectorPointer(uri_vector);
	for (int i = 0; i < length; i++) {
		void* uri = uris[i];
		UNFT* nft = NewObject<UNFT>(UNFT::StaticClass());
		nft->URI = Manager->StringAsCStr(uri);
		nfts.Add(nft);
		Manager->StringDestroy(uri);
	}
	Manager->VectorDestroy(uri_vector);
	return nfts;
}

double UWallet::GetBalance() const {
	return Manager && Manager->WalletGetBalance ? Manager->WalletGetBalance(Instance) : 0.0;
}

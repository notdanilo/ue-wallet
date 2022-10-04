// Fill out your copyright notice in the Description page of Project Settings.


#include "WalletManager.h"
#include "Wallet.h"
#include "Interfaces/IPluginManager.h"

UWalletManager::UWalletManager() {
	FString BaseDir = IPluginManager::Get().FindPlugin("Wallet")->GetBaseDir();
	FString DLLPath = FPaths::Combine(BaseDir, "Binaries", "Win64", "wallet.dll");
	DLLHandle = FPlatformProcess::GetDllHandle(*DLLPath);
	if (DLLHandle) {
		WalletNew = (void* (*)(const char*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("wallet_new"));
		WalletDestroy = (void (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("wallet_destroy"));
		WalletGetBalance = (double (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("wallet_get_balance"));
		WalletGetAddress = (void* (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("wallet_get_address"));
		WalletListNFTs = (void* (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("wallet_list_nfts"));

		StringAsCStr = (char* (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("string_as_cstr"));
		StringDestroy = (void (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("string_destroy"));

		VectorDestroy = (void (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("vector_destroy"));
		VectorLength = (int (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("vector_length"));
		VectorPointer = (void** (*)(void*)) FPlatformProcess::GetDllExport(DLLHandle, TEXT("vector_pointer"));
	}
	UE_LOG(LogTemp, Warning, TEXT("UWalletManager"));
}

UWalletManager::~UWalletManager() {
	if (DLLHandle) {
		FPlatformProcess::FreeDllHandle(DLLHandle);
		DLLHandle = 0;
		WalletNew = 0;
		WalletDestroy = 0;
		WalletGetBalance = 0;
		WalletGetAddress = 0;
		WalletListNFTs = 0;

		StringAsCStr = 0;
		StringDestroy = 0;

		VectorDestroy = 0;
		VectorLength = 0;
		VectorPointer = 0;
	}
	UE_LOG(LogTemp, Warning, TEXT("~UWalletManager"));
}

UWallet* UWalletManager::GetOrCreateWallet(const FString& SlotName, const FString& AddressIfCreated) {
	UWallet** StoredWallet = Wallets.Find(SlotName);

	if (StoredWallet && IsValid(*StoredWallet)) return *StoredWallet;
	else {
		UWallet* NewWallet = UWallet::CreateWallet(this, AddressIfCreated);
		Wallets.Add(SlotName, NewWallet);
		return NewWallet;
	}
}

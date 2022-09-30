// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "NFT.h"
#include "WalletManager.h"
#include "Wallet.generated.h"

/**
 * 
 */
UCLASS(BlueprintType)
class WALLET_API UWallet : public UObject
{
	GENERATED_BODY()
public:
	static UWallet* CreateWallet(UWalletManager* Manager, const FString& Address);
	virtual ~UWallet();

	UFUNCTION(BlueprintCallable, Category = "Wallet")
	double GetBalance() const;

	UFUNCTION(BlueprintCallable, Category = "Wallet")
	FString GetAddress() const;

	UFUNCTION(BlueprintCallable, Category = "Wallet")
	TArray<UNFT*> ListNFTs() const;

private:
	UWalletManager* Manager = 0;
	void* Instance = 0;
};

// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"
#include "WalletManager.generated.h"

class UWallet;

/**
 * 
 */
UCLASS()
class WALLET_API UWalletManager : public UGameInstanceSubsystem
{
	GENERATED_BODY()
public:
	UFUNCTION(BlueprintCallable, Category = "Wallet")
	UWallet* GetOrCreateWallet(const FString& SlotName, const FString& AddressIfCreated);

	UWalletManager();
	virtual ~UWalletManager();

	/*
	Is there any reason why we should use these instead of constructors and destructors?
	virtual void Initialize(FSubsystemCollectionBase& Collection);
	virtual void Deinitialize();
	*/

	void* (*WalletNew)(const char*) = 0;
	void (*WalletDestroy)(void* wallet) = 0;
	double (*WalletGetBalance)(void* wallet) = 0;
	void* (*WalletGetAddress)(void* wallet) = 0;
	void* (*WalletListNFTs)(void* wallet) = 0;

	char* (*StringAsCStr)(void* rstring) = 0;
	void (*StringDestroy)(void* rstring) = 0;

	void (*VectorDestroy)(void* vector) = 0;
	int (*VectorLength)(void* vector) = 0;
	void** (*VectorPointer)(void* vector) = 0;
private:
	void* DLLHandle = 0;
	TMap<FString, UWallet*> Wallets;
};

// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "Wallet.generated.h"

/**
 * 
 */
UCLASS()
class WALLET_API UWallet : public UObject
{
public:
	GENERATED_BODY()
	UFUNCTION(BlueprintCallable, Category = "Wallet")
	static int CreateWallet();
};

// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "NFT.generated.h"

/**
 * 
 */
UCLASS(BlueprintType)
class WALLET_API UNFT : public UObject
{
public:
	GENERATED_BODY()

	UPROPERTY(BlueprintReadWrite, Category = "Wallet|NFT")
	FString URI;
};

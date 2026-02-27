// File: errors.rs - This file is part of Tetcore
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
// NFT module error types for minting, transferring, and marketplace operations.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NftError {
    CollectionNotFound,
    CollectionSupplyExceeded,
    NotCollectionOwner,
    AssetNotFound,
    NotAssetOwner,
    AlreadyOwned,
    InsufficientBalance,
    InsufficientPayment,
    ListingNotFound,
    ListingExpired,
    CannotTransferToSelf,
    ApprovalNotFound,
    MaxSupplyReached,
    InvalidTokenId,
    MetadataNotFound,
    RoyaltyNotSet,
    InvalidRoyalty,
    NoPermission,
}

impl NftError {
    pub fn as_u32(&self) -> u32 {
        match self {
            NftError::CollectionNotFound => 0,
            NftError::CollectionSupplyExceeded => 1,
            NftError::NotCollectionOwner => 2,
            NftError::AssetNotFound => 3,
            NftError::NotAssetOwner => 4,
            NftError::AlreadyOwned => 5,
            NftError::InsufficientBalance => 6,
            NftError::InsufficientPayment => 7,
            NftError::ListingNotFound => 8,
            NftError::ListingExpired => 9,
            NftError::CannotTransferToSelf => 10,
            NftError::ApprovalNotFound => 11,
            NftError::MaxSupplyReached => 12,
            NftError::InvalidTokenId => 13,
            NftError::MetadataNotFound => 14,
            NftError::RoyaltyNotSet => 15,
            NftError::InvalidRoyalty => 16,
            NftError::NoPermission => 17,
        }
    }
}

impl core::fmt::Debug for NftError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NftError::CollectionNotFound => write!(f, "Collection not found"),
            NftError::CollectionSupplyExceeded => write!(f, "Collection supply exceeded"),
            NftError::NotCollectionOwner => write!(f, "Not collection owner"),
            NftError::AssetNotFound => write!(f, "Asset not found"),
            NftError::NotAssetOwner => write!(f, "Not asset owner"),
            NftError::AlreadyOwned => write!(f, "Already owned"),
            NftError::InsufficientBalance => write!(f, "Insufficient balance"),
            NftError::InsufficientPayment => write!(f, "Insufficient payment"),
            NftError::ListingNotFound => write!(f, "Listing not found"),
            NftError::ListingExpired => write!(f, "Listing expired"),
            NftError::CannotTransferToSelf => write!(f, "Cannot transfer to self"),
            NftError::ApprovalNotFound => write!(f, "Approval not found"),
            NftError::MaxSupplyReached => write!(f, "Max supply reached"),
            NftError::InvalidTokenId => write!(f, "Invalid token ID"),
            NftError::MetadataNotFound => write!(f, "Metadata not found"),
            NftError::RoyaltyNotSet => write!(f, "Royalty not set"),
            NftError::InvalidRoyalty => write!(f, "Invalid royalty"),
            NftError::NoPermission => write!(f, "No permission"),
        }
    }
}

impl core::fmt::Display for NftError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for NftError {}

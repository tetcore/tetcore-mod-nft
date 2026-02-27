// File: lib.rs - This file is part of Tetcore
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
// Commodities Fabric Module: NFTs for Tetcore
// A module for managing non-fungible tokens (NFTs) optimized for frequent trading.
// Assets have an owner and can be created, destroyed, and transferred.

pub mod nft;
pub mod types;
pub mod errors;

pub use nft::NftModule;
pub use types::{AssetId, AssetInfo, CollectionId, NftId, NftInfo, TokenBalance};
pub use errors::NftError;

// File: nft.rs - This file is part of Tetcore
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
// NFT module implementation providing minting, burning, transferring, and
// marketplace functionality for non-fungible tokens.

use crate::errors::NftError;
use crate::types::Address;
use crate::types::{
    AssetId, AssetInfo, Collection, CollectionId, NftId, NftInfo, NftListing, NftState, Ownership,
    RoyaltyInfo, TokenBalance,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct NftModule {
    collections: HashMap<CollectionId, Collection>,
    nfts: HashMap<AssetId, NftInfo>,
    ownership: HashMap<AssetId, Ownership>,
    next_collection_id: CollectionId,
    next_nft_id: u64,
    listings: HashMap<AssetId, NftListing>,
    collection_royalty: HashMap<CollectionId, Option<RoyaltyInfo>>,
}

impl NftModule {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
            nfts: HashMap::new(),
            ownership: HashMap::new(),
            next_collection_id: CollectionId::default(),
            next_nft_id: 0,
            listings: HashMap::new(),
            collection_royalty: HashMap::new(),
        }
    }

    pub fn create_collection(
        &mut self,
        name: String,
        description: String,
        owner: Address,
        max_supply: Option<u64>,
    ) -> Result<CollectionId, NftError> {
        let collection_id = self.next_collection_id;
        self.next_collection_id = CollectionId(collection_id.0 + 1);

        let mut collection = Collection::new(collection_id, name, description, owner);
        if let Some(max) = max_supply {
            collection = collection.with_max_supply(max);
        }

        self.collections.insert(collection_id, collection);
        self.collection_royalty.insert(collection_id, None);

        Ok(collection_id)
    }

    pub fn set_collection_royalty(
        &mut self,
        collection_id: CollectionId,
        royalty: Option<RoyaltyInfo>,
        sender: Address,
    ) -> Result<(), NftError> {
        let collection = self
            .collections
            .get(&collection_id)
            .ok_or(NftError::CollectionNotFound)?;

        if collection.owner != sender {
            return Err(NftError::NotCollectionOwner);
        }

        self.collection_royalty.insert(collection_id, royalty);
        Ok(())
    }

    pub fn mint(
        &mut self,
        collection_id: CollectionId,
        owner: Address,
        info: AssetInfo,
    ) -> Result<AssetId, NftError> {
        let collection = self
            .collections
            .get(&collection_id)
            .ok_or(NftError::CollectionNotFound)?;

        if !collection.can_mint() {
            return Err(NftError::CollectionSupplyExceeded);
        }

        let nft_id = NftId(self.next_nft_id);
        self.next_nft_id += 1;

        let asset_id = AssetId::new(collection_id, nft_id);

        let mut nft_info = NftInfo::new(nft_id, owner, info, collection_id);
        nft_info.created_at = 0;

        self.nfts.insert(asset_id.clone(), nft_info);
        self.ownership
            .insert(asset_id.clone(), Ownership::new(owner));

        if let Some(collection) = self.collections.get_mut(&collection_id) {
            collection.total_minted += 1;
        }

        Ok(asset_id)
    }

    pub fn mint_batch(
        &mut self,
        collection_id: CollectionId,
        owner: Address,
        infos: Vec<AssetInfo>,
    ) -> Result<Vec<AssetId>, NftError> {
        let mut results = Vec::new();
        for info in infos {
            let asset_id = self.mint(collection_id, owner, info)?;
            results.push(asset_id);
        }
        Ok(results)
    }

    pub fn burn(&mut self, asset_id: &AssetId, sender: Address) -> Result<(), NftError> {
        let ownership = self
            .ownership
            .get(asset_id)
            .ok_or(NftError::AssetNotFound)?;

        if ownership.owner != sender {
            return Err(NftError::NotAssetOwner);
        }

        self.nfts.remove(asset_id);
        self.ownership.remove(asset_id);
        self.listings.remove(asset_id);

        if let Some(collection) = self.collections.get_mut(&asset_id.collection_id) {
            collection.total_burned += 1;
        }

        Ok(())
    }

    pub fn transfer(
        &mut self,
        asset_id: &AssetId,
        from: Address,
        to: Address,
    ) -> Result<(), NftError> {
        let ownership = self
            .ownership
            .get_mut(asset_id)
            .ok_or(NftError::AssetNotFound)?;

        if ownership.owner != from && Some(&from) != ownership.approved.as_ref() {
            return Err(NftError::NotAssetOwner);
        }

        ownership.owner = to;
        ownership.clear_approval();

        if let Some(nft_info) = self.nfts.get_mut(asset_id) {
            nft_info.owner = to;
            nft_info.transfers += 1;
        }

        Ok(())
    }

    pub fn approve(
        &mut self,
        asset_id: &AssetId,
        owner: Address,
        approved: Address,
    ) -> Result<(), NftError> {
        let ownership = self
            .ownership
            .get_mut(asset_id)
            .ok_or(NftError::AssetNotFound)?;

        if ownership.owner != owner {
            return Err(NftError::NotAssetOwner);
        }

        ownership.approve(approved);
        Ok(())
    }

    pub fn set_approval_for_all(
        &mut self,
        owner: Address,
        operator: Address,
        approved: bool,
    ) -> Result<(), NftError> {
        for (_, ownership) in self.ownership.iter_mut() {
            if ownership.owner == owner {
                if approved {
                    ownership.approve(operator);
                } else if ownership.approved == Some(operator) {
                    ownership.clear_approval();
                }
            }
        }
        Ok(())
    }

    pub fn owner_of(&self, asset_id: &AssetId) -> Option<Address> {
        self.ownership.get(asset_id).map(|o| o.owner)
    }

    pub fn get_asset(&self, asset_id: &AssetId) -> Option<&NftInfo> {
        self.nfts.get(asset_id)
    }

    pub fn get_collection(&self, collection_id: &CollectionId) -> Option<&Collection> {
        self.collections.get(collection_id)
    }

    pub fn assets_for_account(&self, account: Address) -> Vec<(AssetId, &NftInfo)> {
        self.nfts
            .iter()
            .filter_map(|(id, info)| {
                if let Some(ownership) = self.ownership.get(id) {
                    if ownership.owner == account {
                        return Some((id.clone(), info));
                    }
                }
                None
            })
            .collect()
    }

    pub fn total_assets(&self) -> u64 {
        self.nfts.len() as u64
    }

    pub fn total_for_account(&self, account: Address) -> u64 {
        self.ownership
            .values()
            .filter(|o| o.owner == account)
            .count() as u64
    }

    pub fn total_for_collection(&self, collection_id: CollectionId) -> u64 {
        self.nfts
            .keys()
            .filter(|id| id.collection_id == collection_id)
            .count() as u64
    }

    pub fn list(
        &mut self,
        asset_id: AssetId,
        seller: Address,
        price: u128,
        currency: Address,
        expires_at: u64,
    ) -> Result<(), NftError> {
        let ownership = self
            .ownership
            .get(&asset_id)
            .ok_or(NftError::AssetNotFound)?;

        if ownership.owner != seller {
            return Err(NftError::NotAssetOwner);
        }

        let listing = NftListing::new(asset_id, seller, price, currency, expires_at);
        self.listings.insert(asset_id, listing);
        Ok(())
    }

    pub fn unlist(&mut self, asset_id: &AssetId, sender: Address) -> Result<(), NftError> {
        let listing = self
            .listings
            .get(asset_id)
            .ok_or(NftError::ListingNotFound)?;

        if listing.seller != sender {
            return Err(NftError::NotAssetOwner);
        }

        self.listings.remove(asset_id);
        Ok(())
    }

    pub fn buy(
        &mut self,
        asset_id: &AssetId,
        buyer: Address,
        payment: u128,
    ) -> Result<(), NftError> {
        let listing = self
            .listings
            .get(asset_id)
            .ok_or(NftError::ListingNotFound)?;

        if listing.is_expired(0) {
            return Err(NftError::ListingExpired);
        }

        if payment < listing.price {
            return Err(NftError::InsufficientPayment);
        }

        let seller = listing.seller;
        self.transfer(asset_id, seller, buyer)?;
        self.listings.remove(asset_id);

        Ok(())
    }

    pub fn get_listing(&self, asset_id: &AssetId) -> Option<&NftListing> {
        self.listings.get(asset_id)
    }

    pub fn all_listings(&self) -> Vec<(&AssetId, &NftListing)> {
        self.listings.iter().collect()
    }

    pub fn set_base_uri(
        &mut self,
        collection_id: CollectionId,
        base_uri: String,
        sender: Address,
    ) -> Result<(), NftError> {
        let collection = self
            .collections
            .get(&collection_id)
            .ok_or(NftError::CollectionNotFound)?;

        if collection.owner != sender {
            return Err(NftError::NotCollectionOwner);
        }

        if let Some(c) = self.collections.get_mut(&collection_id) {
            c.base_uri = Some(base_uri);
        }

        Ok(())
    }

    pub fn collection_info(&self, collection_id: &CollectionId) -> Option<&Collection> {
        self.collections.get(collection_id)
    }

    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        self.ownership
            .values()
            .filter(|o| o.owner == owner)
            .any(|o| o.approved == Some(operator))
    }
}

impl Default for NftModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address(i: u8) -> Address {
        let mut bytes = [0u8; 32];
        bytes[31] = i;
        Address(bytes)
    }

    #[test]
    fn test_create_collection() {
        let mut module = NftModule::new();
        let owner = test_address(1);

        let result = module.create_collection(
            "Test Collection".to_string(),
            "A test collection".to_string(),
            owner,
            Some(100),
        );

        assert!(result.is_ok());
        let collection_id = result.unwrap();
        assert_eq!(collection_id.0, 0);
    }

    #[test]
    fn test_mint_and_transfer() {
        let mut module = NftModule::new();
        let owner = test_address(1);
        let recipient = test_address(2);

        let collection_id = module
            .create_collection("Test".to_string(), "Test".to_string(), owner, None)
            .unwrap();

        let asset_id = module
            .mint(
                collection_id,
                owner,
                AssetInfo::new("Test NFT".to_string(), "Description".to_string()),
            )
            .unwrap();

        assert_eq!(module.owner_of(&asset_id), Some(owner));

        module.transfer(&asset_id, owner, recipient).unwrap();

        assert_eq!(module.owner_of(&asset_id), Some(recipient));
    }

    #[test]
    fn test_burn() {
        let mut module = NftModule::new();
        let owner = test_address(1);

        let collection_id = module
            .create_collection("Test".to_string(), "Test".to_string(), owner, None)
            .unwrap();

        let asset_id = module
            .mint(
                collection_id,
                owner,
                AssetInfo::new("Test NFT".to_string(), "Description".to_string()),
            )
            .unwrap();

        assert_eq!(module.total_assets(), 1);

        module.burn(&asset_id, owner).unwrap();

        assert_eq!(module.total_assets(), 0);
    }

    #[test]
    fn test_approve() {
        let mut module = NftModule::new();
        let owner = test_address(1);
        let operator = test_address(2);

        let collection_id = module
            .create_collection("Test".to_string(), "Test".to_string(), owner, None)
            .unwrap();

        let asset_id = module
            .mint(
                collection_id,
                owner,
                AssetInfo::new("Test NFT".to_string(), "Description".to_string()),
            )
            .unwrap();

        module.approve(&asset_id, owner, operator).unwrap();

        let ownership = module.ownership.get(&asset_id).unwrap();
        assert_eq!(ownership.approved, Some(operator));
    }

    #[test]
    fn test_listing() {
        let mut module = NftModule::new();
        let owner = test_address(1);
        let buyer = test_address(2);

        let collection_id = module
            .create_collection("Test".to_string(), "Test".to_string(), owner, None)
            .unwrap();

        let asset_id = module
            .mint(
                collection_id,
                owner,
                AssetInfo::new("Test NFT".to_string(), "Description".to_string()),
            )
            .unwrap();

        let currency = Address([3u8; 32]);

        module.list(asset_id, owner, 1000, currency, 1000).unwrap();

        let listing = module.get_listing(&asset_id).unwrap();
        assert_eq!(listing.price, 1000);
    }
}

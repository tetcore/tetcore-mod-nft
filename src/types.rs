// File: types.rs - This file is part of Tetcore
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
// NFT types including asset identifiers, collection info, ownership, and metadata.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Address(pub [u8; 32]);

impl Address {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Debug, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash32(pub [u8; 32]);

impl Hash32 {
    pub fn empty() -> Self {
        Self([0u8; 32])
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct CollectionId(pub u32);

impl CollectionId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct NftId(pub u64);

impl NftId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for NftId {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId {
    pub collection_id: CollectionId,
    pub nft_id: NftId,
}

impl AssetId {
    pub fn new(collection_id: CollectionId, nft_id: NftId) -> Self {
        Self {
            collection_id,
            nft_id,
        }
    }

    pub fn to_hash(&self) -> Hash32 {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&self.collection_id.0.to_le_bytes());
        hasher.update(&self.nft_id.0.to_le_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Hash32(hash)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    pub name: String,
    pub description: String,
    pub image_uri: Option<String>,
    pub metadata_uri: Option<String>,
    pub attributes: Vec<(String, String)>,
}

impl AssetInfo {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            image_uri: None,
            metadata_uri: None,
            attributes: Vec::new(),
        }
    }

    pub fn with_image(mut self, uri: String) -> Self {
        self.image_uri = Some(uri);
        self
    }

    pub fn with_metadata(mut self, uri: String) -> Self {
        self.metadata_uri = Some(uri);
        self
    }

    pub fn add_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.push((key, value));
        self
    }
}

impl Default for AssetInfo {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NftInfo {
    pub id: NftId,
    pub owner: Address,
    pub info: AssetInfo,
    pub collection_id: CollectionId,
    pub created_at: u64,
    pub transfers: u32,
}

impl NftInfo {
    pub fn new(id: NftId, owner: Address, info: AssetInfo, collection_id: CollectionId) -> Self {
        Self {
            id,
            owner,
            info,
            collection_id,
            created_at: 0,
            transfers: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: CollectionId,
    pub name: String,
    pub description: String,
    pub owner: Address,
    pub max_supply: Option<u64>,
    pub total_minted: u64,
    pub total_burned: u64,
    pub base_uri: Option<String>,
}

impl Collection {
    pub fn new(id: CollectionId, name: String, description: String, owner: Address) -> Self {
        Self {
            id,
            name,
            description,
            owner,
            max_supply: None,
            total_minted: 0,
            total_burned: 0,
            base_uri: None,
        }
    }

    pub fn with_max_supply(mut self, max: u64) -> Self {
        self.max_supply = Some(max);
        self
    }

    pub fn with_base_uri(mut self, uri: String) -> Self {
        self.base_uri = Some(uri);
        self
    }

    pub fn can_mint(&self) -> bool {
        match self.max_supply {
            Some(max) => self.total_minted < max,
            None => true,
        }
    }

    pub fn supply_left(&self) -> Option<u64> {
        self.max_supply
            .map(|max| max.saturating_sub(self.total_minted))
    }
}

pub type TokenBalance = u64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ownership {
    pub owner: Address,
    pub approved: Option<Address>,
}

impl Ownership {
    pub fn new(owner: Address) -> Self {
        Self {
            owner,
            approved: None,
        }
    }

    pub fn is_approved(&self) -> bool {
        self.approved.is_some()
    }

    pub fn approve(&mut self, approved: Address) {
        self.approved = Some(approved);
    }

    pub fn clear_approval(&mut self) {
        self.approved = None;
    }

    pub fn owner_or_approved(&self) -> Address {
        self.approved.unwrap_or(self.owner)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NftState {
    Active,
    Frozen,
    Burned,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NftListing {
    pub asset_id: AssetId,
    pub seller: Address,
    pub price: u128,
    pub currency: Address,
    pub expires_at: u64,
}

impl NftListing {
    pub fn new(
        asset_id: AssetId,
        seller: Address,
        price: u128,
        currency: Address,
        expires_at: u64,
    ) -> Self {
        Self {
            asset_id,
            seller,
            price,
            currency,
            expires_at,
        }
    }

    pub fn is_expired(&self, current_block: u64) -> bool {
        current_block > self.expires_at
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoyaltyInfo {
    pub recipient: Address,
    pub basis_points: u16,
}

impl RoyaltyInfo {
    pub fn new(recipient: Address, basis_points: u16) -> Self {
        Self {
            recipient,
            basis_points: basis_points.min(10000),
        }
    }

    pub fn calculate_royalty(&self, sale_price: u128) -> u128 {
        (sale_price * self.basis_points as u128) / 10000
    }
}

# Commodities Fabric Module: NFTs for Tetcore

[![Compatible with Tetcore](https://img.shields.io/badge/Tetcore-v0.1.0-E6007A)](https://github.com/tetcore/tetcore)

This is a Tetcore module that defines and implements an interface for managing a set of [non-fungible tokens (NFTs)](https://en.wikipedia.org/wiki/Non-fungible_token). Assets have an owner and can be created, destroyed, and transferred.

## Interface

This package defines a public module for working with NFTs, providing functionality for:

- **Collections**: Create and manage NFT collections with optional max supply
- **Minting**: Create new NFTs within collections
- **Burning**: Destroy NFTs
- **Transferring**: Transfer ownership between accounts
- **Approvals**: Approve operators to transfer on behalf of owners
- **Marketplace**: List NFTs for sale, buy listings

## Architecture

### Core Types

- **CollectionId**: Unique identifier for an NFT collection
- **NftId**: Unique identifier for an individual NFT within a collection
- **AssetId**: Composite identifier combining collection and NFT IDs
- **AssetInfo**: Metadata including name, description, image URI, attributes
- **Collection**: Collection configuration including owner, max supply, royalty

### Storage

The module maintains:
- `collections`: Map of all collections
- `nfts`: Map of all NFT data
- `ownership`: Map of asset ownership
- `listings`: Active marketplace listings

## Usage

### Creating a Collection

```rust
let mut nft_module = NftModule::new();

let collection_id = nft_module.create_collection(
    "My Collection".to_string(),
    "Description of my collection".to_string(),
    owner_address,
    Some(10000), // max supply
)?;
```

### Minting an NFT

```rust
let asset_id = nft_module.mint(
    collection_id,
    recipient_address,
    AssetInfo::new("NFT Name".to_string(), "Description".to_string())
        .with_image("ipfs://...".to_string())
        .add_attribute("rarity".to_string(), "legendary".to_string()),
)?;
```

### Transferring

```rust
nft_module.transfer(&asset_id, from_address, to_address)?;
```

### Marketplace Listing

```rust
nft_module.list(
    asset_id,
    seller_address,
    1000, // price
    currency_address,
    expires_at_block,
)?;
```

## Traits

### UniqueAssets Trait

This module implements a variation of the `UniqueAssets` trait:

- `total() -> u64`: Total number of NFTs
- `owner_of(AssetId) -> Address`: Get owner of an NFT
- `mint() -> Result<AssetId>`: Create new NFT
- `burn(AssetId)`: Destroy NFT
- `transfer(AssetId, Address, Address)`: Transfer ownership

## Design Decisions

### Sorted Storage

This implementation uses _sorted_ lists of assets per owner for efficient trading.
Assets are sorted by ID to enable:
- Fast lookups during transfer
- Efficient burning operations
- Ordered enumeration

### Marketplace

Built-in marketplace functionality supports:
- Fixed-price listings
- Expiration times
- Royalty support for collections

## Tests

The module includes comprehensive tests:

```bash
cargo test
```

Run specific test modules:

```bash
cargo test --lib nft
```

## Integration

To integrate this module into your Tetcore runtime:

1. Add to your Cargo.toml:
```toml
tetcore-mod-nft = { path = "../tetcore-mod-nft" }
```

2. Configure in your runtime:
```rust
impl tetcore_mod_nft::Config for Runtime { ... }
```

3. Add to runtime composition

## Acknowledgements

This project was inspired by works such as:

- [The ERC-721 specification](https://eips.ethereum.org/EIPS/eip-721)
- [OpenZeppelin's ERC-721 implementation](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/token/ERC721)
- [Substrate's NFT pallet](https://github.com/paritytech/substrate/tree/master/frame/nfts)
- [The original Substratekitties project](https://github.com/shawntabrizi/substrate-collectables-workshop/)
- [danforbes' pallet-nft](https://github.com/danforbes/pallet-nft)

## License

See LICENSE file.

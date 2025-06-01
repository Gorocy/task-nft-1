# NFT Minter Program

## Program ID
```
AP9Q9Zni19LTXPr1yvrv93kAVZfjnBETNey3VspK4GSh
```
## Instructions

### `mint_nft`
Mints a new NFT for a development project.

**Parameters:**
- `github_repo` (String): GitHub repository URL (max 200 characters)
- `image_url` (String): URL to the NFT image (max 200 characters)

**Accounts:**
- `minter`: The wallet minting the NFT (pays for transaction)
- `mint`: The new token mint account (created)
- `token_account`: Associated token account for the minter (created)
- `metadata_account`: Metaplex metadata account (PDA)
- `master_edition_account`: Metaplex master edition account (PDA)
- Various program accounts (Token, Associated Token, Metadata, System, Rent)

## NFT Properties

- **Name**: Extracted from GitHub URL (last segment, max 32 characters)
- **Symbol**: "DEV"
- **Decimals**: 0 (non-fungible)
- **Supply**: 1 (unique NFT)
- **Immutable**: Metadata cannot be updated after minting

## Error Codes

- `RepoTooLong`: GitHub repo URL exceeds 200 characters
- `ImageUrlTooLong`: Image URL exceeds 200 characters
- `NameTooLong`: Extracted project name exceeds 32 characters

## Events

### `NftMinted`
Emitted when an NFT is successfully minted.

**Fields:**
- `mint`: Public key of the minted token
- `minter`: Public key of the wallet that minted the NFT
- `github_repo`: The GitHub repository URL
- `image_url`: The NFT image URL
- `project_name`: The extracted project name

### Building
```bash
anchor build
```

### Testing
```bash
anchor test
```

### Deployment
```bash
anchor deploy
```

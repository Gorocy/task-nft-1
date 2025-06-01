use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

declare_id!("AP9Q9Zni19LTXPr1yvrv93kAVZfjnBETNey3VspK4GSh");

#[program]
pub mod simple_nft_minter {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>, github_repo: String, image_url: String) -> Result<()> {
        // Validate length
        require!(github_repo.len() <= 200, ErrorCode::RepoTooLong);
        require!(image_url.len() <= 200, ErrorCode::ImageUrlTooLong);

        // Improved URL validation
        require!(
            is_valid_github_url(&github_repo),
            ErrorCode::InvalidGithubUrl
        );
        require!(is_valid_image_url(&image_url), ErrorCode::InvalidImageUrl);

        // Improved project name extraction
        let project_name = extract_repo_name(&github_repo);
        require!(project_name.len() <= 32, ErrorCode::NameTooLong);

        // Mint single token
        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.minter.to_account_info(),
                },
            ),
            1, // Amount 
        )?;

        // Create Metaplex metadata
        let data_v2 = DataV2 {
            name: project_name.clone(),
            symbol: "DEV".to_string(),
            uri: image_url.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        // Create metadata account
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    mint_authority: ctx.accounts.minter.to_account_info(),
                    update_authority: ctx.accounts.minter.to_account_info(),
                    payer: ctx.accounts.minter.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            data_v2,
            true, // is_mutable
            true, // update_authority_is_signer
            None, // collection_details
        )?;

        // Create master edition
        create_master_edition_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    update_authority: ctx.accounts.minter.to_account_info(),
                    mint_authority: ctx.accounts.minter.to_account_info(),
                    payer: ctx.accounts.minter.to_account_info(),
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            Some(1), // max_supply
        )?;

        // Emit event
        emit!(NftMinted {
            mint: ctx.accounts.mint.key(),
            minter: ctx.accounts.minter.key(),
            github_repo,
            image_url,
            project_name,
        });

        Ok(())
    }
}

// Helper function to extract repo name from GitHub URL
fn extract_repo_name(github_url: &str) -> String {
    // Remove trailing slash if it exists
    let cleaned_url = github_url.trim_end_matches('/');

    if let Some(last_slash) = cleaned_url.rfind('/') {
        let mut repo_name = &cleaned_url[last_slash + 1..];

        // Remove .git from the end if it exists
        if repo_name.ends_with(".git") {
            repo_name = &repo_name[..repo_name.len() - 4];
        }

        // Check if the name is not empty and doesn't exceed the limit
        if !repo_name.is_empty() && repo_name.len() <= 32 {
            // Replace special characters with underscores (optional)
            return repo_name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
        }
    }
    "Dev Project".to_string()
}

fn is_valid_github_url(url: &str) -> bool {
    url.starts_with("https://github.com/")
}

fn is_valid_image_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    (url_lower.starts_with("https://"))
        && (url_lower.ends_with(".png")
            || url_lower.ends_with(".jpg")
            || url_lower.ends_with(".jpeg")
            || url_lower.ends_with(".gif")
            || url_lower.ends_with(".svg")
            || url_lower.contains("ipfs://")
            || url_lower.contains("arweave.net"))
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,

    #[account(
        init,
        payer = minter,
        mint::decimals = 0,
        mint::authority = minter,
        mint::freeze_authority = minter,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = minter,
        associated_token::mint = mint,
        associated_token::authority = minter,
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: Metadata account PDA validated by seeds constraint
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Master edition account PDA validated by seeds constraint
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub master_edition_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct NftMinted {
    pub mint: Pubkey,
    pub minter: Pubkey,
    pub github_repo: String,
    pub image_url: String,
    pub project_name: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("GitHub repo URL too long (max 200 characters)")]
    RepoTooLong,
    #[msg("Image URL too long (max 200 characters)")]
    ImageUrlTooLong,
    #[msg("Project name too long (max 32 characters)")]
    NameTooLong,
    #[msg("Invalid GitHub URL format")]
    InvalidGithubUrl,
    #[msg("Invalid image URL format")]
    InvalidImageUrl,
}

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};

declare_id!("VTDgMbDRsJgucL1hCaQkNcTQNjWLYfdnoh5m5tJuUHS");

#[program]
pub mod solana_mint {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // Instruction to create a new token mint and mint initial supply to the user
    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>,
        decimals: u8,
        amount: u64,
    ) -> Result<()> {
        // Initialize the mint account
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            decimals,
            ctx.accounts.authority.key,
            Some(ctx.accounts.authority.key),
        )?;

        // Mint the initial supply to the user's associated token account
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

// Context for the create_token_mint instruction
#[derive(Accounts)]
pub struct CreateTokenMint<'info> {
    // The mint account to be created (must be a signer)
    #[account(init, payer = authority, space = 82)]
    pub mint: Account<'info, Mint>,

    // The user's token account to receive the initial supply
    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub user_token_account: Account<'info, TokenAccount>,

    // The authority who pays for the account creation and will be the mint authority
    #[account(mut)]
    pub authority: Signer<'info>,

    // System program for account creation
    pub system_program: Program<'info, System>,
    // Rent sysvar
    pub rent: Sysvar<'info, Rent>,
    // SPL Token program
    pub token_program: Program<'info, Token>,
    // Associated Token program
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}

#[derive(Accounts)]
pub struct Initialize {}

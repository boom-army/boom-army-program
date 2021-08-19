//! Sosol intereaction

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use std::convert::Into;
use percentage::Percentage;

#[program]
pub mod sosol {
    use super::*;

    pub fn interaction(ctx: Context<Interaction>, interaction_fee: u64) -> ProgramResult {
        // Set storage percent split between accounts
        const STORAGE_PERCENT_SPLIT: usize = 10;

        let storage_percent = Percentage::from(STORAGE_PERCENT_SPLIT);
        let storage_fee = storage_percent.apply_to(interaction_fee);
        let creator_fee = interaction_fee - storage_fee;

        let to = ctx.accounts.to.to_account_info().clone();
        let from = ctx.accounts.from.to_account_info().clone();
        let owner = ctx.accounts.owner.to_account_info().clone();

        // Transfer funds to the content creator
        let cpi_accounts = Transfer {
            from,
            to,
            authority: owner,
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, creator_fee)?;

        // Transfer funds to the content storage host
        let to_storage = ctx.accounts.to_storage_account.to_account_info().clone();
        let from_2 = ctx.accounts.from.to_account_info().clone();
        let owner_2 = ctx.accounts.owner.to_account_info().clone();
        let cpi_accounts = Transfer {
            from: from_2,
            to: to_storage,
            authority: owner_2,
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, storage_fee)?;
        
        // Feeling verified. Might implement this later.
        // let from_account_balance = token::accessor::amount(from)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Interaction<'info> {
    #[account(mut, has_one = owner)]
    from: CpiAccount<'info, TokenAccount>,
    #[account(mut, "from.mint == to.mint")]
    to: CpiAccount<'info, TokenAccount>,
    #[account(mut, "from.mint == to_storage_account.mint")]
    to_storage_account: CpiAccount<'info, TokenAccount>,
    #[account(signer)]
    owner: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}

#[error]
pub enum ErrorCode {
    #[msg("The derived interaction signer does not match that which was given.")]
    InvalidInteractionSigner,
}


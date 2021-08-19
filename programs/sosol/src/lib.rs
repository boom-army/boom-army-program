//! Sosol intereaction

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use std::convert::Into;

#[program]
pub mod sosol {
    use super::*;

    pub fn interaction(ctx: Context<Interaction>, interaction_fee: u64) -> ProgramResult {
        // const STORAGE_PERCENT_SPLIT: usize = 10;
        // Transfer funds to the check.
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info().clone(),
            to: ctx.accounts.to.to_account_info().clone(),
            authority: ctx.accounts.owner.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, interaction_fee)?;
        // let to = ctx.accounts.to.to_account_info().clone().key;
        // let from = ctx.accounts.from.to_account_info().clone().key;
        // let owner = ctx.accounts.owner.to_account_info().clone().key;
        // let to_storage_account = ctx.accounts.to_storage_account.to_account_info().clone().key;
        // msg!(
        //     "to: {}, from: {}",
        //     to,
        //     from,
        //     // owner,
        //     // to_storage_account,
        //     // interaction_fee,
        //     // STORAGE_PERCENT_SPLIT,
        // );
        // 1 - check balance against interaction_fee
        // let from_account_balance = token::accessor::amount(from)?;

        // 3 - split payment using storage_percent_fee

        // 2 - pay to creator_account_key
        msg!("Calling the token program to transfer tokens to the content creator...");
        // 1 . transfer token
        // let cpi_accounts = Transfer {
        //     from: ctx.accounts.from.to_account_info().clone(),
        //     to: ctx.accounts.to.to_account_info().clone(),
        //     authority: ctx.accounts.owner.clone(),
        // };
        // let cpi_program = ctx.accounts.token_program.clone();
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // token::transfer(cpi_ctx, interaction_fee)?;

        // 4 - if has storage_percent_fee -> pay to storage_account_key

        // // Safety checks.
        // apply_risk_checks(DidSwap {
        //     authority: *ctx.accounts.authority.key,
        //     given_amount: amount,
        //     min_expected_swap_amount,
        //     from_amount,
        //     to_amount,
        //     spill_amount: 0,
        //     from_mint: token::accessor::mint(from_token)?,
        //     to_mint: token::accessor::mint(to_token)?,
        //     quote_mint: match side {
        //         Side::Bid => token::accessor::mint(from_token)?,
        //         Side::Ask => token::accessor::mint(to_token)?,
        //     },
        // })?;

        // 6 - clean up / delete accounts
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Interaction<'info> {
    #[account(mut, has_one = owner)]
    from: CpiAccount<'info, TokenAccount>,
    #[account(mut, "from.mint == to.mint")]
    to: CpiAccount<'info, TokenAccount>,
    #[account("from.mint == to_storage_account.mint")]
    to_storage_account: CpiAccount<'info, TokenAccount>,
    #[account(signer)]
    owner: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}

#[error]
pub enum ErrorCode {
    #[msg("The given nonce does not create a valid program derived address.")]
    InvalidInteractionNonce,
    #[msg("The derived interaction signer does not match that which was given.")]
    InvalidInteractionSigner,
    #[msg("The given interaction has already been burned.")]
    AlreadyBurned,
}

// // Asserts the swap event is valid.
// see examples/swap/programs/swap/src/lib.rs : 177
// fn apply_risk_checks(event: DidSwap) -> Result<()> {
//     // Reject if the resulting amount is less than the client's expectation.
//     if event.to_amount < event.min_expected_swap_amount {
//         return Err(ErrorCode::SlippageExceeded.into());
//     }
//     emit!(event);
//     Ok(())
// }

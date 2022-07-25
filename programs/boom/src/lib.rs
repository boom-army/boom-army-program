//! Boom interaction

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use std::convert::Into;
use percentage::Percentage;

declare_id!("BooManQtsP9pBNudF2HDGNT9xkjL63BiWVWpfkvLkmQW");

#[program]
pub mod boom {
    use super::*;

    pub fn interaction(_ctx: Context<Interaction>, interaction_fee: u64) -> Result<()> {
        // Set storage percent split between accounts
        const STORAGE_PERCENT_SPLIT: usize = 5;

        let storage_percent = Percentage::from(STORAGE_PERCENT_SPLIT);
        let storage_fee = storage_percent.apply_to(interaction_fee);
        // Safely subtract value avoiding underflow
        let creator_fee = interaction_fee.saturating_sub(storage_fee);

        let to = _ctx.accounts.to.to_account_info().clone();
        let from = _ctx.accounts.from.to_account_info().clone();
        let owner = _ctx.accounts.owner.to_account_info().clone();

        // Check account has funds
        if _ctx.accounts.from.amount < interaction_fee {
            return Err(ContractError::NotEnoughTokens.into());
        }

        // Transfer funds to the content creator
        let cpi_accounts = Transfer {
            from,
            to,
            authority: owner,
        };
        let cpi_program = _ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, creator_fee)?;

        // Transfer funds to the content storage host
        let to_storage = _ctx.accounts.to_storage_account.to_account_info().clone();
        let from_2 = _ctx.accounts.from.to_account_info().clone();
        let owner_2 = _ctx.accounts.owner.to_account_info().clone();
        let cpi_accounts = Transfer {
            from: from_2,
            to: to_storage,
            authority: owner_2,
        };
        let cpi_program = _ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, storage_fee)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Interaction<'info> {
    #[account(mut, has_one = owner)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut, constraint = from.mint == to.mint)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut, constraint = from.mint == to_storage_account.mint)]
    pub to_storage_account: Account<'info, TokenAccount>,
    #[account(signer)]
    owner: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}

#[error_code]
pub enum ContractError {
    #[msg("The token account doesn't have enough funds to make this transaction.")]
    NotEnoughTokens
}

#[macro_export]
macro_rules! security_txt {
    ($($name:ident: $value:expr),*) => {
        #[cfg_attr(target_arch = "bpf", link_section = ".security.txt")]
        #[allow(dead_code)]
        #[no_mangle]
        pub static security_txt: &str = concat! {
            "=======BEGIN SECURITY.TXT V1=======\0",
            $(stringify!($name), "\0", $value, "\0",)*
            "=======END SECURITY.TXT V1=======\0"
        };
    };
}

security_txt! {
    name: "Boom.Army",
    project_url: "https://boom.army",
    contacts: "email:3zy3[at]protonmail.ch",
    policy: "https://github.com/boom-army/boom-army-program/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/boom-army/boom-army-program",
    auditors: "None",
    acknowledgements: "
The following hackers could've stolen all our money but didn't: 
- @r2djo
"
}

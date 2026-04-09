// ForgeMind v1 — Token-2022 Non-Transferable Receipt Token Stub
// Soulbound-style ownership with multiplier metadata for capital-weighted rewards

use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};

declare_id!("FgM1...receipt-mint-placeholder"); // Will be replaced after deployment

#[program]
pub mod receipt_token {
    use super::*;

    pub fn mint_receipt(ctx: Context<MintReceipt>, amount: u64, intelligence_multiplier: u16) -> Result<()> {
        // Mint non-transferable Token-2022 receipt
        let cpi_accounts = token_2022::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        token_2022::mint_to(cpi_ctx, amount)?;

        // TODO: Store multiplier metadata (capital_weight * compute * improvement * intelligence)
        msg!("Receipt minted for {} USDC. Intelligence multiplier: {}", amount, intelligence_multiplier);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintReceipt<'info> {
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

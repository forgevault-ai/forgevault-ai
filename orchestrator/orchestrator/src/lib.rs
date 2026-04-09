use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};

declare_id!("FgM1...ForgeMindOrchestratorV1"); // placeholder - we will deploy later

#[program]
pub mod forgevault_orchestrator {
    use super::*;

    // Initialize the vault (Genesis call)
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🔨 ForgeMind v1 orchestrator initialized");
        msg!("Genesis Architect: forgevault-ai | Seed: 25_000 USDC");
        Ok(())
    }

    // Mint soulbound receipt token with multiplier
    pub fn mint_receipt(ctx: Context<MintReceipt>, amount: u64, intelligence_mult: u16) -> Result<()> {
        let cpi_accounts = token_2022::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        token_2022::mint_to(cpi_ctx, amount)?;

        msg!("Receipt minted: {} USDC | Intelligence multiplier: {}", amount, intelligence_mult);
        Ok(())
    }

    // Basic milestone checker stub (will trigger FMD mint on 2x ATH later)
    pub fn check_milestone(ctx: Context<CheckMilestone>) -> Result<()> {
        msg!("Checking milestone... Current AUM vs 2x seed target");
        // TODO: integrate on-chain AUM oracle + performance proof
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct MintReceipt<'info> {
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct CheckMilestone {}

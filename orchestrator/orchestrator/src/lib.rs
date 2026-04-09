use anchor_lang::prelude::*;

declare_id!("FgM1...your-future-program-id-here"); // placeholder

#[program]
pub mod forgevault_orchestrator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("ForgeMind v1 orchestrator initialized - milestone detection ready");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

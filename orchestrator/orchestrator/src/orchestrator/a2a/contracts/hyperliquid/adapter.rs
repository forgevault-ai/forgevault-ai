// ForgeMind v1 — Hyperliquid Execution Adapter Stub
// 100% of all vault trading (perps & spot) routes here for signing and submission

use anchor_lang::prelude::*;

#[derive(Debug)]
pub struct TradeSignal {
    pub asset: String,      // e.g. "BTC", "ETH"
    pub side: String,       // "long" | "short"
    pub size: u64,
    pub leverage: u8,
    pub entry_price: Option<u64>,
    pub stop_loss: Option<u64>,
    pub take_profit: Option<u64>,
}

#[program]
pub mod hyperliquid_adapter {
    use super::*;

    pub fn execute_signal(ctx: Context<ExecuteSignal>, signal: TradeSignal) -> Result<()> {
        // In v1 this will call Hyperliquid API (REST or WebSocket) via off-chain signer or CPI
        msg!("🔥 Executing on Hyperliquid: {} {} size={} leverage={}x", 
             signal.side.to_uppercase(), signal.asset, signal.size, signal.leverage);

        // TODO: Integrate real Hyperliquid signing + submission
        // For now we log + emit event for shadow execution testing

        emit!(ExecutionEvent {
            signal: format!("{:?}", signal),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct ExecutionEvent {
    pub signal: String,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct ExecuteSignal {}

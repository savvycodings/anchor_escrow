use anchor_lang::prelude::*;

declare_id!("2MJjQF68MmUHh8DPmrS746uXCbwdNGgpQuKBVf2QwDkJ");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

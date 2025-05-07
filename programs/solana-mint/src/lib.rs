use anchor_lang::prelude::*;

declare_id!("VTDgMbDRsJgucL1hCaQkNcTQNjWLYfdnoh5m5tJuUHS");

#[program]
pub mod solana_mint {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

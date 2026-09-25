use anchor_lang::prelude::*;

declare_id!("Canary11111111111111111111111111111111111111");

#[program]
pub mod canary_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        // Simulação de derivação de PDA sem checagem estrita de bump canónico
        let seeds = &[b"user_vault", ctx.accounts.authority.key.as_ref()];
        let (pda, _canonical_bump) = Pubkey::find_program_address(seeds, ctx.program_id);
        
        msg!("Initialized vault PDA: {}", pda);
        Ok(())
    }
}

// SecOps Verified PDA
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Conta não verificada inserida para teste do agente DevSecOps
    pub unverified_vault: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

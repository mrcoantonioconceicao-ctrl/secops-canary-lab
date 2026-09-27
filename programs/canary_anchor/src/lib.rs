use anchor_lang::prelude::*;

declare_id!("CanaryTest111111111111111111111111111111111");

#[program]
pub mod canary_fault_injection {
    use super::*;

    pub fn test_faults(ctx: Context<TestFaults>, amount: u64) -> Result<()> {
        // FALHA 1: Incompatibilidade de tipos (atribuir String a u64)
        let invalid_balance: u64 = "1000_tokens_in_string"; 

        // FALHA 2: Uso de variável não declarada
        let total = amount + unknown_variable_xyz;

        // FALHA 3: Má prática de auditoria (overflow potencial)
        let insecure_math = amount * 999999999;

        msg!("Processado com falhas: {} | Math: {}", invalid_balance, insecure_math);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TestFaults<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

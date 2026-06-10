use anchor_lang::prelude::*;

#[account]
pub struct ReputationSBT {
    pub owner: Pubkey,
    pub tasks_completed: u32,
    pub score: u64,
    pub issued_at: i64,
}

#[derive(Accounts)]
pub struct MintReputation<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 80,
        seeds = [b"reputation", contributor.key().as_ref()],
        bump
    )]
    pub reputation: Account<'info, ReputationSBT>,
    /// CHECK: contributor receiving reputation
    pub contributor: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn mint_reputation(ctx: Context<MintReputation>) -> Result<()> {
    let rep = &mut ctx.accounts.reputation;
    rep.owner            = ctx.accounts.contributor.key();
    rep.tasks_completed += 1;
    rep.score           += 100; // base score per task
    rep.issued_at        = Clock::get()?.unix_timestamp;
    Ok(())
}

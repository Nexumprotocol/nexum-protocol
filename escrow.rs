use anchor_lang::prelude::*;

#[account]
pub struct TaskEscrow {
    pub task_id: u64,
    pub poster: Pubkey,
    pub contributor: Option<Pubkey>,
    pub reward: u64,
    pub deadline: i64,
    pub status: TaskStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Open,
    InProgress,
    Submitted,
    Completed,
    Disputed,
}

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CreateTask<'info> {
    #[account(
        init,
        payer = poster,
        space = 8 + 200,
        seeds = [b"task", poster.key().as_ref(), &task_id.to_le_bytes()],
        bump
    )]
    pub task_escrow: Account<'info, TaskEscrow>,
    #[account(mut)]
    pub poster: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptTask<'info> {
    #[account(mut)]
    pub task_escrow: Account<'info, TaskEscrow>,
    pub contributor: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveTask<'info> {
    #[account(mut)]
    pub task_escrow: Account<'info, TaskEscrow>,
    #[account(mut)]
    pub poster: Signer<'info>,
    /// CHECK: contributor wallet for payout
    #[account(mut)]
    pub contributor: AccountInfo<'info>,
}

pub fn create_task(ctx: Context<CreateTask>, task_id: u64, reward: u64, deadline: i64) -> Result<()> {
    let escrow = &mut ctx.accounts.task_escrow;
    escrow.task_id    = task_id;
    escrow.poster     = ctx.accounts.poster.key();
    escrow.contributor= None;
    escrow.reward     = reward;
    escrow.deadline   = deadline;
    escrow.status     = TaskStatus::Open;
    escrow.bump       = ctx.bumps.task_escrow;
    Ok(())
}

pub fn accept_task(ctx: Context<AcceptTask>, _task_id: u64) -> Result<()> {
    let escrow = &mut ctx.accounts.task_escrow;
    require!(escrow.status == TaskStatus::Open, NexumError::TaskNotOpen);
    escrow.contributor = Some(ctx.accounts.contributor.key());
    escrow.status      = TaskStatus::InProgress;
    Ok(())
}

pub fn approve_task(ctx: Context<ApproveTask>, _task_id: u64) -> Result<()> {
    let escrow = &mut ctx.accounts.task_escrow;
    require!(escrow.status == TaskStatus::Submitted, NexumError::TaskNotSubmitted);
    escrow.status = TaskStatus::Completed;
    // Transfer reward to contributor — full implementation in v0.1.0
    Ok(())
}

#[error_code]
pub enum NexumError {
    #[msg("Task is not open for acceptance")]
    TaskNotOpen,
    #[msg("Task has not been submitted yet")]
    TaskNotSubmitted,
}

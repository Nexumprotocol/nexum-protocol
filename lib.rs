use anchor_lang::prelude::*;

declare_id!("NXMxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

pub mod escrow;
pub mod reputation;

#[program]
pub mod nexum {
    use super::*;

    pub fn create_task(
        ctx: Context<escrow::CreateTask>,
        task_id: u64,
        reward: u64,
        deadline: i64,
    ) -> Result<()> {
        escrow::create_task(ctx, task_id, reward, deadline)
    }

    pub fn accept_task(ctx: Context<escrow::AcceptTask>, task_id: u64) -> Result<()> {
        escrow::accept_task(ctx, task_id)
    }

    pub fn approve_task(ctx: Context<escrow::ApproveTask>, task_id: u64) -> Result<()> {
        escrow::approve_task(ctx, task_id)
    }

    pub fn mint_reputation(ctx: Context<reputation::MintReputation>) -> Result<()> {
        reputation::mint_reputation(ctx)
    }
}

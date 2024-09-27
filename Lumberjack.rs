use anchor_lang::prelude::*;

declare_id!("4ayoaXt8odfcNneUuZPeiYQfSCDH4XyG8iEkaPK2pUHx");

const MAX_ENERGY: u64 = 5;
const TIME_TO_REFILL_ENERGY: i64 = 30;

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough energy")]
    NotEnoughEnergy,
}

#[program]
pub mod lumberjack {
    use super::*;

    pub fn init_player(ctx: Context<InitPlayer>) -> Result<()> {
        ctx.accounts.player.energy = MAX_ENERGY;
        ctx.accounts.player.last_login = Clock::get()?.unix_timestamp;
        Ok(())
    }

}

pub fn chop_tree(ctx: Context<ChopTree>) -> Result<()> {
    let account = &mut ctx.accounts;
    update_energy(account)?;

    if ctx.accounts.player.energy == 0 {
        return err!(ErrorCode::NotEnoughEnergy);
    }

    ctx.accounts.player.wood = ctx.accounts.player.wood + 1;
    ctx.accounts.player.energy = ctx.accounts.player.energy -1;

    Ok(())
}

pub fn update_energy(ctx:)

#[derive(Accounts)]
pub struct ChopTree<'info> {
    #[account(
        mut,
        seeds= [b"player", signer.key.as_ref()],
        bump,
    )]
    pub player: Account<'info, PlayerData>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitPlayer<'info> {
    #[account( 
        init, 
        payer = signer,
        space = 1000,
        seeds = [b"player", signer.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, PlayerData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerData {
    pub name: String,
    pub level: u8,
    pub xp: u64,
    pub wood: u64,
    pub energy: u64,
    pub last_login: i64,
}

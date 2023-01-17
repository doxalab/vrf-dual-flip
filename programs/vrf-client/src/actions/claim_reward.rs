use crate::*;
const TEST_GAME_SEED: &[u8] = b"10";
#[derive(Accounts)]
#[instruction(game_id: String)] // rpc parameters hint
pub struct ClaimReward<'info> {
    #[account(
        mut,
        seeds = [
            GAME_SEED,
            game_id.as_ref(),
            owner.key().as_ref(),
        ],
        bump,
    )]
    pub game: Account<'info, GameState>,
    #[account(
        mut,
        seeds = [
            STATE_SEED,
            vrf.key().as_ref(),
        ],
        bump = state.load()?.bump,
        has_one = vrf @ VrfClientErrorCode::InvalidVrfAccount
    )]
    pub state: AccountLoader<'info, VrfClientState>,
    #[account(mut)]
    pub vrf: AccountLoader<'info, VrfAccountData>,
    #[account(
        mut,
        seeds = [
            ESCROW_SEED,
            game_id.as_ref(),
            owner.key().as_ref(),
        ],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ClaimRewardParams {}

impl ClaimReward<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _game_id: String, game_bump: u8) -> Result<()> {
        let state = ctx.accounts.state.load_mut()?;
        let game = &mut ctx.accounts.game;
        let result : u64 = (state.result %2) as u64;
        let owner = ctx.accounts.owner.key();
        if (owner == game.owner && result == game.owner_choice) || (owner != game.owner && result != game.owner_choice) {
            msg!("You are winner");
            // Transferring the winning amount
            let seeds = &[GAME_SEED, TEST_GAME_SEED, owner.as_ref(), &[game_bump]];
            let signer = &[&seeds[..]];
            let cpi_accounts = Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.owner_token_account.to_account_info(),
                authority: game.to_account_info(),
            };
            let token_program = ctx.accounts.token_program.to_account_info();
            let transfer_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer);
            token::transfer(transfer_ctx, game.bet_amount)?;
        } else {
            msg!("You are loser, cannot claim");
        }

        Ok(())
    }
}

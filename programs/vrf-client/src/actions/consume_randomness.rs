use crate::*;
const TEST_GAME_SEED: &[u8] = b"10";
#[derive(Accounts)]
#[instruction(params: ConsumeRandomnessParams)] // rpc parameters hint
pub struct ConsumeRandomness<'info> {
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
    pub vrf: AccountLoader<'info, VrfAccountData>,
    #[account(
        mut,
        seeds = [
            GAME_SEED,
            TEST_GAME_SEED,
            owner.key().as_ref(),
        ],
        bump,
    )]
    pub game: Account<'info, GameState>,
    #[account(
        mut,
        seeds = [
            ESCROW_SEED,
            TEST_GAME_SEED,
            owner.key().as_ref(),
        ],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    pub token_program: Program<'info, Token>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ConsumeRandomnessParams {}

impl ConsumeRandomness<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, game_bump: u8) -> Result<()> {
        let vrf = ctx.accounts.vrf.load()?;
        let result_buffer = vrf.get_result()?;
        if result_buffer == [0u8; 32] {
            msg!("vrf buffer empty");
            return Ok(());
        }

        let state = &mut ctx.accounts.state.load_mut()?;
        let max_result = state.max_result;
        if result_buffer == state.result_buffer {
            msg!("result_buffer unchanged");
            return Ok(());
        }

        msg!("Result buffer is {:?}", result_buffer);
        let value: &[u128] = bytemuck::cast_slice(&result_buffer[..]);
        msg!("u128 buffer {:?}", value);
        let result = value[0] % max_result as u128 + 1;
        msg!("Current VRF Value [1 - {}) = {}!", max_result, result);

        let game = &mut ctx.accounts.game;
        let owner = ctx.accounts.owner.key();
        if result % 2 == game.owner_choice.into() {
            msg!("You are winner");
            game.winner = Some(game.owner);
            game.result = ((result as u64) % 2).into();
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
            msg!("You are loser");
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
        }

        if state.result != result {
            state.result_buffer = result_buffer;
            state.result = result;
            state.timestamp = clock::Clock::get().unwrap().unix_timestamp;

            emit!(VrfClientUpdated {
                vrf_client: ctx.accounts.state.key(),
                max_result: state.max_result,
                result: state.result,
                result_buffer: result_buffer,
                timestamp: state.timestamp,
            });
        }

        Ok(())
    }
}

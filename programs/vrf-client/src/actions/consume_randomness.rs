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
    /// CHECK:
    pub owner: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ConsumeRandomnessParams {}

impl ConsumeRandomness<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: &ConsumeRandomnessParams) -> Result<()> {
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
        if result % 2 == game.owner_choice.into() {
            msg!("You are winner");
            game.winner = Some(game.owner);
            game.result = ((result as u64) % 2).into();
        } else {
            msg!("You are loser");
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

use crate::*;

#[derive(Accounts)]
#[instruction(params: RequestRandomnessParams)] // rpc parameters hint
pub struct RequestRandomness<'info> {
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
    #[account(
        mut,
        seeds = [
            GAME_SEED,
            params.game_id.as_ref(),
            owner.key().as_ref(),
        ],
        bump,
    )]
    pub game: Box<Account<'info, GameState>>,

    // SWITCHBOARD ACCOUNTS
    #[account(mut,
        has_one = escrow
    )]
    pub vrf: AccountLoader<'info, VrfAccountData>,
    #[account(mut,
        has_one = data_buffer
    )]
    pub oracle_queue: AccountLoader<'info, OracleQueueAccountData>,
    /// CHECK:
    #[account(mut,
        constraint =
            oracle_queue.load()?.authority == queue_authority.key()
    )]
    pub queue_authority: UncheckedAccount<'info>,
    /// CHECK
    #[account(mut)]
    pub data_buffer: AccountInfo<'info>,
    #[account(mut)]
    pub permission: AccountLoader<'info, PermissionAccountData>,
    #[account(mut,
        constraint =
            escrow.owner == program_state.key()
            && escrow.mint == program_state.load()?.token_mint
    )]
    pub escrow: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub program_state: AccountLoader<'info, SbState>,
    /// CHECK:
    #[account(
        address = *vrf.to_account_info().owner,
        constraint = switchboard_program.executable == true
    )]
    pub switchboard_program: AccountInfo<'info>,

    // PAYER ACCOUNTS
    #[account(mut,
        constraint =
            payer_wallet.owner == payer_authority.key()
            && escrow.mint == program_state.load()?.token_mint
    )]
    pub payer_wallet: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    #[account(signer)]
    pub payer_authority: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [
            ESCROW_SEED,
            params.game_id.as_ref(),
            owner.key().as_ref(),
        ],
        bump,
    )] 
    pub escrow_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub joinee: Signer<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,

    // SYSTEM ACCOUNTS
    /// CHECK:
    #[account(address = solana_program::sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RequestRandomnessParams {
    pub permission_bump: u8,
    pub switchboard_state_bump: u8,
    pub game_id: String
}

impl RequestRandomness<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &RequestRandomnessParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: &RequestRandomnessParams) -> Result<()> {
        let client_state = ctx.accounts.state.load()?;
        let bump = client_state.bump.clone();
        let max_result = client_state.max_result;
        drop(client_state);

        let switchboard_program = ctx.accounts.switchboard_program.to_account_info();

        let vrf_request_randomness = VrfRequestRandomness {
            authority: ctx.accounts.state.to_account_info(),
            vrf: ctx.accounts.vrf.to_account_info(),
            oracle_queue: ctx.accounts.oracle_queue.to_account_info(),
            queue_authority: ctx.accounts.queue_authority.to_account_info(),
            data_buffer: ctx.accounts.data_buffer.to_account_info(),
            permission: ctx.accounts.permission.to_account_info(),
            escrow: *ctx.accounts.escrow.clone(),
            payer_wallet: *ctx.accounts.payer_wallet.clone(),
            payer_authority: ctx.accounts.payer_authority.to_account_info(),
            recent_blockhashes: ctx.accounts.recent_blockhashes.to_account_info(),
            program_state: ctx.accounts.program_state.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let vrf_key = ctx.accounts.vrf.key();
        let state_seeds: &[&[&[u8]]] = &[&[&STATE_SEED, vrf_key.as_ref(), &[bump]]];

        msg!("requesting randomness");
        vrf_request_randomness.invoke_signed(
            switchboard_program,
            params.switchboard_state_bump,
            params.permission_bump,
            state_seeds,
        )?;

        let mut client_state = ctx.accounts.state.load_mut()?;
        client_state.result = 0;

        let game = &mut ctx.accounts.game; 
        game.joinee = Some(ctx.accounts.joinee.key());
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.joinee.to_account_info()
        };
        let token_program = ctx.accounts.token_program.to_account_info();
        let transfer_ctx = CpiContext::new(token_program, cpi_accounts);
        token::transfer(
            transfer_ctx,
            game.bet_amount 
        )?;

        emit!(RandomnessRequested {
            vrf_client: ctx.accounts.state.key(),
            max_result: max_result,
            timestamp: clock::Clock::get().unwrap().unix_timestamp
        });

        msg!("randomness requested successfully");
        Ok(())
    }
}

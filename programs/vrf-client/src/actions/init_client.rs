use crate::*;

#[derive(Accounts)]
#[instruction(params: InitClientParams)]
pub struct InitClient<'info> {
    #[account(
        init,
        seeds = [
            GAME_SEED,
            params.game_id.as_ref(),
            payer.key().as_ref(),
        ],
        payer = payer,
        space = 8 + std::mem::size_of::<GameState>(),
        bump,
    )]
    pub game: Account<'info, GameState>,
    #[account(
        init,
        seeds = [
            ESCROW_SEED,
            params.game_id.as_ref(),
            payer.key().as_ref(),
        ],
        payer = payer,
        bump,
        token::mint = token_mint,
        token::authority = game
    )] 
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [
            STATE_SEED,
            vrf.key().as_ref()
        ],
        payer = payer,
        space = 8 + std::mem::size_of::<VrfClientState>(),
        bump,
    )]
    pub state: AccountLoader<'info, VrfClientState>,
    #[account(
        constraint = vrf.load()?.authority == state.key() @ VrfClientErrorCode::InvalidVrfAuthorityError
    )]
    pub vrf: AccountLoader<'info, VrfAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitClientParams {
    pub max_result: u64,
    pub game_id: String,
    pub choice: u64,
    pub bet_amount: u64,
}

impl InitClient<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &InitClientParams) -> Result<()> {
        msg!("init_client validate");
        if params.max_result > 1337 {
            return Err(error!(VrfClientErrorCode::MaxResultExceedsMaximum));
        }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: &InitClientParams) -> Result<()> {
        msg!("init_client actuate");

        let mut state = ctx.accounts.state.load_init()?;
        *state = VrfClientState::default();
        state.bump = ctx.bumps.get("state").unwrap().clone();
        state.vrf = ctx.accounts.vrf.key();

        let game = &mut ctx.accounts.game;
        game.owner = ctx.accounts.payer.key();
        game.owner_choice = params.choice;
        game.joinee = Option::None;
        game.winner = Option::None;
        game.bet_amount = params.bet_amount;
        game.result = Option::None;
        game.room_creation_time = Clock::get()?.unix_timestamp;

        if params.max_result == 0 {
            state.max_result = 1337;
        } else {
            state.max_result = params.max_result;
        }
        
        // Transferring the bet amount
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info()
        };
        let token_program = ctx.accounts.token_program.to_account_info();
        let transfer_ctx = CpiContext::new(token_program, cpi_accounts);
        token::transfer(
            transfer_ctx,
            params.bet_amount 
        )?;

        emit!(VrfClientCreated {
            vrf_client: ctx.accounts.state.key(),
            max_result: params.max_result,
            timestamp: clock::Clock::get().unwrap().unix_timestamp
        });

        Ok(())
    }
}

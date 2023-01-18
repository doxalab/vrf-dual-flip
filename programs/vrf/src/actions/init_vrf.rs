use crate::*;

#[derive(Accounts)]
#[instruction(params: InitVrfParams)]
pub struct InitVrf<'info> {
    #[account(
        init,
        seeds = [
            VRF_SEED,
            payer.key().as_ref()
        ],
        payer = payer,
        space = 8 + std::mem::size_of::<VRFKey>(),
        bump,
    )]
    pub state: Account<'info, VRFKey>,
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
    pub vrf_state: AccountLoader<'info, VrfClientState>,
    pub vrf: AccountLoader<'info, VrfAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitVrfParams {
    
}

impl InitVrf<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &InitVrfParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: &InitVrfParams) -> Result<()> {
        msg!("init_vrf actuate");

        let state = &mut ctx.accounts.state;
        state.key = ctx.accounts.vrf.key();

        Ok(())
    }
}

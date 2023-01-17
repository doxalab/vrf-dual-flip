use anchor_lang::prelude::*;

pub mod actions;
pub use actions::*;

pub use anchor_lang::solana_program::clock;
pub use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
pub use switchboard_v2::{
    OracleQueueAccountData, PermissionAccountData, SbState, VrfAccountData, VrfRequestRandomness,
};

declare_id!("21yN3N5jquXxbPDSRZiqANhZsDQUAgQvz5vNysHXL3ut");

#[program]
pub mod vrf {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn init_client(ctx: Context<InitClient>, params: InitClientParams) -> Result<()> {
        InitClient::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn init_vrf(ctx: Context<InitVrf>, params: InitVrfParams) -> Result<()> {
        InitVrf::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn request_randomness(
        ctx: Context<RequestRandomness>,
        params: RequestRandomnessParams,
    ) -> Result<()> {
        RequestRandomness::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn consume_randomness(
        ctx: Context<ConsumeRandomness>,
        params: ConsumeRandomnessParams,
    ) -> Result<()> {
        ConsumeRandomness::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        game_id: String,
        game_bump: u8,
    ) -> Result<()> {
        ClaimReward::actuate(ctx, game_id, game_bump)
    }
}

const STATE_SEED: &[u8] = b"CLIENTSEED";
const VRF_SEED: &[u8] = b"VRF";
const GAME_SEED: &[u8] = b"GAME";
const ESCROW_SEED: &[u8] = b"ESCROW";

#[repr(packed)]
#[account(zero_copy)]
#[derive(Default)]
pub struct VrfClientState {
    pub bump: u8,
    pub max_result: u64,
    pub result_buffer: [u8; 32],
    pub result: u128,
    pub timestamp: i64,
    pub vrf: Pubkey,
}

#[account]
pub struct GameState {
    pub owner: Pubkey,
    pub owner_choice: u64,
    pub joinee: Option<Pubkey>,
    pub winner: Option<Pubkey>,
    pub bet_amount: u64,
    pub result: Option<u64>,
    pub room_creation_time: i64
}

#[account]
pub struct VRFKey {
    pub key: Pubkey,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum VrfClientErrorCode {
    #[msg("Switchboard VRF Account's authority should be set to the client's state pubkey")]
    InvalidVrfAuthorityError,
    #[msg("The max result must not exceed u64")]
    MaxResultExceedsMaximum,
    #[msg("Invalid VRF account provided.")]
    InvalidVrfAccount,
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
}

#[event]
pub struct VrfClientCreated {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub timestamp: i64,
}

#[event]
pub struct RandomnessRequested {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub timestamp: i64,
}

#[event]
pub struct VrfClientUpdated {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub result_buffer: [u8; 32],
    pub result: u128,
    pub timestamp: i64,
}

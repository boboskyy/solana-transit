use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DeliveryProfile {
    pub courier: Pubkey,
    #[max_len(512)]
    pub delivery_secret: String,
    pub delivery_reward_lamports: u64,
}

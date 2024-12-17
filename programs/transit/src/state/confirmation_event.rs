use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ConfirmationEvent {
    pub courier: Pubkey,
    pub received_at: Option<u64>,
    pub delivered_at: Option<u64>,
}

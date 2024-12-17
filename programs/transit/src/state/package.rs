use anchor_lang::prelude::*;

use super::{ConfirmationEvent, DeliveryProfile};

#[account]
#[derive(InitSpace)]
pub struct Package {
    #[max_len(128)]
    pub package_id: String,
    #[max_len(256)]
    pub public_package_info: String,
    #[max_len(5)]
    pub couriers: Vec<DeliveryProfile>,
    pub current_holder: Pubkey,
    #[max_len(5)]
    pub confirmations: Vec<ConfirmationEvent>,
}

impl Package {
    pub fn get_next_courier(&self, current: Pubkey) -> Option<Pubkey> {
        let current_index = self.couriers.iter().position(|c| c.courier == current)?;
        self.couriers.get(current_index + 1).map(|c| c.courier)
    }
}

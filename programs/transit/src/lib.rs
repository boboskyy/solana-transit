pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9mwAa7UueUoYthqYgMDDchzktvzWZHHmsccsE8KKXBt9");

#[program]
pub mod transit {

    use super::*;

    pub fn initialize_package(
        context: Context<InitializePackage>,
        package_id: String,
        public_package_info: String,
        couriers: Vec<state::DeliveryProfile>,
    ) -> Result<()> {
        instructions::initialize_package(context, package_id, public_package_info, couriers)
    }

    pub fn confirm_pickup(
        context: Context<ConfirmPickup>,
        package_id: String,
        public_package_info: String,
    ) -> Result<()> {
        instructions::confirm_pickup(context, package_id, public_package_info)
    }

    pub fn confirm_delivery(
        context: Context<ConfirmDelivery>,
        package_id: String,
        public_package_info: String,
    ) -> Result<()> {
        instructions::confirm_delivery(context, package_id, public_package_info)
    }
}

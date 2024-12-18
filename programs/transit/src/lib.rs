pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DLzH1giBHuB1YDDfbiimgmcEUSAwWT8vK7BdHK8idauj");

/*
  _________      .__                         /\/\     ___________                           ._____________
 /   _____/ ____ |  | _____    ____ _____    \ \ \    \__    ___/___________    ____   _____|__\__    ___/
 \_____  \ /  _ \|  | \__  \  /    \\__  \    \ \ \     |    |  \_  __ \__  \  /    \ /  ___/  | |    |
 /        (  <_> )  |__/ __ \|   |  \/ __ \_   \ \ \    |    |   |  | \// __ \|   |  \\___ \|  | |    |
/_______  /\____/|____(____  /___|  (____  /    \ \ \   |____|   |__|  (____  /___|  /____  >__| |____|
        \/                 \/     \/     \/      \/\/                       \/     \/     \/
*/

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

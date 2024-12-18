use crate::constants::{self};
use crate::error::TransitError;
use crate::{DeliveryProfile, Package};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(package_id: String, public_package_info: String)]
pub struct InitializePackage<'info> {
    #[account(init, payer = creator, space = 8 + Package::INIT_SPACE, seeds = [package_id.as_bytes(), public_package_info.as_bytes()], bump)]
    pub package: Account<'info, Package>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_package(
    context: Context<InitializePackage>,
    package_id: String,
    public_package_info: String,
    couriers: Vec<DeliveryProfile>,
) -> Result<()> {
    let package = &mut context.accounts.package;

    require!(
        package_id.len() > 0 && package_id.len() <= 128,
        TransitError::InvalidPackageId
    );

    require!(
        package_id.chars().all(|c| c.is_alphanumeric() || c == '_'),
        TransitError::InvalidPackageId
    );

    require!(
        public_package_info.len() <= 256,
        TransitError::InvalidPublicPackageInfo
    );

    require!(
        couriers.len() <= constants::COURIER_LIMIT,
        TransitError::TooManyCouriers
    );

    package.package_id = package_id;
    package.public_package_info = public_package_info;
    package.couriers.clone_from(&couriers);
    package.current_holder = couriers[0].courier;
    package.confirmations = Vec::new();

    Ok(())
}

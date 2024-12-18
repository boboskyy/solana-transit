use crate::error::TransitError;
use crate::state::Package;
use anchor_lang::prelude::*;

pub fn confirm_delivery(
    ctx: Context<ConfirmDelivery>,
    _package_id: String,
    _public_package_info: String,
) -> Result<()> {
    let package = &mut ctx.accounts.package;
    let courier = &mut ctx.accounts.courier;

    let courier_pubkey = courier.key();

    require!(
        package.current_holder == courier_pubkey,
        TransitError::UnauthorizedCourier
    );

    let clock: Clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let confirmation = package
        .confirmations
        .iter_mut()
        .find(|c| c.courier == courier_pubkey)
        .ok_or(TransitError::NoConfirmationFound)?;

    confirmation.delivered_at = Some(current_timestamp as u64);

    let next_courier_exists = package.get_next_courier(courier_pubkey).is_some();

    if !next_courier_exists {
        let final_courier_profile = package
            .couriers
            .iter()
            .find(|p| p.courier == courier_pubkey)
            .ok_or(TransitError::CourierNotFound)?;

        let reward_lamports = final_courier_profile.delivery_reward_lamports;

        require!(
            **package.to_account_info().lamports.borrow() >= reward_lamports,
            TransitError::InsufficientLamports
        );

        **package.to_account_info().lamports.borrow_mut() -= reward_lamports;
        **courier.to_account_info().lamports.borrow_mut() += reward_lamports;

        msg!(
            "Paid to final courier: {} amount: {}",
            final_courier_profile.courier,
            reward_lamports
        );
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(package_id: String, public_package_info: String)]
pub struct ConfirmDelivery<'info> {
    #[account(mut,
        seeds = [ package_id.as_bytes(), public_package_info.as_bytes() ],
        bump
    )]
    pub package: Account<'info, Package>,

    #[account(mut)]
    pub courier: Signer<'info>,
    pub system_program: Program<'info, System>,
}

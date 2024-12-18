use crate::{
    error::TransitError,
    state::{ConfirmationEvent, Package},
};
use anchor_lang::prelude::*;

pub fn confirm_pickup(
    ctx: Context<ConfirmPickup>,
    _package_id: String,
    _public_package_info: String,
) -> Result<()> {
    let package = &mut ctx.accounts.package;
    let courier = ctx.accounts.courier.key();

    require!(
        package.current_holder == courier
            || (package.get_next_courier(package.current_holder) == Some(courier)
                && package.confirmations.last().unwrap().delivered_at.is_some()),
        TransitError::UnauthorizedCourier
    );

    let clock: Clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let is_first_courier = package.couriers[0].courier == courier;

    let confirmation = if let Some(c) = package
        .confirmations
        .iter_mut()
        .find(|c| c.courier == courier)
    {
        c
    } else {
        package.confirmations.push(ConfirmationEvent {
            courier,
            received_at: None,
            delivered_at: None,
        });
        package
            .confirmations
            .last_mut()
            .ok_or(TransitError::InternalError)?
    };
    confirmation.received_at = Some(current_timestamp as u64);

    if !is_first_courier {
        if let Some(prev_confirmation) = package
            .confirmations
            .iter()
            .rev()
            .find(|c| c.delivered_at.is_some() && c.courier != courier)
        {
            let previous_courier_profile = package
                .couriers
                .iter()
                .find(|p| p.courier == prev_confirmation.courier)
                .ok_or(TransitError::CourierNotFound)?;

            let reward_lamports = previous_courier_profile.delivery_reward_lamports;

            let receiver = &ctx.accounts.receiver;

            if receiver.key() != previous_courier_profile.courier {
                return Err(TransitError::InvalidRewardAccount.into());
            }

            require!(
                **package.to_account_info().lamports.borrow() >= reward_lamports,
                TransitError::InsufficientLamports
            );

            **package.to_account_info().lamports.borrow_mut() -= reward_lamports;
            **receiver.lamports.borrow_mut() += reward_lamports;

            msg!(
                "Paid to: {} amount: {}",
                previous_courier_profile.courier,
                reward_lamports
            );
        }

        if let Some(next_courier) = package.get_next_courier(package.current_holder) {
            package.current_holder = next_courier;
        }
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(package_id: String, public_package_info: String)]
pub struct ConfirmPickup<'info> {
    #[account(mut,
        seeds = [ package_id.as_bytes(), public_package_info.as_bytes() ],
        bump
    )]
    pub package: Account<'info, Package>,

    /// CHECK
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub courier: Signer<'info>,
}

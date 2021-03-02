use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::RecurringPaymentsError;

#[derive(Debug, PartialEq)]
pub enum RecurringPaymentsInstruction {
    /// Starts the trade by creating and populating an RecurringPayments account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the RecurringPayments
    /// 1. `[writable]` The subscription account, it will hold all necessary info about the subscription.
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The RecurringPayments account, it will hold all necessary info about the trade.
    /// 4. `[]` The clock sysvar
    CreateSubscriptionPlan {
        /// nonce used to create valid program address
        nonce: u8,
        /// Length of the subscription (1 Month ususally) in days
        subscription_timeframe: u64,
        /// max amount that can be withdrawn in one timeframe
        max_amount: u64,
    },
    /// Starts the trade by creating and populating an RecurringPayments account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the RecurringPayments
    /// 1. `[writable]` The subscription account, it will hold all necessary info about the subscription.
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The RecurringPayments account, it will hold all necessary info about the trade.
    /// 4. `[]` The clock sysvar
    CreateSubscription {
        /// Length of the subscription (1 Month ususally) in days
        subscription_timeframe: u64,
        /// max amount that can be withdrawn in one timeframe
        max_amount: u64,
    },

    Claim {},
}

impl RecurringPaymentsInstruction {
    /// Unpacks a byte buffer
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, src) = input.split_first().ok_or(RecurringPaymentsError::InvalidInstruction)?;

        Ok(match tag {
            0 => {
                let (&nonce, src) = src.split_first().ok_or(RecurringPaymentsError::InvalidInstruction)?;
                let (subscription_timeframe, src) = Self::unpack_u64(src)?;
                let (max_amount, _src) = Self::unpack_u64(src)?;

                Self::CreateSubscriptionPlan {
                    nonce,
                    subscription_timeframe,
                    max_amount,
                }
            }
            1 => {
                let (subscription_timeframe, src) = Self::unpack_u64(src)?;
                let (max_amount, _src) = Self::unpack_u64(src)?;

                Self::CreateSubscription {
                    subscription_timeframe,
                    max_amount,
                }
            }
            2 => Self::Claim {},
            _ => return Err(RecurringPaymentsError::InvalidInstruction.into()),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, src) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(RecurringPaymentsError::InvalidInstruction)?;
            Ok((amount, src))
        } else {
            Err(RecurringPaymentsError::InvalidInstruction.into())
        }
    }
}

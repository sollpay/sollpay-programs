use crate::error::RecurringPaymentsError;
use crate::instruction::RecurringPaymentsInstruction;
use crate::state::{Subscription, SubscriptionPlan};
use num_traits::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::{Clock, UnixTimestamp},
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    msg,
    program_error::{PrintProgramError, ProgramError},
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::Sysvar,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Processes an RecurringPaymentsInstruction
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = RecurringPaymentsInstruction::unpack(instruction_data)?;

        msg!("Instruction: {:?}", instruction);
        match instruction {
            RecurringPaymentsInstruction::CreateSubscriptionPlan {
                subscription_timeframe,
                max_amount,
            } => Self::process_create_subscription_plan(accounts, subscription_timeframe, max_amount, program_id),
            RecurringPaymentsInstruction::CreateSubscription {
                subscription_timeframe,
                max_amount,
            } => Self::process_create_subscription(accounts, subscription_timeframe, max_amount, program_id),
            RecurringPaymentsInstruction::Claim {} => Self::process_claim(accounts, program_id),
        }
    }

    fn process_create_subscription_plan(
        accounts: &[AccountInfo],
        subscription_timeframe: u64,
        max_amount: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let subscription_plan_account = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;

        // TODO: Checks

        pack_subscription_plan(
            subscription_plan_account,
            *owner.key,
            *token_mint.key,
            subscription_timeframe,
            max_amount,
        )?;

        Ok(())
    }

    fn process_create_subscription(
        accounts: &[AccountInfo],
        subscription_timeframe: u64,
        max_amount: u64,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let subscription_account = next_account_info(account_info_iter)?;
        let subscription_plan_account = next_account_info(account_info_iter)?;
        let token_address = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let clock_sysvar_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(clock_sysvar_info)?;
        let cycle_start = clock.unix_timestamp - 24 * 60 * 60; // TODO: temp

        let subscription_plan = SubscriptionPlan::unpack(&subscription_plan_account.data.borrow())?;

        // TODO: 8?
        if subscription_plan.subscription_timeframe >> 8 != subscription_timeframe {
            return Err(RecurringPaymentsError::InvalidSubscriptionTimeframe.into());
        }

        // TODO: 8?
        if subscription_plan.max_amount >> 8 != max_amount {
            return Err(RecurringPaymentsError::InvalidMaxAmount.into());
        }

        // TODO: Checks

        pack_subscription(
            subscription_account,
            *subscription_plan_account.key,
            *token_address.key,
            *owner.key,
            cycle_start,
            subscription_timeframe,
            max_amount,
        )?;

        Ok(())
    }

    fn process_claim(accounts: &[AccountInfo], _program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let subscription_account = next_account_info(account_info_iter)?;

        let subscription = Subscription::unpack(&subscription_account.data.borrow())?;

        Ok(())
    }
}

impl PrintProgramError for RecurringPaymentsError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            RecurringPaymentsError::InvalidInstruction => msg!("Error: Invalid instruction"),
            RecurringPaymentsError::InvalidMaxAmount => msg!("Error: Invalid max amount"),
            RecurringPaymentsError::InvalidSubscriptionTimeframe => msg!("Error: Invalid subscription timeframe"),
        }
    }
}

fn pack_subscription_plan(
    subscription_plan_account: &AccountInfo,
    owner: Pubkey,
    token_mint: Pubkey,
    subscription_timeframe: u64,
    max_amount: u64,
) -> ProgramResult {
    let mut subscription_plan = SubscriptionPlan::unpack_unchecked(&subscription_plan_account.data.borrow())?;
    if subscription_plan.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    subscription_plan.is_initialized = true; // TODO: check
    subscription_plan.owner = owner;
    subscription_plan.token_mint = token_mint;
    subscription_plan.subscription_timeframe = subscription_timeframe;
    subscription_plan.max_amount = max_amount;

    SubscriptionPlan::pack(subscription_plan, &mut subscription_plan_account.data.borrow_mut())
}

fn pack_subscription(
    subscription_account: &AccountInfo,
    subscription_plan_account: Pubkey,
    token_address: Pubkey,
    owner: Pubkey,
    cycle_start: UnixTimestamp,
    subscription_timeframe: u64,
    max_amount: u64,
) -> ProgramResult {
    let mut subscription = Subscription::unpack_unchecked(&subscription_account.data.borrow())?;
    if subscription.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    subscription.is_initialized = true; // TODO: check
    subscription.is_approved = true; // TODO: check
    subscription.subscription_plan_account = subscription_plan_account;
    subscription.token_address = token_address;
    subscription.owner = owner;
    subscription.cycle_start = cycle_start;
    subscription.subscription_timeframe = subscription_timeframe;
    subscription.max_amount = max_amount;
    subscription.withdrawn_amount = 0;

    Subscription::pack(subscription, &mut subscription_account.data.borrow_mut())
}

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
    program::invoke_signed,
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
                nonce,
                subscription_timeframe,
                max_amount,
            } => {
                Self::process_create_subscription_plan(accounts, nonce, subscription_timeframe, max_amount, program_id)
            }
            RecurringPaymentsInstruction::CreateSubscription {
                subscription_timeframe,
                max_amount,
            } => Self::process_create_subscription(accounts, subscription_timeframe, max_amount, program_id),
            RecurringPaymentsInstruction::Claim {} => Self::process_claim(accounts, program_id),
        }
    }

    fn process_create_subscription_plan(
        accounts: &[AccountInfo],
        nonce: u8,
        subscription_timeframe: u64,
        max_amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let subscription_plan_account_info = next_account_info(account_info_iter)?;
        let owner_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let token_info = next_account_info(account_info_iter)?;

        if *authority_info.key != Self::authority_id(program_id, subscription_plan_account_info.key, nonce)? {
            return Err(RecurringPaymentsError::InvalidProgramAddress.into());
        }

        pack_subscription_plan(
            subscription_plan_account_info,
            nonce,
            *owner_info.key,
            *authority_info.key,
            *token_info.key,
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

        let subscription_account_info = next_account_info(account_info_iter)?;
        let subscription_plan_account_info = next_account_info(account_info_iter)?;
        let token_account_info = next_account_info(account_info_iter)?;
        let _token_program_info = next_account_info(account_info_iter)?;
        let clock_sysvar_info = next_account_info(account_info_iter)?;
        let fee_account_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(clock_sysvar_info)?;
        let cycle_start = clock.unix_timestamp;

        let subscription_plan = SubscriptionPlan::unpack(&subscription_plan_account_info.data.borrow())?;

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
            subscription_account_info,
            *subscription_plan_account_info.key,
            *token_account_info.key,
            *fee_account_info.key,
            subscription_plan.owner,
            cycle_start,
            subscription_timeframe,
            max_amount,
        )?;

        Ok(())
    }

    fn process_claim(_accounts: &[AccountInfo], _program_id: &Pubkey) -> ProgramResult {
        

        Ok(())
    }

    /// Calculates the authority id by generating a program address.
    pub fn authority_id(program_id: &Pubkey, my_info: &Pubkey, nonce: u8) -> Result<Pubkey, RecurringPaymentsError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(RecurringPaymentsError::InvalidProgramAddress))
    }

    

    /// Unpacks a spl_token `Account`.
    pub fn unpack_token_account(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<spl_token::state::Account, RecurringPaymentsError> {
        if account_info.owner != token_program_id {
            Err(RecurringPaymentsError::IncorrectTokenProgramId)
        } else {
            spl_token::state::Account::unpack(&account_info.data.borrow())
                .map_err(|_| RecurringPaymentsError::ExpectedAccount)
        }
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
            RecurringPaymentsError::InvalidProgramAddress => {
                msg!("Error: Invalid program address generated from nonce and key")
            }
            RecurringPaymentsError::IncorrectTokenProgramId => {
                msg!("Error: The provided token program does not match the expected token program")
            }
            RecurringPaymentsError::ExpectedAccount => msg!("Error: Deserialized account is not an SPL Token account"),
        }
    }
}

fn pack_subscription_plan(
    subscription_plan_account_info: &AccountInfo,
    nonce: u8,
    owner: Pubkey,
    authority: Pubkey,
    token: Pubkey,
    subscription_timeframe: u64,
    max_amount: u64,
) -> ProgramResult {
    let mut subscription_plan = SubscriptionPlan::unpack_unchecked(&subscription_plan_account_info.data.borrow())?;
    if subscription_plan.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    subscription_plan.is_initialized = true; // TODO: check
    subscription_plan.nonce = nonce;
    subscription_plan.owner = owner;
    subscription_plan.authority = authority;
    subscription_plan.token = token;
    subscription_plan.subscription_timeframe = subscription_timeframe;
    subscription_plan.max_amount = max_amount;

    SubscriptionPlan::pack(subscription_plan, &mut subscription_plan_account_info.data.borrow_mut())
}

fn pack_subscription(
    subscription_account_info: &AccountInfo,
    subscription_plan_account: Pubkey,
    token_account: Pubkey,
    _fee_account: Pubkey,
    owner: Pubkey,
    cycle_start: UnixTimestamp,
    subscription_timeframe: u64,
    max_amount: u64,
) -> ProgramResult {
    let mut subscription = Subscription::unpack_unchecked(&subscription_account_info.data.borrow())?;
    if subscription.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    subscription.is_initialized = true;
    subscription.is_approved = true; // TODO: check
    subscription.subscription_plan_account = subscription_plan_account;
    subscription.token_account = token_account;
    subscription.owner = owner;
    subscription.cycle_start = cycle_start;
    subscription.subscription_timeframe = subscription_timeframe;
    subscription.max_amount = max_amount;
    subscription.withdrawn_amount = 0;

    Subscription::pack(subscription, &mut subscription_account_info.data.borrow_mut())
}

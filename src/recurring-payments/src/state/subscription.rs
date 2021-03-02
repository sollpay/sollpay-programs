use crate::constants::SUBSCRIPTION_SIZE;
use arrayref::{array_mut_ref, array_ref, mut_array_refs};
use solana_program::{
  clock::UnixTimestamp,
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack, Sealed},
  pubkey::Pubkey,
};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Subscription {
  pub is_initialized: bool,
  pub is_approved: bool, // true if the subscription is active
  pub subscription_plan_account: Pubkey,
  pub token_account: Pubkey,
  pub owner: Pubkey,
  // pub customer: Pubkey,            // customer that allowed for withdraw
  // pub payout_address: Pubkey,      // address of the Business that can withdraw
  pub cycle_start: UnixTimestamp,  // start of the subscription cycle
  pub subscription_timeframe: u64, // length of the subscription (1 Month ususally) in days
  pub max_amount: u64,             // max amount that can be withdrawn in one timeframe
  pub withdrawn_amount: u64,       // amount that has been withdrawn so far this timeframe
}

impl Sealed for Subscription {}

impl IsInitialized for Subscription {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

impl Pack for Subscription {
  const LEN: usize = SUBSCRIPTION_SIZE;

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, Subscription::LEN];

    let (is_initialized, _src) = src.split_at(1);
    let is_initialized = match is_initialized {
      [0] => false,
      [1] => true,
      _ => return Err(ProgramError::InvalidAccountData),
    };

    let (is_approved, src) = src.split_at(1);
    let is_approved = match is_approved {
      [0] => false,
      [1] => true,
      _ => return Err(ProgramError::InvalidAccountData),
    };

    let (subscription_plan_account, src) = src.split_at(32);
    let subscription_plan_account = Pubkey::new_from_array(
      subscription_plan_account
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?,
    );
    let (token_account, src) = src.split_at(32);
    let token_account = Pubkey::new_from_array(token_account.try_into().map_err(|_| ProgramError::InvalidAccountData)?);
    let (owner, src) = src.split_at(32);
    let owner = Pubkey::new_from_array(owner.try_into().map_err(|_| ProgramError::InvalidAccountData)?);

    let (cycle_start, src) = src.split_at(8);
    let cycle_start =
      UnixTimestamp::from_le_bytes(cycle_start.try_into().map_err(|_| ProgramError::InvalidAccountData)?);

    let (subscription_timeframe, src) = src.split_at(8);
    let subscription_timeframe = u64::from_le_bytes(
      subscription_timeframe
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?,
    );

    let (max_amount, src) = src.split_at(8);
    let max_amount = u64::from_le_bytes(max_amount.try_into().map_err(|_| ProgramError::InvalidAccountData)?);

    let (withdrawn_amount, _src) = src.split_at(8);
    let withdrawn_amount = u64::from_le_bytes(
      withdrawn_amount
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?,
    );

    Ok(Subscription {
      is_initialized,
      is_approved,
      subscription_plan_account,
      token_account,
      owner,
      cycle_start,
      subscription_timeframe,
      max_amount,
      withdrawn_amount,
    })
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, Subscription::LEN];
    let (
      is_initialized_dst,
      is_approved_dst,
      subscription_plan_account_dst,
      token_account_dst,
      owner_dst,
      cycle_start_dst,
      subscription_timeframe_dst,
      max_amount_dst,
      withdrawn_amount_dst,
    ) = mut_array_refs![dst, 1, 1, 32, 32, 32, 8, 8, 8, 8];

    let &Subscription {
      is_initialized,
      is_approved,
      ref subscription_plan_account,
      ref token_account,
      ref owner,
      cycle_start,
      subscription_timeframe,
      max_amount,
      withdrawn_amount,
    } = self;

    is_approved_dst[0] = is_approved as u8;
    is_initialized_dst[0] = is_initialized as u8;
    *subscription_plan_account_dst = subscription_plan_account.to_bytes();
    *token_account_dst = token_account.to_bytes();
    *owner_dst = owner.to_bytes();
    *cycle_start_dst = cycle_start.to_le_bytes();
    *subscription_timeframe_dst = subscription_timeframe.to_le_bytes();
    *max_amount_dst = max_amount.to_le_bytes();
    *withdrawn_amount_dst = withdrawn_amount.to_le_bytes();
  }
}

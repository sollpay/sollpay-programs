use crate::constants::SUBSCRIPTION_PLAN_SIZE;
use arrayref::{array_mut_ref, array_ref, mut_array_refs};
use solana_program::{
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack, Sealed},
  pubkey::Pubkey,
};
use std::convert::TryInto;

#[derive(Debug)]
pub struct SubscriptionPlan {
  pub is_initialized: bool,
  pub nonce: u8,
  pub owner: Pubkey,
  pub authority: Pubkey,
  pub token: Pubkey,
  pub subscription_timeframe: u64, // length of the subscription (1 Month ususally) in days
  pub max_amount: u64,             // max amount that can be withdrawn in one timeframe
}

impl Sealed for SubscriptionPlan {}

impl IsInitialized for SubscriptionPlan {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

impl Pack for SubscriptionPlan {
  const LEN: usize = SUBSCRIPTION_PLAN_SIZE;

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, SubscriptionPlan::LEN];

    let (is_initialized, _src) = src.split_at(1);
    let is_initialized = match is_initialized {
      [0] => false,
      [1] => true,
      _ => return Err(ProgramError::InvalidAccountData),
    };

    let (nonce, src) = src.split_at(1);
    let nonce = nonce[0];
    let (owner, src) = src.split_at(32);
    let owner = Pubkey::new_from_array(owner.try_into().map_err(|_| ProgramError::InvalidAccountData)?);
    let (authority, src) = src.split_at(32);
    let authority = Pubkey::new_from_array(authority.try_into().map_err(|_| ProgramError::InvalidAccountData)?);
    let (token, src) = src.split_at(32);
    let token = Pubkey::new_from_array(token.try_into().map_err(|_| ProgramError::InvalidAccountData)?);

    let (subscription_timeframe, src) = src.split_at(8);
    let subscription_timeframe = u64::from_le_bytes(
      subscription_timeframe
        .try_into()
        .map_err(|_| ProgramError::InvalidAccountData)?,
    );

    let (max_amount, _src) = src.split_at(8);
    let max_amount = u64::from_le_bytes(max_amount.try_into().map_err(|_| ProgramError::InvalidAccountData)?);

    Ok(SubscriptionPlan {
      is_initialized,
      nonce,
      owner,
      authority,
      token,
      subscription_timeframe,
      max_amount,
    })
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, SubscriptionPlan::LEN];
    let (
      is_initialized_dst,
      nonce_dst,
      owner_dst,
      authority_dst,
      token_dst,
      subscription_timeframe_dst,
      max_amount_dst,
    ) = mut_array_refs![dst, 1, 1, 32, 32, 32, 8, 8];

    let &SubscriptionPlan {
      is_initialized,
      nonce,
      owner,
      authority,
      token,
      subscription_timeframe,
      max_amount,
    } = self;

    is_initialized_dst[0] = is_initialized as u8;
    nonce_dst[0] = nonce;
    *owner_dst = owner.to_bytes();
    *authority_dst = authority.to_bytes();
    *token_dst = token.to_bytes();
    *subscription_timeframe_dst = subscription_timeframe.to_le_bytes();
    *max_amount_dst = max_amount.to_le_bytes();

    // is_initialized_dst[0] = *is_initialized as u8;
    // nonce_dst[0] = *nonce;
    // owner_dst.copy_from_slice(owner.as_ref());
    // authority_dst.copy_from_slice(authority.as_ref());
    // token_dst.copy_from_slice(token.as_ref());
    // *subscription_timeframe_dst = subscription_timeframe.to_le_bytes();
    // *max_amount_dst = max_amount.to_le_bytes();
  }
}

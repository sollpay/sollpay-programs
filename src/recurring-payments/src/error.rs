//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the RecurringPayments program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RecurringPaymentsError {
  /// Invalid instruction number passed in.
  #[error("Invalid instruction")]
  InvalidInstruction,
  #[error("Invalid max amount")]
  InvalidMaxAmount,
  #[error("Invalid subscription timeframe")]
  InvalidSubscriptionTimeframe,
  #[error("Invalid program address generated from nonce and key")]
  InvalidProgramAddress,
  #[error("The provided token program does not match the expected token program")]
  IncorrectTokenProgramId,
  #[error("Deserialized account is not an SPL Token account")]
  ExpectedAccount,
}

impl From<RecurringPaymentsError> for ProgramError {
  fn from(e: RecurringPaymentsError) -> Self {
    ProgramError::Custom(e as u32)
  }
}

impl<T> DecodeError<T> for RecurringPaymentsError {
  fn type_of() -> &'static str {
    "RecurringPaymentsError"
  }
}

//! Program entrypoint definitions

use crate::{error::RecurringPaymentsError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, program_error::PrintProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    msg!(
        "Process instruction. Program id: {}, {} accounts, data: {:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<RecurringPaymentsError>();
        return Err(error);
    }
    Ok(())
}

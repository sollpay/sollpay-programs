import {
  BPF_LOADER_PROGRAM_ID,
  SystemProgram,
  Transaction,
  Account,
  BpfLoader,
  Connection,
  PublicKey
} from '@solana/web3.js'
import * as fs from 'fs'
import { sendAndConfirmTransaction } from './util/send-and-confirm-transaction'

export async function estCostLoadProgram(connection, pathToProgram) {
  const { feeCalculator } = await connection.getRecentBlockhash()

  const data = await fs.readFileSync(pathToProgram)

  const cost =
    feeCalculator.lamportsPerSignature * ((BpfLoader.getMinNumSignatures(data.length) + 2) * 2) +
    (await connection.getMinimumBalanceForRentExemption(data.length))

  return cost
}

export async function loadProgram(connection, payerAccount, pathToProgram) {
  const data = await fs.readFileSync(pathToProgram)

  const programAccount = new Account()

  console.log('ProgramAccount:', programAccount.publicKey.toString())

  await BpfLoader.load(
    connection,
    payerAccount,
    programAccount,
    data,
    BPF_LOADER_PROGRAM_ID
  )
  //await BpfLoader.load(connection, payerAccount, programAccount, data, BPF_LOADER_PROGRAM_ID)

  return programAccount.publicKey
}

export async function estCostMakeAccount(connection, numBytes) {
  const { feeCalculator } = await connection.getRecentBlockhash()

  return (
    (await connection.getMinimumBalanceForRentExemption(numBytes)) +
    2 * feeCalculator.lamportsPerSignature
  )
}

export async function makeAccount(
  connection: Connection,
  payerAccount: Account,
  numBytes,
  programId: PublicKey
) {
  const dataAccount = new Account()
  const rentExemption = await connection.getMinimumBalanceForRentExemption(numBytes)
  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payerAccount.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: rentExemption,
      space: numBytes,
      programId: programId
    })
  )
  await sendAndConfirmTransaction(
    'createAccount',
    connection,
    transaction,
    payerAccount,
    dataAccount
  )

  return dataAccount.publicKey
}

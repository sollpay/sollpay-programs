import { sendAndConfirmTransaction as realSendAndConfirmTransaction } from '@solana/web3.js'
import { Account, Connection, Transaction } from '@solana/web3.js'

export async function sendAndConfirmTransaction(
  title: string,
  connection: Connection,
  transaction: Transaction,
  ...signers: Array<Account>
): Promise<void> {
  console.log(`Confirming ${title}`)
  const signature = await realSendAndConfirmTransaction(connection, transaction, signers, {
    commitment: 'max'
  })
}

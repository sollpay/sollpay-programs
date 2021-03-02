import {
  Transaction,
  PublicKey,
  sendAndConfirmTransaction,
  TransactionInstruction,
  SYSVAR_CLOCK_PUBKEY,
  Account,
  SystemProgram
} from '@solana/web3.js'
import { getOurAccount } from '../ourAccount'
import { getNodeConnection } from '../nodeConnection'
import { createToken, TOKEN_PROGRAM_ID } from '../createToken'
import { Token } from '@solana/spl-token'
import { airDrop } from '../util/air-drop'
import { getStore } from '../storeConfig'
import * as BufferLayout from 'buffer-layout'
import { Numberu64 } from '@solana/spl-token-swap'

/**
 * Layout for a public key
 */
export const publicKey = (property = 'publicKey'): BufferLayout.Layout => {
  return BufferLayout.blob(32, property)
}

/**
 * Layout for a 64bit unsigned value
 */
const uint64 = (property: string = 'uint64'): Object => {
  return BufferLayout.blob(8, property)
}

function createSubscriptionPlanInstruction(
  subscriptionAccount: PublicKey,
  tokenAddress: PublicKey,
  owner: PublicKey,
  maxAmount: number | Numberu64,
  subscriptionTimeframe: number | Numberu64,
  recurringPaymentsProgramId: PublicKey
): TransactionInstruction {
  const dataLayout = BufferLayout.struct([
    BufferLayout.u8('instruction'),
    uint64('subscription_timeframe'),
    uint64('max_amount')
  ])

  const data = Buffer.alloc(dataLayout.span)
  dataLayout.encode(
    {
      instruction: 1, // CreateSubscriptionPlan instruction
      // @ts-ignore
      subscription_timeframe: new Numberu64(subscriptionTimeframe).toBuffer(),
      // @ts-ignore
      max_amount: new Numberu64(maxAmount).toBuffer()
    },
    data
  )

  const keys = [
    { pubkey: subscriptionAccount, isSigner: false, isWritable: true },
    { pubkey: tokenAddress, isSigner: false, isWritable: false },
    { pubkey: owner, isSigner: false, isWritable: false }
  ]

  return new TransactionInstruction({
    keys,
    programId: recurringPaymentsProgramId,
    data
  })
}

function createSubscriptionInstruction(
  subscriptionAccount: PublicKey,
  tokenAddress: PublicKey,
  customer: PublicKey,
  payoutAddress: PublicKey,
  maxAmount: number | Numberu64,
  subscriptionTimeframe: number | Numberu64,
  recurringPaymentsProgramId: PublicKey
): TransactionInstruction {
  const dataLayout = BufferLayout.struct([
    BufferLayout.u8('instruction'),
    uint64('max_amount'),
    uint64('subscription_timeframe')
  ])

  const data = Buffer.alloc(dataLayout.span)
  dataLayout.encode(
    {
      instruction: 0, // CreateSubscription instruction
      // @ts-ignore
      subscription_timeframe: new Numberu64(subscriptionTimeframe).toBuffer(),
      // @ts-ignore
      max_amount: new Numberu64(maxAmount).toBuffer()
    },
    data
  )

  const keys = [
    { pubkey: subscriptionAccount, isSigner: false, isWritable: true },
    { pubkey: tokenAddress, isSigner: false, isWritable: false },
    { pubkey: customer, isSigner: false, isWritable: false },
    { pubkey: payoutAddress, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false }
  ]

  return new TransactionInstruction({
    keys,
    programId: recurringPaymentsProgramId,
    data
  })
}

const SUBSCRIPTION_PLAN_SIZE = 81
const SUBSCRIPTION_SIZE = 162

const main = async () => {
  const _ourAccount = await getOurAccount()

  const connection = await getNodeConnection()
  const s = await getStore(connection, 'recurring-payments.json')

  console.log(await connection.getBalance(s.accountId))

  // Create token
  const _tokenAddress = await createToken(connection, _ourAccount, 9, undefined)
  const token = new Token(connection, _tokenAddress, TOKEN_PROGRAM_ID, _ourAccount)

  const subscriptionAccount = new Account()
  console.log('subscriptionAccount', subscriptionAccount.publicKey.toBase58())
  const subscriptionPlanAccount = new Account()
  console.log('subscriptionPlanAccount', subscriptionPlanAccount.publicKey.toBase58())

  const tokenAddress = await token.createAccount(_ourAccount.publicKey)
  // Mint token account for test
  await token.mintTo(tokenAddress, _ourAccount, [], 1000 * 10 ** 9)

  const customer = new Account()
  const payoutAddress = new Account()
  const maxAmount = 10
  const subscriptionTimeframe = 10

  const transaction = new Transaction()
  // Subscription account
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: _ourAccount.publicKey,
      newAccountPubkey: subscriptionAccount.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(SUBSCRIPTION_SIZE),
      space: SUBSCRIPTION_SIZE,
      programId: s.programId
    })
  )

  // SubscriptionPlan account
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: _ourAccount.publicKey,
      newAccountPubkey: subscriptionPlanAccount.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(SUBSCRIPTION_PLAN_SIZE),
      space: SUBSCRIPTION_PLAN_SIZE,
      programId: s.programId
    })
  )

  await token.approve(tokenAddress, s.programId, _ourAccount, [], 500)

  // Approve
  // transaction.add(
  //   Token.createApproveInstruction(
  //     TOKEN_PROGRAM_ID, // programId SPL Token program account
  //     tokenAddress, // account Public key of the account
  //     s.programId, // delegate Account authorized to perform a transfer of tokens from the source account
  //     _ourAccount.publicKey, // owner Owner of the source account
  //     [], // multiSigners Signing accounts if `owner` is a multiSig
  //     500 // amount Maximum number of tokens the delegate may transfer
  //   )
  // );

  transaction.add(
    createSubscriptionPlanInstruction(
      subscriptionPlanAccount.publicKey,
      tokenAddress,
      _ourAccount.publicKey,
      subscriptionTimeframe,
      maxAmount,
      s.programId
    )
  )

  transaction.add(
    createSubscriptionInstruction(
      subscriptionAccount.publicKey,
      tokenAddress,
      customer.publicKey,
      payoutAddress.publicKey,
      maxAmount,
      subscriptionTimeframe,
      s.programId
    )
  )

  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [_ourAccount, subscriptionAccount, subscriptionPlanAccount],
    {
      commitment: 'max'
    }
  )
  console.log(signature)
}
main()

import { Account, PublicKey, SYSVAR_CLOCK_PUBKEY, TransactionInstruction } from '@solana/web3.js'
import { Token as SPLToken } from '@solana/spl-token'


const TOKEN_PROGRAM_ID: PublicKey = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
const RECURRING_PAYMENTS_PROGRAM_ID: PublicKey = new PublicKey('recurring payments program')

const walletPublicKey: PublicKey = new PublicKey('current wallet')
const merchantPublicKey: PublicKey = new PublicKey('merchant public key')
const subscriptionAccount: Account = new Account() // New account for subscription data

const sourcePublicKey: PublicKey = new PublicKey('') // PublicKey of token account
const amount: number = 1000

SPLToken.createApproveInstruction(
  TOKEN_PROGRAM_ID,
  sourcePublicKey,
  RECURRING_PAYMENTS_PROGRAM_ID,
  walletPublicKey,
  [],
  amount
)

subscribeInstruction(RECURRING_PAYMENTS_PROGRAM_ID, merchantPublicKey, subscriptionAccount)

// instruction subscribe

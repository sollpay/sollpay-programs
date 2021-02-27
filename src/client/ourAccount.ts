import { Account } from '@solana/web3.js'
import * as fs from 'fs'

export async function getOurAccount() {
  const keypairFile = './keypair.json'

  if (!fs.existsSync(keypairFile)) {
    console.log('The expected keypair file', keypairFile, 'was not found')
    process.exit(1)
  }
  const checkSecret = fs
    .readFileSync(keypairFile, 'utf-8')
    .split(',')
    .map(x => parseInt(x))

  const account = new Account(checkSecret)
  console.log('Our account:', account.publicKey.toBase58())
  return account
}

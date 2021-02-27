import { Account } from '@solana/web3.js'
import * as fs from 'fs'

async function main() {
  const keypairFile = './keypair.json'

  if (fs.existsSync(keypairFile)) {
    console.log('The file', keypairFile, 'already exists.')
    return
  }

  console.log('-----')
  console.log('Making new keypair')
  console.log()

  const wallet = new Account()

  const sk = wallet.secretKey.slice(0, 32)
  const pk = wallet.secretKey.slice(32)
  const address = wallet.publicKey.toBase58()

  console.log('SK in bytes:', sk)
  console.log('PK in bytes:', pk)
  console.log("PK as base58 ('the address'):", address)

  fs.writeFileSync(keypairFile, wallet.secretKey.toString())
  console.log(wallet.secretKey.toString())
  const checkSecret = fs
    .readFileSync(keypairFile, 'utf-8')
    .split(',')
    .map(x => parseInt(x))

  const checkWallet = new Account(checkSecret)
  if (checkWallet.publicKey.toBase58() !== address) {
    console.log('Something went wrong')
    process.exit(1)
  }

  console.log('Wallet keypair is in root of project:', keypairFile)
  console.log('-----')
}

main()

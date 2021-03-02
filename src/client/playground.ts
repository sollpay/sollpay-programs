import {
  Transaction,
  SystemProgram,
  PublicKey,
  sendAndConfirmTransaction
} from '@solana/web3.js'
import { getOurAccount } from './ourAccount'
import { getNodeConnection } from './nodeConnection'
// const url = 'http://devnet.solana.com'
// const connection = new solanaWeb3.Connection(url)
import { getStore } from './storeConfig'

const main = async () => {
  const ourAccount = await getOurAccount()

  const connection = await getNodeConnection()
  // connection.getBalance(ourAccount.publicKey).then(balance => {
  //   console.log(`${ourAccount.publicKey} has a balance of ${balance}`)
  // })
  const s = await getStore(connection, 'token-name-service.json')

  // console.log(new PublicKey('7JRWSgMszap7MUUcZMaMXpUHEAyQ1k9y5cTBvRxQMCU6'))
  // // // // await sendAndConfirmTransaction(
  // // // //   'vote',
  // // // //   connection,
  // // // //   new Transaction().add(instruction),
  // // // //   ourAccount
  // // // // )
  // await connection.requestAirdrop(s.accountId, 1 * 1e9)
  console.log(await connection.getBalance(s.accountId))

  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: ourAccount.publicKey,
      toPubkey: new PublicKey('gGpPMzvCxptKYXLZvpGra8HNJMZX7LKkX6ySc2ZLGAn'),
      lamports: 2 * 1e9
    })
  )

  const signature = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(transaction),
    [ourAccount],
    {
      commitment: 'max'
    }
  )
  console.log(signature)
}
main()

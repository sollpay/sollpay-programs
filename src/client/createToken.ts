import { Connection, PublicKey, Account } from "@solana/web3.js";
import { Token } from "@solana/spl-token";

export const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

export async function createToken(
  connection: Connection,
  ourAccount: Account,
  decimals: number = 9,
  freezeAuthority?: string,
  mintAuthority?: string
): Promise<PublicKey> {
  console.log("create token");
  const token = await Token.createMint(
    connection,
    ourAccount,
    mintAuthority ? new PublicKey(mintAuthority) : ourAccount.publicKey,
    freezeAuthority ? new PublicKey(freezeAuthority) : null,
    decimals,
    TOKEN_PROGRAM_ID
  );
  // @ts-expect-error
  console.log(`created token ${token.publicKey.toString()}`);
  // @ts-expect-error
  return token.publicKey;
}

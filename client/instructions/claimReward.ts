import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ClaimRewardArgs {
  gameId: string
  gameBump: number
}

export interface ClaimRewardAccounts {
  game: PublicKey
  escrowTokenAccount: PublicKey
  ownerTokenAccount: PublicKey
  owner: PublicKey
  tokenProgram: PublicKey
}

export const layout = borsh.struct([borsh.str("gameId"), borsh.u8("gameBump")])

export function claimReward(
  args: ClaimRewardArgs,
  accounts: ClaimRewardAccounts
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.game, isSigner: false, isWritable: true },
    { pubkey: accounts.escrowTokenAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.ownerTokenAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.owner, isSigner: true, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([149, 95, 181, 242, 94, 90, 158, 162])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      gameId: args.gameId,
      gameBump: args.gameBump,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}

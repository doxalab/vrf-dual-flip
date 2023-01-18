import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface InitVrfArgs {
  params: types.InitVrfParamsFields
}

export interface InitVrfAccounts {
  state: PublicKey
  vrfState: PublicKey
  vrf: PublicKey
  payer: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([types.InitVrfParams.layout("params")])

export function initVrf(args: InitVrfArgs, accounts: InitVrfAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.state, isSigner: false, isWritable: true },
    { pubkey: accounts.vrfState, isSigner: false, isWritable: true },
    { pubkey: accounts.vrf, isSigner: false, isWritable: false },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([214, 244, 0, 243, 20, 87, 110, 182])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      params: types.InitVrfParams.toEncodable(args.params),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}

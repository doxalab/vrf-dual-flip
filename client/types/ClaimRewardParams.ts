import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"

export interface ClaimRewardParamsFields {}

export interface ClaimRewardParamsJSON {}

export class ClaimRewardParams {
  constructor(fields: ClaimRewardParamsFields) {}

  static layout(property?: string) {
    return borsh.struct([], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new ClaimRewardParams({})
  }

  static toEncodable(fields: ClaimRewardParamsFields) {
    return {}
  }

  toJSON(): ClaimRewardParamsJSON {
    return {}
  }

  static fromJSON(obj: ClaimRewardParamsJSON): ClaimRewardParams {
    return new ClaimRewardParams({})
  }

  toEncodable() {
    return ClaimRewardParams.toEncodable(this)
  }
}

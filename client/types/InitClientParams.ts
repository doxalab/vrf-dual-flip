import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"

export interface InitClientParamsFields {
  maxResult: BN
  gameId: string
  choice: BN
  betAmount: BN
}

export interface InitClientParamsJSON {
  maxResult: string
  gameId: string
  choice: string
  betAmount: string
}

export class InitClientParams {
  readonly maxResult: BN
  readonly gameId: string
  readonly choice: BN
  readonly betAmount: BN

  constructor(fields: InitClientParamsFields) {
    this.maxResult = fields.maxResult
    this.gameId = fields.gameId
    this.choice = fields.choice
    this.betAmount = fields.betAmount
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("maxResult"),
        borsh.str("gameId"),
        borsh.u64("choice"),
        borsh.u64("betAmount"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new InitClientParams({
      maxResult: obj.maxResult,
      gameId: obj.gameId,
      choice: obj.choice,
      betAmount: obj.betAmount,
    })
  }

  static toEncodable(fields: InitClientParamsFields) {
    return {
      maxResult: fields.maxResult,
      gameId: fields.gameId,
      choice: fields.choice,
      betAmount: fields.betAmount,
    }
  }

  toJSON(): InitClientParamsJSON {
    return {
      maxResult: this.maxResult.toString(),
      gameId: this.gameId,
      choice: this.choice.toString(),
      betAmount: this.betAmount.toString(),
    }
  }

  static fromJSON(obj: InitClientParamsJSON): InitClientParams {
    return new InitClientParams({
      maxResult: new BN(obj.maxResult),
      gameId: obj.gameId,
      choice: new BN(obj.choice),
      betAmount: new BN(obj.betAmount),
    })
  }

  toEncodable() {
    return InitClientParams.toEncodable(this)
  }
}

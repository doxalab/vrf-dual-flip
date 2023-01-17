import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"

export interface RequestRandomnessParamsFields {
  permissionBump: number
  switchboardStateBump: number
  gameId: string
}

export interface RequestRandomnessParamsJSON {
  permissionBump: number
  switchboardStateBump: number
  gameId: string
}

export class RequestRandomnessParams {
  readonly permissionBump: number
  readonly switchboardStateBump: number
  readonly gameId: string

  constructor(fields: RequestRandomnessParamsFields) {
    this.permissionBump = fields.permissionBump
    this.switchboardStateBump = fields.switchboardStateBump
    this.gameId = fields.gameId
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u8("permissionBump"),
        borsh.u8("switchboardStateBump"),
        borsh.str("gameId"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new RequestRandomnessParams({
      permissionBump: obj.permissionBump,
      switchboardStateBump: obj.switchboardStateBump,
      gameId: obj.gameId,
    })
  }

  static toEncodable(fields: RequestRandomnessParamsFields) {
    return {
      permissionBump: fields.permissionBump,
      switchboardStateBump: fields.switchboardStateBump,
      gameId: fields.gameId,
    }
  }

  toJSON(): RequestRandomnessParamsJSON {
    return {
      permissionBump: this.permissionBump,
      switchboardStateBump: this.switchboardStateBump,
      gameId: this.gameId,
    }
  }

  static fromJSON(obj: RequestRandomnessParamsJSON): RequestRandomnessParams {
    return new RequestRandomnessParams({
      permissionBump: obj.permissionBump,
      switchboardStateBump: obj.switchboardStateBump,
      gameId: obj.gameId,
    })
  }

  toEncodable() {
    return RequestRandomnessParams.toEncodable(this)
  }
}

import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface VRFKeyFields {
  key: PublicKey
}

export interface VRFKeyJSON {
  key: string
}

export class VRFKey {
  readonly key: PublicKey

  static readonly discriminator = Buffer.from([
    225, 77, 130, 75, 151, 34, 64, 46,
  ])

  static readonly layout = borsh.struct([borsh.publicKey("key")])

  constructor(fields: VRFKeyFields) {
    this.key = fields.key
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<VRFKey | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(PROGRAM_ID)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[]
  ): Promise<Array<VRFKey | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(PROGRAM_ID)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): VRFKey {
    if (!data.slice(0, 8).equals(VRFKey.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = VRFKey.layout.decode(data.slice(8))

    return new VRFKey({
      key: dec.key,
    })
  }

  toJSON(): VRFKeyJSON {
    return {
      key: this.key.toString(),
    }
  }

  static fromJSON(obj: VRFKeyJSON): VRFKey {
    return new VRFKey({
      key: new PublicKey(obj.key),
    })
  }
}

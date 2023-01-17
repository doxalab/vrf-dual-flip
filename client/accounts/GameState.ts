import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface GameStateFields {
  owner: PublicKey
  ownerChoice: BN
  joinee: PublicKey | null
  winner: PublicKey | null
  betAmount: BN
  result: BN | null
  roomCreationTime: BN
}

export interface GameStateJSON {
  owner: string
  ownerChoice: string
  joinee: string | null
  winner: string | null
  betAmount: string
  result: string | null
  roomCreationTime: string
}

export class GameState {
  readonly owner: PublicKey
  readonly ownerChoice: BN
  readonly joinee: PublicKey | null
  readonly winner: PublicKey | null
  readonly betAmount: BN
  readonly result: BN | null
  readonly roomCreationTime: BN

  static readonly discriminator = Buffer.from([
    144, 94, 208, 172, 248, 99, 134, 120,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("owner"),
    borsh.u64("ownerChoice"),
    borsh.option(borsh.publicKey(), "joinee"),
    borsh.option(borsh.publicKey(), "winner"),
    borsh.u64("betAmount"),
    borsh.option(borsh.u64(), "result"),
    borsh.i64("roomCreationTime"),
  ])

  constructor(fields: GameStateFields) {
    this.owner = fields.owner
    this.ownerChoice = fields.ownerChoice
    this.joinee = fields.joinee
    this.winner = fields.winner
    this.betAmount = fields.betAmount
    this.result = fields.result
    this.roomCreationTime = fields.roomCreationTime
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<GameState | null> {
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
  ): Promise<Array<GameState | null>> {
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

  static decode(data: Buffer): GameState {
    if (!data.slice(0, 8).equals(GameState.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = GameState.layout.decode(data.slice(8))

    return new GameState({
      owner: dec.owner,
      ownerChoice: dec.ownerChoice,
      joinee: dec.joinee,
      winner: dec.winner,
      betAmount: dec.betAmount,
      result: dec.result,
      roomCreationTime: dec.roomCreationTime,
    })
  }

  toJSON(): GameStateJSON {
    return {
      owner: this.owner.toString(),
      ownerChoice: this.ownerChoice.toString(),
      joinee: (this.joinee && this.joinee.toString()) || null,
      winner: (this.winner && this.winner.toString()) || null,
      betAmount: this.betAmount.toString(),
      result: (this.result && this.result.toString()) || null,
      roomCreationTime: this.roomCreationTime.toString(),
    }
  }

  static fromJSON(obj: GameStateJSON): GameState {
    return new GameState({
      owner: new PublicKey(obj.owner),
      ownerChoice: new BN(obj.ownerChoice),
      joinee: (obj.joinee && new PublicKey(obj.joinee)) || null,
      winner: (obj.winner && new PublicKey(obj.winner)) || null,
      betAmount: new BN(obj.betAmount),
      result: (obj.result && new BN(obj.result)) || null,
      roomCreationTime: new BN(obj.roomCreationTime),
    })
  }
}

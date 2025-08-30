import { Type } from "@sinclair/typebox";
import { SolanaBaseEventSchema } from "./solana-base";

export const PumpfunTradeEventSchema = Type.Intersect([
  SolanaBaseEventSchema,
  Type.Object({
    mint: Type.String({ description: "Token mint being traded" }),
    solAmount: Type.String({ description: "SOL amount in lamports" }),
    tokenAmount: Type.String({ description: "Token amount in smallest unit" }),
    isBuy: Type.Boolean({ description: "True if buying tokens, false if selling" }),
    trader: Type.String({ description: "Trader wallet address" }),
  }),
], {
  $id: "PumpfunTradeEvent",
  title: "PumpfunTradeEvent",
  description: "Pumpfun trade event",
});
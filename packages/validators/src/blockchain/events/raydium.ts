import { Type } from "@sinclair/typebox";
import { SolanaBaseEventSchema } from "./solana-base";

export const RaydiumTradeEventSchema = Type.Intersect([
  SolanaBaseEventSchema,
  Type.Object({
    swapType: Type.Union([
      Type.Literal("swapBaseIn"),
      Type.Literal("swapBaseOut"),
    ], { description: "Type of swap operation" }),
    poolId: Type.String({ description: "AMM pool identifier" }),
    user: Type.String({ description: "User performing the swap" }),
    // For swapBaseIn
    amountIn: Type.Optional(Type.String({ description: "Exact input amount (for swapBaseIn)" })),
    minimumAmountOut: Type.Optional(Type.String({ description: "Minimum expected output amount (for swapBaseIn)" })),
    // For swapBaseOut  
    maxAmountIn: Type.Optional(Type.String({ description: "Maximum input amount allowed (for swapBaseOut)" })),
    amountOut: Type.Optional(Type.String({ description: "Exact output amount (for swapBaseOut)" })),
  }),
], {
  $id: "RaydiumTradeEvent",
  title: "RaydiumTradeEvent",
  description: "Unified Raydium trade event for all swap types",
});
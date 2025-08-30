import { Type } from "@sinclair/typebox";
import { SolanaBaseEventSchema } from "./solana-base";

export const RaydiumSwapBaseInSchema = Type.Intersect([
  SolanaBaseEventSchema,
  Type.Object({
    poolId: Type.String({ description: "AMM pool identifier" }),
    amountIn: Type.String({ description: "Exact input amount" }),
    minimumAmountOut: Type.String({ description: "Minimum expected output amount" }),
    user: Type.String({ description: "User performing the swap" }),
  }),
], {
  $id: "RaydiumSwapBaseIn",
  title: "RaydiumSwapBaseIn",
  description: "Raydium swap base in trade event",
});

export const RaydiumSwapBaseOutSchema = Type.Intersect([
  SolanaBaseEventSchema,
  Type.Object({
    poolId: Type.String({ description: "AMM pool identifier" }),
    maxAmountIn: Type.String({ description: "Maximum input amount allowed" }),
    amountOut: Type.String({ description: "Exact output amount" }),
    user: Type.String({ description: "User performing the swap" }),
  }),
], {
  $id: "RaydiumSwapBaseOut",
  title: "RaydiumSwapBaseOut",
  description: "Raydium swap base out trade event",
});
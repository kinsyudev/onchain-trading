import { Type } from "@sinclair/typebox";

import { BaseEventSchema } from "./base";

export const UniswapV2Swap = Type.Union(
  [
    Type.Object({
      sender: Type.String(),
      to: Type.String(),
      amount0In: Type.String(),
      amount1In: Type.String(),
      amount0Out: Type.String(),
      amount1Out: Type.String(),
      pair: Type.String(),
    }),
    BaseEventSchema,
  ],
  {
    $id: "UniswapV2Swap",
    title: "UniswapV2Swap",
    description: "Event emitted when a swap occurs on a Uniswap V2 pair.",
  },
);

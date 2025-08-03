import { UniswapV2Swap } from "@kinsyu/validators/blockchain/events/uniswap-v2";

export const queueSchemas = {
  "uniswap-v2-swaps": UniswapV2Swap,
} as const;

import { UniswapV2Swap } from "@kinsyu/validators/blockchain/events/uniswap-v2";
import { PumpfunTradeEventSchema } from "@kinsyu/validators/blockchain/events/pumpfun";
import { RaydiumTradeEventSchema } from "@kinsyu/validators/blockchain/events/raydium";

export const queueSchemas = {
  "uniswap-v2-swaps": UniswapV2Swap,
  "pumpfun-trades": PumpfunTradeEventSchema,
  "raydium-trades": RaydiumTradeEventSchema,
} as const;

import { createConfig } from "ponder";

import { env } from "./env";
import { UniV2PairAbi } from "./src/abis/UniswapV2PairAbi";

export default createConfig({
  chains: {
    mainnet: {
      id: 1,
      rpc: env.RPC_URL_1,
      ws: env.RPC_URL_1_WS,
    },
    base: {
      id: 8453,
      rpc: env.RPC_URL_8453,
      ws: env.RPC_URL_8453_WS,
    },
  },
  contracts: {
    UniswapV2PairMainnet: {
      chain: "mainnet",
      abi: UniV2PairAbi,
      startBlock: env.ETH_START_BLOCK,
    },
    UniswapV2PairBase: {
      chain: "base",
      abi: UniV2PairAbi,
      startBlock: env.BASE_START_BLOCK,
    },
  },
});

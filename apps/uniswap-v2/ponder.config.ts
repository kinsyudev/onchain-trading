import { createConfig } from "ponder";

import { UniV2PairAbi } from "./abis/UniswapV2PairAbi";

export const START_BLOCK = 22009797;

export default createConfig({
  chains: {
    mainnet: {
      id: 1,
      rpc: process.env.PONDER_RPC_URL_1!,
    },
  },
  contracts: {
    UniswapV2Pair: {
      chain: "mainnet",
      abi: UniV2PairAbi,
      startBlock: START_BLOCK,
    },
  },
});

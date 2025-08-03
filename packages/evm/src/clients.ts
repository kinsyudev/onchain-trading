import type { PublicClient } from "viem";
import { createPublicClient, http } from "viem";
import {
  arbitrum,
  avalanche,
  base,
  bsc,
  mainnet,
  optimism,
  polygon,
} from "viem/chains";

import { env } from "../env";

export function getClient(chainId: number): PublicClient {
  const rpcUrlKey = `RPC_URL_${chainId}` as keyof typeof env;
  const rpcUrl = env[rpcUrlKey];

  if (!rpcUrl) {
    throw new Error(`RPC_URL_${chainId} is not configured`);
  }

  switch (chainId) {
    case 1:
      return createPublicClient({
        chain: mainnet,
        transport: http(rpcUrl),
      }) as PublicClient;
    case 42161:
      return createPublicClient({
        chain: arbitrum,
        transport: http(rpcUrl),
      }) as PublicClient;
    case 10:
      return createPublicClient({
        chain: optimism,
        transport: http(rpcUrl),
      }) as PublicClient;
    case 137:
      return createPublicClient({
        chain: polygon,
        transport: http(rpcUrl),
      }) as PublicClient;
    case 8453:
      return createPublicClient({
        chain: base,
        transport: http(rpcUrl),
      }) as PublicClient;
    case 56:
      return createPublicClient({
        chain: bsc,
        transport: http(rpcUrl),
      }) as PublicClient;
    case 43114:
      return createPublicClient({
        chain: avalanche,
        transport: http(rpcUrl),
      }) as PublicClient;
    default:
      throw new Error(`Unsupported chain ID: ${chainId}`);
  }
}

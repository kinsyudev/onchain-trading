import { createEnv } from "@t3-oss/env-core";
import { z } from "zod/v4";

export const env = createEnv({
  server: {
    RPC_URL_1: z.string().url().optional(), // Ethereum mainnet
    RPC_URL_10: z.string().url().optional(), // Optimism
    RPC_URL_56: z.string().url().optional(), // BSC
    RPC_URL_137: z.string().url().optional(), // Polygon
    RPC_URL_8453: z.string().url().optional(), // Base
    RPC_URL_42161: z.string().url().optional(), // Arbitrum
    RPC_URL_43114: z.string().url().optional(), // Avalanche
  },
  runtimeEnv: process.env,
  skipValidation:
    !!process.env.CI || process.env.npm_lifecycle_event === "lint",
});
import { createEnv } from "@t3-oss/env-core";
import { z } from "zod/v4";

export const env = createEnv({
  server: {
    RPC_URL_1: z.string(), // Ethereum mainnet
    RPC_URL_1_WS: z.string(), // Ethereum mainnet
    RPC_URL_8453: z.string(), // Base
    RPC_URL_8453_WS: z.string(), // Base
  },
  runtimeEnv: process.env,
  skipValidation:
    !!process.env.CI || process.env.npm_lifecycle_event === "lint",
});

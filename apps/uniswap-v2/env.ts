import { createEnv } from "@t3-oss/env-core";
import { z } from "zod/v4";

import { env as evmEnv } from "@kinsyu/evm/env";
import { env as messagingEnv } from "@kinsyu/messaging/env";

export const env = createEnv({
  extends: [messagingEnv, evmEnv],
  server: {
    NODE_ENV: z
      .enum(["development", "production", "test"])
      .default("development"),
    ETH_START_BLOCK: z.number().gt(0),
    BASE_START_BLOCK: z.number().gt(0),
  },

  runtimeEnv: process.env,
});

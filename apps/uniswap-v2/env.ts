import { createEnv } from "@t3-oss/env-core";
import { z } from "zod/v4";

import { env as messagingEnv } from "@kinsyu/messaging/env";

export const env = createEnv({
  extends: [messagingEnv],
  server: {
    NODE_ENV: z
      .enum(["development", "production", "test"])
      .default("development"),
  },

  runtimeEnv: process.env,
});

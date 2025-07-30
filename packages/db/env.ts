import { createEnv } from "@t3-oss/env-core";
import { z } from "zod";

export const env = createEnv({
  server: {
    POSTGRES_URL: z.string().min(1),
    NODE_ENV: z.enum(["development", "production", "test"]).optional(),
  },
  client: {},
  clientPrefix: "PUBLIC_",
  runtimeEnv: process.env,
  skipValidation:
    !!process.env.CI || process.env.npm_lifecycle_event === "lint",
});

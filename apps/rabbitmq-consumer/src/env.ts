import { createEnv } from "@t3-oss/env-core";
import { z } from "zod/v4";

export const env = createEnv({
  server: {
    NODE_ENV: z
      .enum(["development", "production", "test"])
      .default("development"),
    RABBITMQ_URL: z.string().url().default("amqp://localhost:5672"),
  },
  runtimeEnv: process.env,
  emptyStringAsUndefined: true,
});

import type { Static } from "@kinsyu/validators";

import type { queueSchemas } from "./config";

export type SupportedQueues = keyof typeof queueSchemas;

export type QueueMessageMap = typeof queueSchemas;

export type InferQueueMessage<T extends SupportedQueues> = Static<
  QueueMessageMap[T]
>;

export interface QueueConfig {
  durable: boolean;
  autoDelete: boolean;
  exclusive: boolean;
  arguments?: Record<string, unknown>;
}

export interface ConnectionOptions {
  hostname?: string;
  port?: number;
  username?: string;
  password?: string;
  vhost?: string;
}

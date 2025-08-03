import type { Channel } from "amqplib";
import { Value } from "@sinclair/typebox/value";

import type { InferQueueMessage, SupportedQueues } from "./types";
import { queueSchemas } from "./config";

export function sendMessageToQueue<T extends SupportedQueues>(
  channel: Channel,
  queue: T,
  message: InferQueueMessage<T>,
): void {
  const schema = queueSchemas[queue];

  if (!Value.Check(schema, message)) {
    const errors = [...Value.Errors(schema, message)];
    throw new Error(
      `Invalid message for queue ${queue}: ${JSON.stringify(errors)}`,
    );
  }

  const messageBuffer = Buffer.from(JSON.stringify(message));

  channel.sendToQueue(queue, messageBuffer, {
    persistent: true,
  });
}

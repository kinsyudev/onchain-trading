import type { Channel, ConsumeMessage } from "amqplib";
import { Value } from "@sinclair/typebox/value";

import type { InferQueueMessage, SupportedQueues } from "./types";
import { queueSchemas } from "./config";

export type MessageHandler<T extends SupportedQueues> = (
  message: InferQueueMessage<T>,
  originalMessage: ConsumeMessage,
) => Promise<void> | void;

export async function subscribeToQueue<T extends SupportedQueues>(
  channel: Channel,
  queue: T,
  handler: MessageHandler<T>,
  options: {
    noAck?: boolean;
    exclusive?: boolean;
    priority?: number;
    prefetch?: number;
  } = {},
): Promise<void> {
  if (options.prefetch) {
    await channel.prefetch(options.prefetch);
  }

  const schema = queueSchemas[queue];

  await channel.consume(
    queue,
    (msg) => {
      if (!msg) return;

      void (async () => {
        try {
          const content = JSON.parse(msg.content.toString()) as unknown;

          if (!Value.Check(schema, content)) {
            const errors = [...Value.Errors(schema, content)];
            console.error(`Invalid message received for queue ${queue}:`, errors);

            if (!options.noAck) {
              channel.nack(msg, false, false);
            }
            return;
          }

          await handler(content as InferQueueMessage<T>, msg);

          if (!options.noAck) {
            channel.ack(msg);
          }
        } catch (error) {
          console.error(`Error processing message from queue ${queue}:`, error);

          if (!options.noAck) {
            channel.nack(msg, false, true);
          }
        }
      })();
    },
    {
      noAck: options.noAck ?? false,
      exclusive: options.exclusive ?? false,
      priority: options.priority,
    },
  );
}

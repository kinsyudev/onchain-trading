export {
  connectToRabbitMQ,
  assertQueues,
  disconnect,
  type RabbitMQClient,
} from "./connection";
export { sendMessageToQueue } from "./producer";
export { subscribeToQueue, type MessageHandler } from "./consumer";
export type { SupportedQueues, InferQueueMessage, QueueConfig } from "./types";

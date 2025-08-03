import type { RabbitMQClient } from "@kinsyu/messaging";
import {
  connectToRabbitMQ,
  disconnect,
  subscribeToQueue,
} from "@kinsyu/messaging";

import { env } from "./env";

let rabbitMQClient: RabbitMQClient | null = null;
let isShuttingDown = false;

async function gracefulShutdown(signal: string) {
  if (isShuttingDown) return;
  isShuttingDown = true;

  console.log(`\nReceived ${signal}, starting graceful shutdown...`);

  try {
    if (rabbitMQClient) {
      console.log("Closing RabbitMQ connection...");
      await disconnect(rabbitMQClient);
      console.log("RabbitMQ connection closed successfully");
    }
  } catch (error) {
    console.error("Error during shutdown:", error);
  } finally {
    process.exit(0);
  }
}

async function startConsumer() {
  try {
    rabbitMQClient = await connectToRabbitMQ(env.RABBITMQ_URL);

    console.log("Connected to RabbitMQ successfully");

    // Example: Subscribe to a queue
    // Replace with your actual queue name and message handler
    await subscribeToQueue({
      channel: rabbitMQClient.channel,
      queue: "uniswap-v2-swaps",
      handler: async (_message, _originalMessage) => {
        // Your business logic here
      },
      options: {},
    });

    console.log("Consumer is now listening for messages...");
  } catch (error) {
    console.error("Failed to start consumer:", error);
    throw error;
  }
}

async function main() {
  console.log("Starting RabbitMQ consumer...");

  try {
    await startConsumer();
  } catch (error) {
    console.error("Fatal error in consumer:", error);
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on("SIGINT", () => void gracefulShutdown("SIGINT"));
process.on("SIGTERM", () => void gracefulShutdown("SIGTERM"));

// Handle uncaught errors
process.on("uncaughtException", (error) => {
  console.error("Uncaught exception:", error);
  void gracefulShutdown("uncaughtException");
});

process.on("unhandledRejection", (reason, promise) => {
  console.error("Unhandled rejection at:", promise, "reason:", reason);
  void gracefulShutdown("unhandledRejection");
});

// Start the consumer
main().catch((error) => {
  console.error("Failed to start application:", error);
  process.exit(1);
});

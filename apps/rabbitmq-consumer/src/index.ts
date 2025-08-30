import type { RabbitMQClient } from "@kinsyu/messaging";
import {
  connectToRabbitMQ,
  disconnect,
  subscribeToQueue,
} from "@kinsyu/messaging";

import { env } from "./env";
import { handleUniswapV2Swap } from "./handlers/unsiwap-v2";
import { handlePumpfunTrade } from "./handlers/pumpfun";
import { handleRaydiumTrade } from "./handlers/raydium";

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

    // Subscribe to all queues
    await Promise.all([
      // Uniswap V2 swaps queue
      subscribeToQueue({
        channel: rabbitMQClient.channel,
        queue: "uniswap-v2-swaps",
        handler: handleUniswapV2Swap,
        options: {
          noAck: false, // Manual acknowledgment for reliability
        },
      }),

      // Pumpfun trades queue
      subscribeToQueue({
        channel: rabbitMQClient.channel,
        queue: "pumpfun-trades",
        handler: handlePumpfunTrade,
        options: {
          noAck: false,
        },
      }),

      // Raydium trades queue (unified)
      subscribeToQueue({
        channel: rabbitMQClient.channel,
        queue: "raydium-trades",
        handler: handleRaydiumTrade,
        options: {
          noAck: false,
        },
      }),
    ]);

    console.log("Consumer is now listening for messages on all queues...");
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

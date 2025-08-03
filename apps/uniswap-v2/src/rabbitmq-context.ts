import type { RabbitMQClient } from "@kinsyu/messaging";
import { assertQueues, connectToRabbitMQ, disconnect } from "@kinsyu/messaging";

import { env } from "../env";

let rabbitMQClient: RabbitMQClient | null = null;
let connectionPromise: Promise<RabbitMQClient> | null = null;

export async function getRabbitMQClient(): Promise<RabbitMQClient> {
  // If already connected, return the client
  if (rabbitMQClient) {
    return rabbitMQClient;
  }

  // If connection is in progress, wait for it
  if (connectionPromise) {
    return connectionPromise;
  }

  // Start new connection
  connectionPromise = (async () => {
    try {
      // Initialize RabbitMQ connection
      const rabbitmqUrl = env.RABBITMQ_URL;
      const client = await connectToRabbitMQ(rabbitmqUrl);

      // Assert required queues
      await assertQueues(client.channel, ["uniswap-v2-swaps"]);

      // Handle connection errors and automatic reconnection
      client.channelModel.on("error", (err) => {
        console.error("RabbitMQ connection error:", err);
        rabbitMQClient = null;
        connectionPromise = null;
      });

      client.channelModel.on("close", () => {
        console.log("RabbitMQ connection closed, will reconnect on next use");
        rabbitMQClient = null;
        connectionPromise = null;
      });

      console.log("RabbitMQ client initialized successfully");
      rabbitMQClient = client;
      return client;
    } catch (error) {
      // Clear the promise on error so next call will retry
      connectionPromise = null;
      throw error;
    }
  })();

  return connectionPromise;
}

// Cleanup function for graceful shutdown
export async function cleanupRabbitMQ(): Promise<void> {
  // Wait for any pending connection attempts to complete
  if (connectionPromise) {
    try {
      await connectionPromise;
    } catch (error) {
      // Ignore connection errors during cleanup
      console.error("Connection error during cleanup:", error);
    }
  }

  if (rabbitMQClient) {
    try {
      await disconnect(rabbitMQClient);
      rabbitMQClient = null;
      connectionPromise = null;
      console.log("RabbitMQ connection closed gracefully");
    } catch (error) {
      console.error("Error closing RabbitMQ connection:", error);
    }
  }
}

// Register cleanup on process termination
process.on("SIGINT", () => {
  cleanupRabbitMQ()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error("Error during SIGINT cleanup:", error);
      process.exit(1);
    });
});

process.on("SIGTERM", () => {
  cleanupRabbitMQ()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error("Error during SIGTERM cleanup:", error);
      process.exit(1);
    });
});

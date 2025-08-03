import type { Channel, ChannelModel, Connection } from "amqplib";
import amqp from "amqplib";

import type { QueueConfig, SupportedQueues } from "./types";

export interface RabbitMQClient {
  connection: Connection;
  channel: Channel;
  channelModel: ChannelModel;
}

export async function connectToRabbitMQ(
  url = "amqp://admin:admin@localhost:5672",
): Promise<RabbitMQClient> {
  try {
    const channelModel = await amqp.connect(url);
    const channel = await channelModel.createChannel();

    channelModel.on("error", (err) => {
      console.error("RabbitMQ connection error:", err);
    });

    channelModel.on("close", () => {
      console.log("RabbitMQ connection closed");
    });

    return { connection: channelModel.connection, channel, channelModel };
  } catch (error) {
    console.error("Failed to connect to RabbitMQ:", error);
    throw error;
  }
}

export async function assertQueues(
  channel: Channel,
  queues: SupportedQueues[],
  config: QueueConfig = {
    durable: true,
    autoDelete: false,
    exclusive: false,
  },
): Promise<void> {
  for (const queue of queues) {
    await channel.assertQueue(queue, config);
  }
}

export async function disconnect(client: RabbitMQClient): Promise<void> {
  await client.channel.close();
  await client.channelModel.close();
}

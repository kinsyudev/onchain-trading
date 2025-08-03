import { ponder } from "ponder:registry";

import type { Static } from "@kinsyu/validators";
import type { UniswapV2Swap } from "@kinsyu/validators/blockchain/events/uniswap-v2";
import { sendMessageToQueue } from "@kinsyu/messaging";

import { getRabbitMQClient } from "./rabbitmq-context";

// Setup hook to initialize RabbitMQ client
ponder.on("UniswapV2Pair:setup", async () => {
  // Initialize RabbitMQ client during setup phase
  await getRabbitMQClient();

  // Store the client in context for use in event handlers
  // Note: Ponder's context is read-only in event handlers, so we'll use a module-level variable
  console.log("RabbitMQ client ready for use");
});

ponder.on("UniswapV2Pair:Swap", async ({ event, context }) => {
  // Get the RabbitMQ client
  const rabbitmqClient = await getRabbitMQClient();

  // construct payload
  const swapMessage = {
    id: event.id,
    transactionSender: event.transaction.from,
    timestamp: Number(event.block.timestamp),
    transactionHash: event.transaction.hash,
    blockNumber: Number(event.block.number),

    pair: event.log.address,
    sender: event.args.sender,
    to: event.args.to,
    amount0In: String(event.args.amount0In),
    amount0Out: String(event.args.amount0Out),
    amount1In: String(event.args.amount1In),
    amount1Out: String(event.args.amount1Out),
    chainId: context.chain.id,
  } satisfies Static<typeof UniswapV2Swap>;

  // Send message to RabbitMQ queue
  sendMessageToQueue(rabbitmqClient.channel, "uniswap-v2-swaps", swapMessage);
});

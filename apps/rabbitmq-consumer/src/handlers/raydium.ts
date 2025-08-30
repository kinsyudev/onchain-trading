import type { Message } from "amqplib";
import type { Static } from "@sinclair/typebox";
import { RaydiumTradeEventSchema } from "@kinsyu/validators/blockchain/events/raydium";

type RaydiumTradeEvent = Static<typeof RaydiumTradeEventSchema>;

export async function handleRaydiumTrade(message: Message): Promise<void> {
  try {
    const content = message.content.toString();
    const tradeEvent: RaydiumTradeEvent = JSON.parse(content);

    console.log(`🔄 Processing Raydium ${tradeEvent.swapType}:`, {
      signature: tradeEvent.signature,
      slot: tradeEvent.slot,
      poolId: tradeEvent.poolId,
      user: tradeEvent.user,
      swapType: tradeEvent.swapType,
    });

    // Handle different swap types
    if (tradeEvent.swapType === "swapBaseIn") {
      console.log("   SwapBaseIn details:", {
        amountIn: tradeEvent.amountIn,
        minimumAmountOut: tradeEvent.minimumAmountOut,
      });
    } else if (tradeEvent.swapType === "swapBaseOut") {
      console.log("   SwapBaseOut details:", {
        maxAmountIn: tradeEvent.maxAmountIn,
        amountOut: tradeEvent.amountOut,
      });
    }

    // TODO: Add your business logic here
    // Examples:
    // - Store in database
    // - Calculate price impact
    // - Update pool statistics
    // - Track user activity

    console.log("✅ Raydium trade processed successfully");
  } catch (error) {
    console.error("❌ Error processing Raydium trade:", error);
    throw error; // Re-throw to trigger retry logic
  }
}

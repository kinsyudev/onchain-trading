import type { Message } from "amqplib";
import type { Static } from "@sinclair/typebox";
import { PumpfunTradeEventSchema } from "@kinsyu/validators/blockchain/events/pumpfun";

type PumpfunTradeEvent = Static<typeof PumpfunTradeEventSchema>;

export async function handlePumpfunTrade(message: Message): Promise<void> {
  try {
    const content = message.content.toString();
    const tradeEvent: PumpfunTradeEvent = JSON.parse(content);

    console.log("🚀 Processing Pumpfun trade:", {
      signature: tradeEvent.signature,
      slot: tradeEvent.slot,
      mint: tradeEvent.mint,
      trader: tradeEvent.trader,
      solAmount: tradeEvent.solAmount,
      tokenAmount: tradeEvent.tokenAmount,
      isBuy: tradeEvent.isBuy,
    });

    // TODO: Add your business logic here
    // Examples:
    // - Store in database
    // - Send to analytics service
    // - Trigger alerts for large trades
    // - Update price feeds

    console.log("✅ Pumpfun trade processed successfully");
  } catch (error) {
    console.error("❌ Error processing Pumpfun trade:", error);
    throw error; // Re-throw to trigger retry logic
  }
}

import { Type } from "@sinclair/typebox";

export const SolanaBaseEventSchema = Type.Object({
  signature: Type.String({ description: "Transaction signature" }),
  slot: Type.Number({ description: "Slot number when the transaction was processed" }),
  timestamp: Type.Number({ description: "Unix timestamp" }),
  programId: Type.String({ description: "Program ID that generated this event" }),
});

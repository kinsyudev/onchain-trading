import { Type } from "@sinclair/typebox";

export const BaseEventSchema = Type.Object({
  id: Type.String(),
  transactionSender: Type.String(),
  timestamp: Type.Number(),
  transactionHash: Type.String(),
  blockNumber: Type.Number(),
  chainId: Type.Number(),
});

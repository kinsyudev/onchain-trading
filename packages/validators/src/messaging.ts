import { Type } from "@sinclair/typebox";

export const ExampleQueueSchema = Type.Object({
  id: Type.String(),
  message: Type.String(),
  timestamp: Type.Number(),
});

export const AnotherQueueSchema = Type.Object({
  userId: Type.String(),
  action: Type.Union([
    Type.Literal("create"),
    Type.Literal("update"),
    Type.Literal("delete"),
  ]),
  data: Type.Optional(Type.Any()),
});

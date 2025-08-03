import {
  AnotherQueueSchema,
  ExampleQueueSchema,
} from "@kinsyu/validators/messaging";

export const queueSchemas = {
  "example-queue": ExampleQueueSchema,
  "another-queue": AnotherQueueSchema,
} as const;

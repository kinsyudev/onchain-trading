import { Hono } from "hono";
import { client } from "ponder";
import { db } from "ponder:api";
import schema from "ponder:schema";

const app = new Hono();

app.use("/sql/*", client({ db, schema }));

export default app;

import assert from "node:assert/strict";
import test from "node:test";

import { ApiClient, RawApiClientForbiddenError } from "./ApiClient";

test("the legacy API client fails closed for every HTTP verb", async () => {
  const client = new ApiClient({
    baseURL: "https://invalid.example",
    headers: {},
    timeout: 1_000,
  });
  const operations = [
    () => client.get("/resource"),
    () => client.post("/resource", { value: "test" }),
    () => client.put("/resource/id", { value: "test" }),
    () => client.delete("/resource/id"),
  ];

  for (const operation of operations) {
    await assert.rejects(operation, RawApiClientForbiddenError);
  }
});

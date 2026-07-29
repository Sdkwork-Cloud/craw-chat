import assert from "node:assert/strict";
import test from "node:test";

import { ProductService } from "./ProductService";
import { ShoppingCapabilityUnavailableError } from "./ShoppingCapabilityUnavailableError";

test("product and customer-service operations fail closed", async () => {
  const message = {
    content: "Hello",
    id: "message-id",
    senderId: "user-id",
    senderType: "user" as const,
    timestamp: 0,
  };
  for (const operation of [
    () => ProductService.getProducts(),
    () => ProductService.getProductById("product-id"),
    () => ProductService.getProductsByShop("shop-id"),
    () => ProductService.getShopById("shop-id"),
    () => ProductService.getCategories(),
    () => ProductService.getCustomerServiceMessages("shop-id"),
    () => ProductService.sendCustomMessage("shop-id", message),
  ]) {
    await assert.rejects(operation, ShoppingCapabilityUnavailableError);
  }
});

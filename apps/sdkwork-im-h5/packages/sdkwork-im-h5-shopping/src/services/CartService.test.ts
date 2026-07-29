import assert from "node:assert/strict";
import test from "node:test";

import { CartService } from "./CartService";
import { ShoppingCapabilityUnavailableError } from "./ShoppingCapabilityUnavailableError";

test("cart operations fail closed", async () => {
  const product = {
    id: "product-id",
    image: "",
    price: "10.00",
    sales: "0",
    title: "Product",
  };
  for (const operation of [
    () => CartService.getCart(),
    () => CartService.addToCart(product),
    () => CartService.updateQuantity("cart-item-id", 2),
    () => CartService.toggleCheck("cart-item-id", true),
    () => CartService.toggleAllCheck(true),
    () => CartService.removeFromCart(["cart-item-id"]),
    () => CartService.clearCart(),
  ]) {
    await assert.rejects(operation, ShoppingCapabilityUnavailableError);
  }
});

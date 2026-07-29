import type { CartItem, Product, ProductSKU } from "../types";
import { ShoppingCapabilityUnavailableError } from "./ShoppingCapabilityUnavailableError";

export const CartService = {
  async getCart(): Promise<CartItem[]> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async addToCart(
    _product: Product,
    _quantity = 1,
    _sku?: ProductSKU,
    _selectedSpecs?: Record<string, string>,
  ): Promise<void> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async updateQuantity(_cartItemId: string, _quantity: number): Promise<void> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async toggleCheck(_cartItemId: string, _checked: boolean): Promise<void> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async toggleAllCheck(_checked: boolean): Promise<void> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async removeFromCart(_cartItemIds: string[]): Promise<void> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async clearCart(): Promise<void> {
    throw new ShoppingCapabilityUnavailableError();
  },
};

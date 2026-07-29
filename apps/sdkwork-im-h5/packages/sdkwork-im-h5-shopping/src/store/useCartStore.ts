import { create } from "zustand";

import type { CartItem, Product, ProductSKU } from "../types";
import { CartService } from "../services/CartService";

interface CartState {
  items: CartItem[];
  loading: boolean;
  loadCart: () => Promise<void>;
  addToCart: (
    product: Product,
    quantity?: number,
    sku?: ProductSKU,
    selectedSpecs?: Record<string, string>,
  ) => Promise<void>;
  updateQuantity: (id: string, quantity: number) => Promise<void>;
  toggleItemCheck: (id: string, checked: boolean) => Promise<void>;
  toggleAllCheck: (checked: boolean) => Promise<void>;
  removeFromCart: (ids: string[]) => Promise<void>;
  clearCart: () => Promise<void>;
  getCheckedItems: () => CartItem[];
  getTotalPrice: () => number;
}

export const useCartStore = create<CartState>((set, get) => ({
  items: [],
  loading: false,

  loadCart: async () => {
    set({ loading: true });
    try {
      set({ items: await CartService.getCart() });
    } finally {
      set({ loading: false });
    }
  },

  addToCart: async (product, quantity = 1, sku, selectedSpecs) => {
    await CartService.addToCart(product, quantity, sku, selectedSpecs);
  },

  updateQuantity: async (id, quantity) => {
    await CartService.updateQuantity(id, quantity);
  },

  toggleItemCheck: async (id, checked) => {
    await CartService.toggleCheck(id, checked);
  },

  toggleAllCheck: async (checked) => {
    await CartService.toggleAllCheck(checked);
  },

  removeFromCart: async (ids) => {
    await CartService.removeFromCart(ids);
  },

  clearCart: async () => {
    await CartService.clearCart();
  },

  getCheckedItems: () => get().items.filter((item) => item.checked),

  getTotalPrice: () =>
    get()
      .items.filter((item) => item.checked)
      .reduce((total, item) => {
        const itemPrice = item.sku?.price ?? item.product.price;
        return total + Number.parseFloat(itemPrice) * item.quantity;
      }, 0),
}));

import { create } from "zustand";
import { CartItem, Product } from "../types";
import { CartService } from "../services/CartService";

interface CartState {
  items: CartItem[];
  loading: boolean;
  loadCart: () => Promise<void>;
  addToCart: (
    product: Product,
    quantity?: number,
    sku?: import("../types").ProductSKU,
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
      const items = await CartService.getCart();
      set({ items });
    } finally {
      set({ loading: false });
    }
  },

  addToCart: async (product, quantity = 1, sku, selectedSpecs) => {
    set({ loading: true });
    try {
      await CartService.addToCart(product, quantity, sku, selectedSpecs);
      const items = await CartService.getCart();
      set({ items });
    } finally {
      set({ loading: false });
    }
  },

  updateQuantity: async (id, quantity) => {
    try {
      await CartService.updateQuantity(id, quantity);
      const items = await CartService.getCart();
      set({ items });
    } catch (e) {}
  },

  toggleItemCheck: async (id, checked) => {
    try {
      await CartService.toggleCheck(id, checked);
      const items = await CartService.getCart();
      set({ items });
    } catch (e) {}
  },

  toggleAllCheck: async (checked) => {
    try {
      await CartService.toggleAllCheck(checked);
      const items = await CartService.getCart();
      set({ items });
    } catch (e) {}
  },

  removeFromCart: async (ids) => {
    set({ loading: true });
    try {
      await CartService.removeFromCart(ids);
      const items = await CartService.getCart();
      set({ items });
    } finally {
      set({ loading: false });
    }
  },

  clearCart: async () => {
    set({ loading: true });
    try {
      await CartService.clearCart();
      const items = await CartService.getCart();
      set({ items });
    } finally {
      set({ loading: false });
    }
  },

  getCheckedItems: () => {
    return get().items.filter((i) => i.checked);
  },

  getTotalPrice: () => {
    return get()
      .items.filter((i) => i.checked)
      .reduce((acc, current) => {
        const itemPrice = current.sku?.price || current.product.price;
        return acc + parseFloat(itemPrice) * current.quantity;
      }, 0);
  },
}));

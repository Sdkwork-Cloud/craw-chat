import { CartItem, Product } from "../types";

const STORAGE_KEY = "sdkwork_im_h5_shopping_cart";

let mockCart: CartItem[] = [];

const loadCart = () => {
  if (mockCart.length > 0) return mockCart;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      mockCart = JSON.parse(data);
    }
  } catch (e) {
    mockCart = [];
  }
  return mockCart;
};

const saveCart = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(mockCart));
  } catch (e) {
    console.error("Failed to save cart", e);
  }
};

export const CartService = {
  async getCart(): Promise<CartItem[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadCart()]), 200),
    );
  },

  async addToCart(
    product: Product,
    quantity: number = 1,
    sku?: import("../types").ProductSKU,
    selectedSpecs?: Record<string, string>,
  ): Promise<void> {
    return new Promise((resolve) =>
      setTimeout(() => {
        loadCart();
        const existing = mockCart.find(
          (item) => item.productId === product.id && item.sku?.id === sku?.id,
        );
        if (existing) {
          existing.quantity += quantity;
        } else {
          mockCart.push({
            id: `cart_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
            productId: product.id,
            product,
            quantity,
            sku,
            selectedSpecs,
            addedAt: Date.now(),
            checked: true,
          });
        }
        saveCart();
        resolve();
      }, 200),
    );
  },

  async updateQuantity(cartItemId: string, quantity: number): Promise<void> {
    return new Promise((resolve) =>
      setTimeout(() => {
        loadCart();
        const existing = mockCart.find((item) => item.id === cartItemId);
        if (existing) {
          existing.quantity = Math.max(1, quantity);
          saveCart();
        }
        resolve();
      }, 100),
    );
  },

  async toggleCheck(cartItemId: string, checked: boolean): Promise<void> {
    return new Promise((resolve) =>
      setTimeout(() => {
        loadCart();
        const existing = mockCart.find((item) => item.id === cartItemId);
        if (existing) {
          existing.checked = checked;
          saveCart();
        }
        resolve();
      }, 100),
    );
  },

  async toggleAllCheck(checked: boolean): Promise<void> {
    return new Promise((resolve) =>
      setTimeout(() => {
        loadCart();
        mockCart.forEach((item) => {
          item.checked = checked;
        });
        saveCart();
        resolve();
      }, 100),
    );
  },

  async removeFromCart(cartItemIds: string[]): Promise<void> {
    return new Promise((resolve) =>
      setTimeout(() => {
        loadCart();
        mockCart = mockCart.filter((item) => !cartItemIds.includes(item.id));
        saveCart();
        resolve();
      }, 200),
    );
  },

  async clearCart(): Promise<void> {
    return new Promise((resolve) =>
      setTimeout(() => {
        mockCart = [];
        saveCart();
        resolve();
      }, 200),
    );
  },
};

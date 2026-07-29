import type { CustomerServiceMessage, Product, Shop } from "../types";
import { ShoppingCapabilityUnavailableError } from "./ShoppingCapabilityUnavailableError";

export const ProductService = {
  async getProducts(): Promise<Product[]> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async getProductById(_id: string): Promise<Product | null> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async getProductsByShop(_shopId: string): Promise<Product[]> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async getShopById(_id: string): Promise<Shop | null> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async getCategories(): Promise<string[]> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async getCustomerServiceMessages(_shopId: string): Promise<CustomerServiceMessage[]> {
    throw new ShoppingCapabilityUnavailableError();
  },

  async sendCustomMessage(
    _shopId: string,
    _message: CustomerServiceMessage,
  ): Promise<void> {
    throw new ShoppingCapabilityUnavailableError();
  },
};

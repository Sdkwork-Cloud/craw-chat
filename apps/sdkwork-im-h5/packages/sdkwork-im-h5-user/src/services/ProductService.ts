import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface Product {
  id: string;
  title: string;
  price: number;
  image: string;
  sales: string;
}

export const ProductService = {
  async getProducts(): Promise<Product[]> {
    throw new UserCapabilityUnavailableError("User product recommendations");
  },

  async getCategories(): Promise<string[]> {
    throw new UserCapabilityUnavailableError("User product recommendations");
  },
};

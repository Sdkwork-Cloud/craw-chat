/**
 * Products — fail-closed (PRD).
 *
 * Audited as a pure in-memory mock (placeholder images, fake sales figures)
 * with no owner backend SDK. The fake catalog is removed: every method throws
 * a typed `UserCapabilityUnavailableError` so consumers surface a typed
 * unavailable state instead of fabricated products.
 */
import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface Product {
  id: string;
  title: string;
  price: number;
  image: string;
  sales: string;
}

export const ProductService = {
  getProducts: async (): Promise<Product[]> => {
    throw new UserCapabilityUnavailableError("Product catalog");
  },
  getCategories: async (): Promise<string[]> => {
    throw new UserCapabilityUnavailableError("Product categories");
  },
};

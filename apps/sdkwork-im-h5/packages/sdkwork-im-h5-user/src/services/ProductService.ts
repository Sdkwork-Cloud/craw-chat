/**
 * Products — fail-closed (PRD).
 *
 * Audited as a pure in-memory mock (placeholder images, fake sales figures)
 * with no owner backend SDK. The fake catalog is removed: every method throws
 * a typed `ProductCapabilityUnavailableError` so consumers surface a typed
 * unavailable state instead of fabricated products.
 */
export interface Product {
  id: string;
  title: string;
  price: number;
  image: string;
  sales: string;
}

export class ProductCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "ProductCapabilityUnavailableError";
  }
}

export const ProductService = {
  getProducts: async (): Promise<Product[]> => {
    throw new ProductCapabilityUnavailableError("Product catalog");
  },
  getCategories: async (): Promise<string[]> => {
    throw new ProductCapabilityUnavailableError("Product categories");
  },
};

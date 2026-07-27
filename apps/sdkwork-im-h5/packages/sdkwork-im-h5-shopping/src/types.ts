export interface Address {
  id: string;
  name: string;
  phone: string;
  province: string;
  city: string;
  district: string;
  detail: string;
  isDefault: boolean;
}

export interface Shop {
  id: string;
  name: string;
  logo: string;
  fansCount: string;
  rating: string;
  description?: string;
  tags?: string[];
  isOfficial?: boolean;
}

export interface ProductSpecOption {
  id: string;
  name: string;
}

export interface ProductSpec {
  id: string;
  name: string;
  options: ProductSpecOption[];
}

export interface ProductSKU {
  id: string;
  specValues: Record<string, string>; // e.g. { "denomination": "50" }
  price: string;
  originalPrice?: string;
  stock: number;
  image?: string;
  description?: string;
}

export interface Product {
  id: string;
  title: string;
  price: string;
  originalPrice?: string;
  sales: string;
  image: string;
  images?: string[];
  description?: string;
  categoryId?: string;
  shopId?: string;
  isVirtual?: boolean;
  virtualType?: "coupon" | "service" | "game_currency" | "group_chat";
  specs?: ProductSpec[];
  skus?: ProductSKU[];
}

export interface CartItem {
  id: string; // unique cart item id
  productId: string;
  product: Product;
  quantity: number;
  sku?: ProductSKU; // The selected SKU
  selectedSpecs?: Record<string, string>; // the selected spec option ids
  addedAt: number;
  checked: boolean;
}

export interface CustomerServiceMessage {
  id: string;
  content: string;
  senderId: string;
  senderType: "user" | "agent" | "human";
  timestamp: number;
  type?: "text" | "product_card" | "order_card";
  payload?: unknown;
}

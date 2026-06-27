import type { CatalogAppSdkClient } from '@sdkwork/im-pc-core/sdk/catalogAppSdkClient';
import type { OrderAppSdkClient } from '@sdkwork/im-pc-core/sdk/orderAppSdkClient';
import {
  extractAppSdkPayload,
  extractAppSdkRecordsFromResult,
  parseMoneyAmount,
  readNumber,
  readOptionalString,
  readString,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';
import { getCatalogAppSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/catalogAppSdkClient';
import { getOrderAppSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/orderAppSdkClient';

export interface ShopCategory {
  id: string;
  name: string;
  icon: string;
}

export interface ShopProductSku {
  id: string;
  name: string;
  price: number;
  stock: number;
  specs?: Record<string, string>;
  imageUrl?: string;
}

export interface ShopProductOption {
  name: string;
  values: string[];
}

export interface ShopProduct {
  id: string;
  categoryId: string;
  title: string;
  description: string;
  price: number;
  originalPrice?: number;
  imageUrl: string;
  salesVolume: number;
  rating: number;
  tags: string[];
  options?: ShopProductOption[];
  skus?: ShopProductSku[];
  isVirtual?: boolean;
  isCoupon?: boolean;
}

export interface CartItem {
  id: string;
  productId: string;
  skuId?: string;
  quantity: number;
  selected: boolean;
}

export interface ShopOrder {
  id: string;
  items: CartItem[];
  total: number;
  status: 'pending' | 'paid' | 'shipped' | 'completed' | 'cancelled';
  createdAt: number;
  coupons?: { item: CartItem; product: ShopProduct; code: string }[];
}

export interface ShopShippingAddress {
  id: string;
  name: string;
  phone: string;
  address: string;
  isDefault: boolean;
}

export interface ShopService {
  getCategories(): Promise<ShopCategory[]>;
  getProducts(categoryId?: string): Promise<ShopProduct[]>;
  getProductById(id: string): Promise<ShopProduct | null>;
  getCart(): Promise<CartItem[]>;
  addToCart(productId: string, quantity?: number, skuId?: string): Promise<void>;
  updateCartItem(cartItemId: string, quantity: number): Promise<void>;
  toggleCartItemSelection(cartItemId: string): Promise<void>;
  toggleAllCartItems(selected: boolean): Promise<void>;
  removeCartItem(cartItemId: string): Promise<void>;
  checkout(items?: CartItem[]): Promise<{
    orderId: string;
    total: number;
    items: CartItem[];
    generatedCoupons?: { item: CartItem; product: ShopProduct; code: string }[];
  }>;
  toggleFavorite(productId: string): Promise<boolean>;
  isFavorite(productId: string): Promise<boolean>;
  getOrders(): Promise<ShopOrder[]>;
  getShippingAddresses(): Promise<ShopShippingAddress[]>;
  saveShippingAddress(
    address: Omit<ShopShippingAddress, 'id'> & { id?: string },
  ): Promise<ShopShippingAddress>;
  deleteShippingAddress(id: string): Promise<void>;
  initiatePayment(orderId: string, method: string): Promise<void>;
}

const PC_SHOP_FAVORITES_UNAVAILABLE = 'pc shop favorites contract is not available';
const PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE = 'pc shop shipping address contract is not available';
const PC_SHOP_PAYMENT_CONTRACT_UNAVAILABLE = 'pc shop payment contract is not available';

export {
  PC_SHOP_FAVORITES_UNAVAILABLE,
  PC_SHOP_PAYMENT_CONTRACT_UNAVAILABLE,
  PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE,
};

interface ShopServiceOptions {
  catalogClient?: CatalogAppSdkClient;
  orderClient?: OrderAppSdkClient;
}

const DEFAULT_CATEGORY_ICON = 'Store';

function mapCategory(record: Record<string, unknown>): ShopCategory {
  return {
    id: readString(record, 'id'),
    name: readString(record, 'name', 'title') || 'Category',
    icon: readOptionalString(record, 'icon', 'iconUrl') ?? DEFAULT_CATEGORY_ICON,
  };
}

function mapSpuToProduct(record: Record<string, unknown>): ShopProduct {
  const productType = readString(record, 'productType', 'product_type').toLowerCase();
  return {
    id: readString(record, 'id', 'spuId', 'spu_id'),
    categoryId: readString(record, 'categoryId', 'category_id'),
    title: readString(record, 'title', 'name'),
    description: readString(record, 'description', 'subtitle'),
    price: parseMoneyAmount(record.priceAmount ?? record.price_amount ?? record.price),
    originalPrice: parseMoneyAmount(record.originalPriceAmount ?? record.original_price_amount) || undefined,
    imageUrl: readString(record, 'imageUrl', 'image_url', 'coverUrl', 'cover_url'),
    salesVolume: readNumber(record, 'salesVolume', 'sales_volume'),
    rating: readNumber(record, 'rating'),
    tags: Array.isArray(record.tags)
      ? record.tags.filter((tag): tag is string => typeof tag === 'string')
      : [],
    isVirtual: productType.includes('virtual'),
    isCoupon: productType.includes('coupon'),
  };
}

function mapSkuToProductSku(record: Record<string, unknown>): ShopProductSku {
  return {
    id: readString(record, 'id', 'skuId', 'sku_id'),
    name: readString(record, 'name', 'title'),
    price: parseMoneyAmount(record.priceAmount ?? record.price_amount ?? record.price),
    stock: readNumber(record, 'stock', 'quantity'),
    imageUrl: readOptionalString(record, 'imageUrl', 'image_url'),
  };
}

function mapCartItem(record: Record<string, unknown>, selectedIds: Set<string>): CartItem {
  const id = readString(record, 'id', 'cartItemId', 'cart_item_id');
  const skuId = readString(record, 'skuId', 'sku_id');
  return {
    id,
    productId: readString(record, 'productId', 'product_id', 'spuId', 'spu_id') || skuId,
    skuId: skuId || undefined,
    quantity: Math.max(1, readNumber(record, 'quantity')),
    selected: selectedIds.size === 0 ? true : selectedIds.has(id),
  };
}

function mapConsumerOrderStatus(rawStatus: string): ShopOrder['status'] {
  const normalized = rawStatus.trim().toLowerCase();
  if (normalized.includes('cancel')) {
    return 'cancelled';
  }
  if (normalized.includes('complete') || normalized.includes('finished')) {
    return 'completed';
  }
  if (normalized.includes('ship') || normalized.includes('fulfill')) {
    return 'shipped';
  }
  if (normalized.includes('paid') || normalized.includes('pay_success')) {
    return 'paid';
  }
  return 'pending';
}

function mapConsumerOrder(record: Record<string, unknown>): ShopOrder {
  const orderId = readString(record, 'orderId', 'order_id', 'id');
  return {
    id: orderId,
    items: [],
    total: parseMoneyAmount(record.totalAmount ?? record.total_amount ?? record.payableAmount),
    status: mapConsumerOrderStatus(readString(record, 'status', 'statusName', 'status_name')),
    createdAt: Date.parse(readString(record, 'createdAt', 'created_at')) || 0,
  };
}

class SdkworkShopService implements ShopService {
  private readonly selectedCartItemIds = new Set<string>();

  constructor(private readonly options: ShopServiceOptions = {}) {}

  private catalogClient(): CatalogAppSdkClient {
    return this.options.catalogClient ?? getCatalogAppSdkClientWithSession();
  }

  private orderClient(): OrderAppSdkClient {
    return this.options.orderClient ?? getOrderAppSdkClientWithSession();
  }

  async getCategories(): Promise<ShopCategory[]> {
    const result = await this.catalogClient().catalog.categories.list({ pageSize: 100 });
    return extractAppSdkRecordsFromResult(result).map(mapCategory);
  }

  async getProducts(categoryId?: string): Promise<ShopProduct[]> {
    const result = await this.catalogClient().catalog.products.list({
      categoryId,
      pageSize: 100,
      status: 'active',
    });
    return extractAppSdkRecordsFromResult(result).map(mapSpuToProduct);
  }

  async getProductById(id: string): Promise<ShopProduct | null> {
    const normalizedId = id.trim();
    if (!normalizedId) {
      return null;
    }

    try {
      const productResult = await this.catalogClient().catalog.products.retrieve(normalizedId);
      const productRecord = extractAppSdkPayload(productResult);
      if (productRecord && typeof productRecord === 'object' && !Array.isArray(productRecord)) {
        const product = mapSpuToProduct(productRecord as Record<string, unknown>);
        try {
          const skuResult = await this.catalogClient().catalog.skus.retrieve(normalizedId);
          const skuRecord = extractAppSdkPayload(skuResult);
          if (skuRecord && typeof skuRecord === 'object' && !Array.isArray(skuRecord)) {
            const sku = mapSkuToProductSku(skuRecord as Record<string, unknown>);
            product.skus = [sku];
            if (!product.price) {
              product.price = sku.price;
            }
          }
        } catch {
          // Product ids and sku ids may differ; keep the spu snapshot when sku lookup fails.
        }
        return product;
      }
    } catch {
      // Fall through to sku-only lookup.
    }

    try {
      const skuResult = await this.catalogClient().catalog.skus.retrieve(normalizedId);
      const skuRecord = extractAppSdkPayload(skuResult);
      if (!skuRecord || typeof skuRecord !== 'object' || Array.isArray(skuRecord)) {
        return null;
      }
      const sku = mapSkuToProductSku(skuRecord as Record<string, unknown>);
      return {
        id: normalizedId,
        categoryId: readString(skuRecord as Record<string, unknown>, 'categoryId', 'category_id'),
        title: sku.name,
        description: '',
        price: sku.price,
        imageUrl: sku.imageUrl ?? '',
        salesVolume: 0,
        rating: 0,
        tags: [],
        skus: [sku],
      };
    } catch {
      return null;
    }
  }

  async getCart(): Promise<CartItem[]> {
    const result = await this.catalogClient().cart.current.retrieve();
    return extractAppSdkRecordsFromResult(result).map((record) =>
      mapCartItem(record, this.selectedCartItemIds),
    );
  }

  async addToCart(productId: string, quantity = 1, skuId?: string): Promise<void> {
    const resolvedSkuId = (skuId ?? productId).trim();
    if (!resolvedSkuId) {
      throw new Error('A sku id is required to add catalog cart items.');
    }
    await this.catalogClient().cart.items.create({
      skuId: resolvedSkuId,
      quantity: Math.max(1, quantity),
    });
  }

  async updateCartItem(cartItemId: string, quantity: number): Promise<void> {
    await this.catalogClient().cart.items.update(cartItemId, {
      quantity: Math.max(1, quantity),
    });
  }

  async toggleCartItemSelection(cartItemId: string): Promise<void> {
    if (this.selectedCartItemIds.has(cartItemId)) {
      this.selectedCartItemIds.delete(cartItemId);
      return;
    }
    this.selectedCartItemIds.add(cartItemId);
  }

  async toggleAllCartItems(selected: boolean): Promise<void> {
    const cartItems = await this.getCart();
    this.selectedCartItemIds.clear();
    if (selected) {
      for (const item of cartItems) {
        this.selectedCartItemIds.add(item.id);
      }
    }
  }

  async removeCartItem(cartItemId: string): Promise<void> {
    await this.catalogClient().cart.items.delete(cartItemId);
    this.selectedCartItemIds.delete(cartItemId);
  }

  async checkout(items?: CartItem[]): Promise<{
    orderId: string;
    total: number;
    items: CartItem[];
    generatedCoupons?: { item: CartItem; product: ShopProduct; code: string }[];
  }> {
    const cartItems = items ?? (await this.getCart()).filter((item) => item.selected);
    const checkoutLines = cartItems
      .map((item) => ({
        skuId: (item.skuId ?? item.productId).trim(),
        quantity: Math.max(1, item.quantity),
      }))
      .filter((line) => line.skuId.length > 0);

    if (checkoutLines.length === 0) {
      throw new Error('Select at least one cart item before checkout.');
    }

    const sessionResult = await this.orderClient().checkout.sessions.create({
      items: checkoutLines,
      currencyCode: 'CNY',
    });
    const sessionRecord = extractAppSdkPayload(sessionResult);
    const sessionId = readString(sessionRecord as Record<string, unknown>, 'checkoutSessionId', 'checkout_session_id');
    if (!sessionId) {
      throw new Error('Commerce checkout session id is missing from the SDK response.');
    }

    const orderResult = await this.orderClient().checkout.sessions.orders.create(sessionId, {});
    const orderRecord = extractAppSdkPayload(orderResult);
    const orderId = readString(orderRecord as Record<string, unknown>, 'orderId', 'order_id', 'id');
    const total = parseMoneyAmount(
      (orderRecord as Record<string, unknown> | null)?.totalAmount
        ?? (orderRecord as Record<string, unknown> | null)?.total_amount
        ?? (sessionRecord as Record<string, unknown> | null)?.payableAmount
        ?? (sessionRecord as Record<string, unknown> | null)?.payable_amount,
    );

    return {
      orderId,
      total,
      items: cartItems,
    };
  }

  async toggleFavorite(_productId: string): Promise<boolean> {
    throw new Error(PC_SHOP_FAVORITES_UNAVAILABLE);
  }

  async isFavorite(_productId: string): Promise<boolean> {
    return false;
  }

  async getOrders(): Promise<ShopOrder[]> {
    const result = await this.orderClient().orders.list({ pageSize: 100 });
    return extractAppSdkRecordsFromResult(result).map(mapConsumerOrder);
  }

  async getShippingAddresses(): Promise<ShopShippingAddress[]> {
    return [];
  }

  async saveShippingAddress(
    _address: Omit<ShopShippingAddress, 'id'> & { id?: string },
  ): Promise<ShopShippingAddress> {
    throw new Error(PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE);
  }

  async deleteShippingAddress(_id: string): Promise<void> {
    throw new Error(PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE);
  }

  async initiatePayment(_orderId: string, _method: string): Promise<void> {
    throw new Error(PC_SHOP_PAYMENT_CONTRACT_UNAVAILABLE);
  }
}

export function createShopService(options: ShopServiceOptions = {}): ShopService {
  return new SdkworkShopService(options);
}

export const shopService = createShopService();

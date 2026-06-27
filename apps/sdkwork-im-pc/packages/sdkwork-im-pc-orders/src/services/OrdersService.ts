import type { OrderAppSdkClient } from '@sdkwork/im-pc-core/sdk/orderAppSdkClient';
import type { ShopAppSdkClient } from '@sdkwork/im-pc-core/sdk/shopAppSdkClient';
import {
  extractAppSdkPayload,
  extractAppSdkRecordsFromResult,
  parseMoneyAmount,
  readNumber,
  readOptionalString,
  readString,
} from '@sdkwork/im-pc-core/sdk/appSdkResponseHelpers';
import { getOrderAppSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/orderAppSdkClient';
import { getShopAppSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/shopAppSdkClient';

export interface OrderItem {
  id: string;
  productName: string;
  price: number;
  quantity: number;
  imageUrl: string;
}

export interface Order {
  id: string;
  createTime: string;
  customerName: string;
  productInfo: string;
  items?: OrderItem[];
  amount: number;
  status: 'PENDING_PAY' | 'PENDING_SHIP' | 'SHIPPED' | 'COMPLETED' | 'CANCELLED';
}

export interface OrderStats {
  pendingPayAmount: number;
  pendingPayCount: number;
  completedTodayCount: number;
  completedComparedToYesterday: number;
  pendingProcessCount: number;
  pendingTimeoutCount: number;
  monthlyRevenueAmount: number;
  monthlyRevenueCount: number;
}

export interface OrdersService {
  getOrders(): Promise<Order[]>;
  getOrderById(id: string): Promise<Order | null>;
  updateOrderStatus(id: string, status: Order['status']): Promise<Order>;
  getStats(): Promise<OrderStats>;
  createOrder(order: Partial<Order>): Promise<Order>;
  deleteOrder(id: string): Promise<void>;
}

const PC_ORDERS_WRITE_UNAVAILABLE = 'pc orders write contract requires order command headers';

const EMPTY_ORDER_STATS: OrderStats = {
  pendingPayAmount: 0,
  pendingPayCount: 0,
  completedTodayCount: 0,
  completedComparedToYesterday: 0,
  pendingProcessCount: 0,
  pendingTimeoutCount: 0,
  monthlyRevenueAmount: 0,
  monthlyRevenueCount: 0,
};

interface OrdersServiceOptions {
  orderClient?: OrderAppSdkClient;
  shopClient?: ShopAppSdkClient;
}

function mapMerchantOrderStatus(rawStatus: string): Order['status'] {
  const normalized = rawStatus.trim().toLowerCase();
  if (normalized.includes('cancel')) {
    return 'CANCELLED';
  }
  if (normalized.includes('complete') || normalized.includes('finished')) {
    return 'COMPLETED';
  }
  if (normalized.includes('ship') || normalized.includes('fulfill') || normalized.includes('service')) {
    return 'SHIPPED';
  }
  if (normalized.includes('paid') || normalized.includes('awaiting_ship')) {
    return 'PENDING_SHIP';
  }
  return 'PENDING_PAY';
}

function mapOrderItem(record: Record<string, unknown>): OrderItem {
  return {
    id: readString(record, 'id', 'orderItemId', 'order_item_id'),
    productName: readString(record, 'productName', 'product_name', 'title', 'subject'),
    price: parseMoneyAmount(record.unitPrice ?? record.unit_price ?? record.price),
    quantity: Math.max(1, readNumber(record, 'quantity')),
    imageUrl: readString(record, 'imageUrl', 'image_url', 'productImage', 'product_image'),
  };
}

function mapMerchantOrder(record: Record<string, unknown>): Order {
  const items = Array.isArray(record.items)
    ? record.items
        .map((entry) => (typeof entry === 'object' && entry != null ? mapOrderItem(entry as Record<string, unknown>) : null))
        .filter((entry): entry is OrderItem => entry != null)
    : undefined;

  const productInfo =
    readOptionalString(record, 'subject', 'productInfo', 'product_info')
    ?? items?.map((item) => item.productName).filter(Boolean).join(', ')
    ?? 'Commerce order';

  return {
    id: readString(record, 'id', 'orderId', 'order_id'),
    createTime: readString(record, 'createdAt', 'created_at', 'createTime'),
    customerName: readString(
      record,
      'customerName',
      'customer_name',
      'buyerName',
      'buyer_name',
      'ownerUserId',
      'owner_user_id',
    ) || 'Customer',
    productInfo,
    items,
    amount: parseMoneyAmount(record.totalAmount ?? record.total_amount ?? record.payableAmount ?? record.amount),
    status: mapMerchantOrderStatus(readString(record, 'status', 'paymentStatus', 'payment_status')),
  };
}

function mapStatisticsToOrderStats(record: Record<string, unknown>): OrderStats {
  return {
    pendingPayAmount: parseMoneyAmount(record.pendingPayAmount ?? record.pending_pay_amount),
    pendingPayCount: readNumber(record, 'pendingPayCount', 'pending_pay_count'),
    completedTodayCount: readNumber(record, 'completedTodayCount', 'completed_today_count'),
    completedComparedToYesterday: readNumber(
      record,
      'completedComparedToYesterday',
      'completed_compared_to_yesterday',
    ),
    pendingProcessCount: readNumber(record, 'pendingProcessCount', 'pending_process_count', 'fulfillmentPendingCount'),
    pendingTimeoutCount: readNumber(record, 'pendingTimeoutCount', 'pending_timeout_count'),
    monthlyRevenueAmount: parseMoneyAmount(
      record.monthlyRevenueAmount ?? record.monthly_revenue_amount ?? record.grossSalesAmount,
    ),
    monthlyRevenueCount: readNumber(record, 'monthlyRevenueCount', 'monthly_revenue_count', 'paidOrderCount'),
  };
}

class SdkworkOrdersService implements OrdersService {
  constructor(private readonly options: OrdersServiceOptions = {}) {}

  private orderClient(): OrderAppSdkClient {
    return this.options.orderClient ?? getOrderAppSdkClientWithSession();
  }

  private shopClient(): ShopAppSdkClient {
    return this.options.shopClient ?? getShopAppSdkClientWithSession();
  }

  private async listMerchantOrders(): Promise<Order[]> {
    const result = await this.shopClient().shops.current.orders.list({ pageSize: 100 });
    return extractAppSdkRecordsFromResult(result).map(mapMerchantOrder);
  }

  private async listConsumerOrders(): Promise<Order[]> {
    const result = await this.orderClient().orders.list({ pageSize: 100 });
    return extractAppSdkRecordsFromResult(result).map((record) => ({
      ...mapMerchantOrder(record),
      customerName: readString(record, 'ownerUserId', 'owner_user_id') || 'Me',
    }));
  }

  async getOrders(): Promise<Order[]> {
    try {
      const merchantOrders = await this.listMerchantOrders();
      if (merchantOrders.length > 0) {
        return merchantOrders;
      }
    } catch {
      // Fall back to consumer-owned orders when the current principal has no shop context.
    }
    return this.listConsumerOrders();
  }

  async getOrderById(id: string): Promise<Order | null> {
    const normalizedId = id.trim();
    if (!normalizedId) {
      return null;
    }

    try {
      const merchantResult = await this.shopClient().shops.current.orders.retrieve(normalizedId);
      const merchantRecord = extractAppSdkPayload(merchantResult);
      if (merchantRecord && typeof merchantRecord === 'object' && !Array.isArray(merchantRecord)) {
        return mapMerchantOrder(merchantRecord as Record<string, unknown>);
      }
    } catch {
      // Fall through to consumer order lookup.
    }

    try {
      const consumerResult = await this.orderClient().orders.retrieve(normalizedId);
      const consumerRecord = extractAppSdkPayload(consumerResult);
      if (consumerRecord && typeof consumerRecord === 'object' && !Array.isArray(consumerRecord)) {
        return mapMerchantOrder(consumerRecord as Record<string, unknown>);
      }
    } catch {
      return null;
    }

    return null;
  }

  async updateOrderStatus(_id: string, _status: Order['status']): Promise<Order> {
    throw new Error(PC_ORDERS_WRITE_UNAVAILABLE);
  }

  async getStats(): Promise<OrderStats> {
    try {
      const statisticsResult = await this.orderClient().orders.statistics.retrieve();
      const statisticsRecord = extractAppSdkPayload(statisticsResult);
      if (statisticsRecord && typeof statisticsRecord === 'object' && !Array.isArray(statisticsRecord)) {
        return mapStatisticsToOrderStats(statisticsRecord as Record<string, unknown>);
      }
    } catch {
      // Fall through to dashboard metrics.
    }

    try {
      const dashboardResult = await this.shopClient().shops.current.dashboard.retrieve();
      const dashboardRecord = extractAppSdkPayload(dashboardResult);
      if (dashboardRecord && typeof dashboardRecord === 'object' && !Array.isArray(dashboardRecord)) {
        return mapStatisticsToOrderStats(dashboardRecord as Record<string, unknown>);
      }
    } catch {
      return { ...EMPTY_ORDER_STATS };
    }

    return { ...EMPTY_ORDER_STATS };
  }

  async createOrder(_order: Partial<Order>): Promise<Order> {
    throw new Error(PC_ORDERS_WRITE_UNAVAILABLE);
  }

  async deleteOrder(_id: string): Promise<void> {
    throw new Error(PC_ORDERS_WRITE_UNAVAILABLE);
  }
}

export function createOrdersService(options: OrdersServiceOptions = {}): OrdersService {
  return new SdkworkOrdersService(options);
}

export const ordersService = createOrdersService();

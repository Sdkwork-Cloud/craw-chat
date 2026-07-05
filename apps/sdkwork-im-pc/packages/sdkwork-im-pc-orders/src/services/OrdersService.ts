import type { OrderAppSdkClient } from '@sdkwork/im-pc-core/sdk/orderAppSdkClient';
import type { ShopAppSdkClient } from '@sdkwork/im-pc-core/sdk/shopAppSdkClient';
import {
  extractAppSdkPayload,
  mapAppSdkOffsetPage,
  type OffsetListPage,
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

export type OrdersListPage = OffsetListPage<Order>;

export interface OrdersService {
  listOrdersPage(page: number, pageSize: number, status?: Order['status'] | 'ALL'): Promise<OrdersListPage>;
  getOrderById(id: string): Promise<Order | null>;
  updateOrderStatus(id: string, status: Order['status']): Promise<Order>;
  payOrder(id: string): Promise<Order>;
  getStats(): Promise<OrderStats>;
}

const PC_ORDERS_CREATE_UNAVAILABLE = 'pc orders create contract is not available in merchant console';
const PC_ORDERS_DELETE_UNAVAILABLE = 'pc orders delete contract is not available';
const PC_ORDERS_COMPLETE_UNAVAILABLE =
  'order completion is driven by fulfillment lifecycle; no direct completion command is exposed';

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

const COMMERCE_COMMAND = {};

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

function mapWireStatusFilter(status?: Order['status'] | 'ALL'): string | undefined {
  if (!status || status === 'ALL') {
    return undefined;
  }
  switch (status) {
    case 'PENDING_PAY':
      return 'pending_pay';
    case 'PENDING_SHIP':
      return 'pending_ship';
    case 'SHIPPED':
      return 'shipped';
    case 'COMPLETED':
      return 'completed';
    case 'CANCELLED':
      return 'cancelled';
    default:
      return undefined;
  }
}

class SdkworkOrdersService implements OrdersService {
  constructor(private readonly options: OrdersServiceOptions = {}) {}

  private orderClient(): OrderAppSdkClient {
    return this.options.orderClient ?? getOrderAppSdkClientWithSession();
  }

  private shopClient(): ShopAppSdkClient {
    return this.options.shopClient ?? getShopAppSdkClientWithSession();
  }

  private async listMerchantOrdersPage(
    page: number,
    pageSize: number,
    status?: Order['status'] | 'ALL',
  ): Promise<OrdersListPage> {
    const result = await this.shopClient().shops.current.orders.list({
      page,
      pageSize,
      ...(mapWireStatusFilter(status) ? { status: mapWireStatusFilter(status) } : {}),
    });
    return mapAppSdkOffsetPage(result, mapMerchantOrder, page, pageSize);
  }

  private async listConsumerOrdersPage(
    page: number,
    pageSize: number,
    status?: Order['status'] | 'ALL',
  ): Promise<OrdersListPage> {
    const result = await this.orderClient().orders.list({
      page,
      pageSize,
      ...(mapWireStatusFilter(status) ? { status: mapWireStatusFilter(status) } : {}),
    });
    return mapAppSdkOffsetPage(result, (record) => ({
      ...mapMerchantOrder(record),
      customerName: readString(record, 'ownerUserId', 'owner_user_id') || 'Me',
    }), page, pageSize);
  }

  async listOrdersPage(
    page: number,
    pageSize: number,
    status: Order['status'] | 'ALL' = 'ALL',
  ): Promise<OrdersListPage> {
    const normalizedPage = Math.max(1, page);
    const normalizedPageSize = Math.min(Math.max(pageSize, 1), 200);

    try {
      const merchantPage = await this.listMerchantOrdersPage(normalizedPage, normalizedPageSize, status);
      if (merchantPage.items.length > 0 || normalizedPage > 1) {
        return merchantPage;
      }
    } catch {
      // Fall back to consumer-owned orders when the current principal has no shop context.
    }

    return this.listConsumerOrdersPage(normalizedPage, normalizedPageSize, status);
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

  async updateOrderStatus(id: string, status: Order['status']): Promise<Order> {
    const normalizedId = id.trim();
    if (!normalizedId) {
      throw new Error('Order id is required');
    }

    switch (status) {
      case 'CANCELLED':
        try {
          await this.orderClient().orders.cancel(normalizedId, COMMERCE_COMMAND);
        } catch {
          await this.orderClient().orders.cancellations.create(normalizedId, COMMERCE_COMMAND);
        }
        break;
      case 'SHIPPED':
        await this.shopClient().shops.current.orders.fulfillments.create(normalizedId, COMMERCE_COMMAND);
        break;
      case 'COMPLETED':
        throw new Error(PC_ORDERS_COMPLETE_UNAVAILABLE);
      default:
        throw new Error(`Unsupported order status transition: ${status}`);
    }

    const refreshed = await this.getOrderById(normalizedId);
    if (!refreshed) {
      throw new Error('Order command accepted but refresh failed');
    }
    return refreshed;
  }

  async payOrder(id: string): Promise<Order> {
    const normalizedId = id.trim();
    if (!normalizedId) {
      throw new Error('Order id is required');
    }
    await this.orderClient().orders.pay(normalizedId, COMMERCE_COMMAND);
    const refreshed = await this.getOrderById(normalizedId);
    if (!refreshed) {
      throw new Error('Payment command accepted but refresh failed');
    }
    return refreshed;
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
}

export function createOrdersService(options: OrdersServiceOptions = {}): OrdersService {
  return new SdkworkOrdersService(options);
}

export const ordersService = createOrdersService();

export {
  PC_ORDERS_CREATE_UNAVAILABLE,
  PC_ORDERS_DELETE_UNAVAILABLE,
  PC_ORDERS_COMPLETE_UNAVAILABLE,
};

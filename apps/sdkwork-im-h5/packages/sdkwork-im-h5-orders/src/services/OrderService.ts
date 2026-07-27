export interface Order {
  id: string;
  shopName: string;
  isVirtual?: boolean;
  status:
    | "pending_payment"
    | "to_ship"
    | "to_receive"
    | "to_review"
    | "completed"
    | "cancelled"
    | "refunded";
  statusText: string;
  items: {
    id: string;
    image: string;
    title: string;
    specs: string;
    price: number;
    originalPrice?: number;
    quantity: number;
    virtualType?: "coupon" | "service" | "game_currency" | "group_chat";
    voucherCodes?: { code: string; status: "unused" | "used" }[];
  }[];
  totalAmount: number;
  shippingFee: number;
  address?: {
    name: string;
    phone: string;
    detail: string;
  };
  createTime: string;
  payTime?: string;
  shipTime?: string;
}

const INITIAL_ORDERS: Order[] = [
  {
    id: "ORD1234567890",
    shopName: "Apple 产品京东自营旗舰店",
    status: "pending_payment",
    statusText: "等待买家付款",
    items: [
      {
        id: "p1",
        image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/apple/200x200.png",
        title:
          "Apple iPhone 15 Pro (A2849) 256GB 原色钛金属 支持移动联通电信5G 双卡双待手机",
        specs: "原色钛金属, 256GB",
        price: 8999.0,
        quantity: 1,
      },
    ],
    totalAmount: 8999.0,
    shippingFee: 0,
    address: {
      name: "张三",
      phone: "138****8888",
      detail: "北京市海淀区中关村大街1号海龙大厦10层",
    },
    createTime: "2023-10-25 14:30:00",
  },
  {
    id: "ORD1234567891",
    shopName: "三只松鼠旗舰店",
    status: "to_ship",
    statusText: "买家已付款",
    items: [
      {
        id: "p2",
        image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/nuts/200x200.png",
        title: "三只松鼠 坚果大礼包 每日坚果 零食大礼包 送礼 混合坚果 1428g",
        specs: "1428g 豪华版",
        price: 129.0,
        originalPrice: 299.0,
        quantity: 2,
      },
    ],
    totalAmount: 258.0,
    shippingFee: 0,
    address: {
      name: "李四",
      phone: "139****9999",
      detail: "上海市浦东新区世纪大道100号上海环球金融中心",
    },
    createTime: "2023-10-24 09:15:00",
    payTime: "2023-10-24 09:16:30",
  },
  {
    id: "ORD1234567892",
    shopName: "优衣库官方旗舰店",
    status: "to_receive",
    statusText: "卖家已发货",
    items: [
      {
        id: "p3",
        image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/shirt/200x200.png",
        title: "男装/女装 摇粒绒拉链夹克(长袖 抓绒 外套) 461335",
        specs: "藏青色, L",
        price: 199.0,
        quantity: 1,
      },
      {
        id: "p4",
        image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/pants/200x200.png",
        title: "男装 弹力九分裤(棉质 休闲裤) 455486",
        specs: "黑色, L",
        price: 249.0,
        quantity: 1,
      },
    ],
    totalAmount: 448.0,
    shippingFee: 0,
    address: {
      name: "王五",
      phone: "137****7777",
      detail: "广东省深圳市南山区深南大道10000号腾讯大厦",
    },
    createTime: "2023-10-20 18:00:00",
    payTime: "2023-10-20 18:05:00",
    shipTime: "2023-10-21 10:00:00",
  },
  {
    id: "ORD1234567893",
    shopName: "小米官方旗舰店",
    status: "to_review",
    statusText: "交易成功",
    items: [
      {
        id: "p5",
        image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/xiaomi/200x200.png",
        title:
          "小米手环 8 Pro 智能手环 运动手环 腕表级健康监测 独立GNSS五星定位 1.74英寸AMOLED大屏",
        specs: "夜跃黑, 标准版",
        price: 399.0,
        quantity: 1,
      },
    ],
    totalAmount: 399.0,
    shippingFee: 0,
    address: {
      name: "赵六",
      phone: "136****6666",
      detail: "浙江省杭州市余杭区文一西路969号阿里巴巴西溪园区",
    },
    createTime: "2023-10-15 11:20:00",
    payTime: "2023-10-15 11:21:00",
    shipTime: "2023-10-15 16:00:00",
  },
];

const STORAGE_KEY = "sdkwork_im_h5_orders";

let MOCK_ORDERS: Order[] = [];

const loadOrders = () => {
  if (MOCK_ORDERS.length > 0) return MOCK_ORDERS;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_ORDERS = JSON.parse(data);
    } else {
      MOCK_ORDERS = [...INITIAL_ORDERS];
      saveOrders();
    }
  } catch (e) {
    MOCK_ORDERS = [...INITIAL_ORDERS];
  }
  return MOCK_ORDERS;
};

const saveOrders = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_ORDERS));
  } catch (e) {
    console.error("Failed to save orders data", e);
  }
};

// 模拟网络请求延迟
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export class OrderService {
  static async getOrders(): Promise<Order[]> {
    await delay(300);
    // 返回浅拷贝确保数组刷新
    return [...loadOrders()];
  }

  static async getOrderTabs(): Promise<{ id: string; label: string }[]> {
    return [
      { id: "all", label: "全部" },
      { id: "pending_payment", label: "待付款" },
      { id: "to_ship", label: "待发货" },
      { id: "to_receive", label: "待收货" },
      { id: "to_review", label: "待评价" },
    ];
  }

  static async getOrderById(id: string): Promise<Order | undefined> {
    await delay(200);
    return loadOrders().find((o) => o.id === id);
  }

  static async payOrder(id: string): Promise<void> {
    await delay(500);
    loadOrders();
    const order = MOCK_ORDERS.find((o) => o.id === id);
    if (!order) throw new Error("Order not found");
    
    if (order.isVirtual) {
      order.status = "to_review";
      order.statusText = "交易成功(发码成功)";
      order.items = order.items.map(item => {
        if (item.virtualType === "coupon") {
          const codes = [];
          for (let i = 0; i < item.quantity; i++) {
            codes.push({
              code: Math.random().toString(36).substr(2, 8).toUpperCase() + '-' + Math.random().toString(36).substr(2, 4).toUpperCase(),
              status: "unused" as const
            });
          }
          return {
            ...item,
            voucherCodes: codes
          };
        }
        return item;
      });
    } else {
      order.status = "to_ship";
      order.statusText = "买家已付款";
    }
    
    order.payTime = new Date().toLocaleString("zh-CN", { hour12: false });
    saveOrders();
  }

  static async redeemVoucher(code: string): Promise<{ success: boolean; message: string; order?: Order }> {
    await delay(500);
    loadOrders();
    for (const order of MOCK_ORDERS) {
      if (order.isVirtual) {
        for (const item of order.items) {
          if (item.voucherCodes) {
            for (const voucher of item.voucherCodes) {
              if (voucher.code === code) {
                if (voucher.status === "used") {
                  return { success: false, message: "该券码已被核销" };
                }
                voucher.status = "used";
                saveOrders();
                return { success: true, message: "核销成功", order };
              }
            }
          }
        }
      }
    }
    return { success: false, message: "未找到该券码或券码无效" };
  }

  static async cancelOrder(id: string): Promise<void> {
    await delay(400);
    loadOrders();
    const order = MOCK_ORDERS.find((o) => o.id === id);
    if (!order) throw new Error("Order not found");
    order.status = "cancelled";
    order.statusText = "交易关闭";
    saveOrders();
  }

  static async confirmReceipt(id: string): Promise<void> {
    await delay(400);
    loadOrders();
    const order = MOCK_ORDERS.find((o) => o.id === id);
    if (!order) throw new Error("Order not found");
    order.status = "to_review";
    order.statusText = "交易成功";
    saveOrders();
  }

  static async reviewOrder(id: string): Promise<void> {
    await delay(400);
    loadOrders();
    const order = MOCK_ORDERS.find((o) => o.id === id);
    if (!order) throw new Error("Order not found");
    order.status = "completed";
    order.statusText = "交易已完成";
    saveOrders();
  }

  static async remindShipping(id: string): Promise<void> {
    await delay(300);
    // 模拟催发货成功
  }

  static async applyRefund(id: string): Promise<void> {
    await delay(400);
    loadOrders();
    const order = MOCK_ORDERS.find((o) => o.id === id);
    if (!order) throw new Error("Order not found");
    order.status = "refunded";
    order.statusText = "已退款";
    saveOrders();
  }

  static async modifyAddress(id: string): Promise<void> {
    await delay(400);
    // 模拟纯UI操作成功
  }

  static async deleteOrder(id: string): Promise<void> {
    await delay(300);
    loadOrders();
    const index = MOCK_ORDERS.findIndex((o) => o.id === id);
    if (index !== -1) {
      MOCK_ORDERS.splice(index, 1);
      saveOrders();
    }
  }

  static async createOrder(
    order: Omit<Order, "id" | "createTime" | "status" | "statusText">,
  ): Promise<Order> {
    await delay(300);
    loadOrders();
    const newOrder: Order = {
      ...order,
      id: `ORD${Date.now()}`,
      status: "pending_payment",
      statusText: "等待买家付款",
      createTime: new Date().toLocaleString("zh-CN", { hour12: false }),
    };
    MOCK_ORDERS.unshift(newOrder); // Add to the beginning
    saveOrders();
    return newOrder;
  }
}

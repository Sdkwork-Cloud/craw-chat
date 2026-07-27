import { Product, Shop, CustomerServiceMessage } from "../types";

const INITIAL_SHOPS: Shop[] = [
  {
    id: "shop_1",
    name: "官方严选旗舰店",
    logo: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/shop1/100x100.png",
    fansCount: "25.6w",
    rating: "4.9",
    isOfficial: true,
    tags: ["品牌直采", "正品保证"],
    description: "为您提供高品质、高性价比的生活好物。",
  },
  {
    id: "shop_2",
    name: "极客数码生活",
    logo: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/shop2/100x100.png",
    fansCount: "8.2w",
    rating: "4.7",
    isOfficial: false,
    tags: ["7天无理由退货"],
    description: "最新鲜的数码体验，最酷的极客装备。",
  },
];

const INITIAL_PRODUCTS: Product[] = [
  {
    id: "p1",
    title: "2026新款 智能降噪蓝牙耳机 续航50小时 头戴式无线耳机",
    price: "299", // min price
    originalPrice: "499",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/product1/300x400.png",
    sales: "已售 1.2w",
    description: "高品质降噪耳机，续航无忧，支持空间音频。",
    shopId: "shop_2",
    specs: [
      {
        id: "color",
        name: "颜色分类",
        options: [
          { id: "c_black", name: "曜石黑" },
          { id: "c_white", name: "月光白" },
          { id: "c_blue", name: "星空蓝" }
        ]
      },
      {
        id: "version",
        name: "版本",
        options: [
          { id: "v_std", name: "标准版" },
          { id: "v_pro", name: "Pro 尊享版 (支持空间音频)" }
        ]
      }
    ],
    skus: [
      { id: "sku_1_1", specValues: { "color": "c_black", "version": "v_std" }, price: "299", originalPrice: "499", stock: 120 },
      { id: "sku_1_2", specValues: { "color": "c_white", "version": "v_std" }, price: "299", originalPrice: "499", stock: 85 },
      { id: "sku_1_3", specValues: { "color": "c_blue", "version": "v_std" }, price: "299", originalPrice: "499", stock: 40 },
      { id: "sku_1_4", specValues: { "color": "c_black", "version": "v_pro" }, price: "399", originalPrice: "599", stock: 60 },
      { id: "sku_1_5", specValues: { "color": "c_white", "version": "v_pro" }, price: "399", originalPrice: "599", stock: 30 },
      { id: "sku_1_6", specValues: { "color": "c_blue", "version": "v_pro" }, price: "399", originalPrice: "599", stock: 10 }
    ]
  },
  {
    id: "p2",
    title: "便携式迷你筋膜枪 肌肉放松 按摩仪 颈膜枪",
    price: "159",
    originalPrice: "299",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/product2/300x250.png",
    sales: "已售 8000+",
    description: "迷你便携，深度按摩，4档调节。",
    shopId: "shop_1",
    specs: [
      {
        id: "color",
        name: "颜色",
        options: [
          { id: "c_gray", name: "太空灰" },
          { id: "c_pink", name: "樱花粉" }
        ]
      },
      {
        id: "combo",
        name: "套餐",
        options: [
          { id: "cb_std", name: "标配版 (4个按摩头)" },
          { id: "cb_upg", name: "升级版 (6个按摩头+收纳包)" }
        ]
      }
    ],
    skus: [
      { id: "sku_2_1", specValues: { "color": "c_gray", "combo": "cb_std" }, price: "159", stock: 200 },
      { id: "sku_2_2", specValues: { "color": "c_pink", "combo": "cb_std" }, price: "159", stock: 150 },
      { id: "sku_2_3", specValues: { "color": "c_gray", "combo": "cb_upg" }, price: "199", stock: 100 },
      { id: "sku_2_4", specValues: { "color": "c_pink", "combo": "cb_upg" }, price: "199", stock: 80 }
    ]
  },
  {
    id: "p3",
    title: "全棉亲肤四季法兰绒毛毯 宿舍单人/双人被",
    price: "89",
    originalPrice: "129",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/product3/300x450.png",
    sales: "已售 2000+",
    description: "柔软亲肤，保暖舒适，防静电处理。",
    shopId: "shop_1",
    specs: [
      {
        id: "size",
        name: "尺寸",
        options: [
          { id: "sz_1", name: "150x200cm (单人适用)" },
          { id: "sz_2", name: "180x200cm (加大单人)" },
          { id: "sz_3", name: "200x230cm (双人适用)" }
        ]
      },
      {
        id: "color",
        name: "颜色",
        options: [
          { id: "c_blue", name: "深海蓝" },
          { id: "c_yellow", name: "姜黄色" },
          { id: "c_gray", name: "高级灰" }
        ]
      }
    ],
    skus: [
      { id: "sku_3_1", specValues: { "size": "sz_1", "color": "c_blue" }, price: "89", stock: 50 },
      { id: "sku_3_2", specValues: { "size": "sz_1", "color": "c_yellow" }, price: "89", stock: 50 },
      { id: "sku_3_3", specValues: { "size": "sz_1", "color": "c_gray" }, price: "89", stock: 50 },
      { id: "sku_3_4", specValues: { "size": "sz_2", "color": "c_blue" }, price: "109", stock: 40 },
      { id: "sku_3_5", specValues: { "size": "sz_2", "color": "c_yellow" }, price: "109", stock: 40 },
      { id: "sku_3_6", specValues: { "size": "sz_2", "color": "c_gray" }, price: "109", stock: 40 },
      { id: "sku_3_7", specValues: { "size": "sz_3", "color": "c_blue" }, price: "139", stock: 30 },
      { id: "sku_3_8", specValues: { "size": "sz_3", "color": "c_yellow" }, price: "139", stock: 30 },
      { id: "sku_3_9", specValues: { "size": "sz_3", "color": "c_gray" }, price: "139", stock: 30 }
    ]
  },
  {
    id: "p4",
    title: "智能温控养生壶 不锈钢玻璃材质 1.5L 多功能煎药壶煮茶器",
    price: "129",
    originalPrice: "199",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/product4/300x200.png",
    sales: "已售 5.5w",
    description: "智能温控，健康养生，18大功能菜单。",
    shopId: "shop_1",
    specs: [
      {
        id: "color",
        name: "外观颜色",
        options: [
          { id: "c_green", name: "薄荷绿" },
          { id: "c_white", name: "珍珠白" }
        ]
      }
    ],
    skus: [
      { id: "sku_4_1", specValues: { "color": "c_green" }, price: "129", stock: 300 },
      { id: "sku_4_2", specValues: { "color": "c_white" }, price: "129", stock: 250 }
    ]
  },
  {
    id: "p_virtual_1",
    title: "星巴克 咖啡星冰乐 电子代金券/兑换券",
    price: "28", // min price
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/coffee/300x350.png",
    sales: "已售 10w+",
    description: "购买后立即发码，支持全国门店核销。",
    categoryId: "卡券",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "denomination",
        name: "面值",
        options: [
          { id: "d_28", name: "28元 (兑换券)" },
          { id: "d_32", name: "32元 (兑换券)" },
          { id: "d_50", name: "50元 (组合券)" },
        ]
      }
    ],
    skus: [
      { id: "sku_1", specValues: { "denomination": "d_28" }, price: "28", originalPrice: "33", stock: 999, description: "可兑换一杯大杯拿铁或美式咖啡，全国任意门店通用", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_1/300x300.png" },
      { id: "sku_2", specValues: { "denomination": "d_32" }, price: "32", originalPrice: "38", stock: 999, description: "可兑换任意大杯星冰乐，含当季特饮", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_2/300x300.png" },
      { id: "sku_3", specValues: { "denomination": "d_50" }, price: "48", originalPrice: "56", stock: 999, description: "包含一杯大杯拿铁及任意切片蛋糕一份", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_3/300x300.png" }
    ]
  },
  {
    id: "p_virtual_2",
    title: "高端AI实战俱乐部 / 技术大牛社群",
    price: "199",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/community/300x400.png",
    sales: "已售 500+",
    description: "购买后将自动拉您进入专属高端交流群，群内有大佬在线解答各种技术难题，定期分享干货。",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "group_chat",
    specs: [
      {
        id: "group_type",
        name: "圈子类型",
        options: [
          { id: "g_ai_1", name: "大模型应用实战圈 (入门营)" },
          { id: "g_ai_2", name: "大模型应用实战圈 (进阶营)" },
          { id: "g_web_1", name: "大厂面试冲刺圈 (前端方向)" },
          { id: "g_web_2", name: "大厂面试冲刺圈 (后端方向)" }
        ]
      },
      {
        id: "duration",
        name: "服务周期",
        options: [
          { id: "dur_quarter", name: "按季订阅 (3个月)" },
          { id: "dur_year", name: "按年订阅 (12个月)" }
        ]
      }
    ],
    skus: [
      { id: "sku_gc_1", specValues: { "group_type": "g_ai_1", "duration": "dur_quarter" }, price: "199", originalPrice: "299", stock: 100 },
      { id: "sku_gc_2", specValues: { "group_type": "g_ai_1", "duration": "dur_year" }, price: "599", originalPrice: "899", stock: 50 },
      { id: "sku_gc_3", specValues: { "group_type": "g_ai_2", "duration": "dur_quarter" }, price: "299", originalPrice: "399", stock: 80 },
      { id: "sku_gc_4", specValues: { "group_type": "g_ai_2", "duration": "dur_year" }, price: "899", originalPrice: "1199", stock: 40 },
      { id: "sku_gc_5", specValues: { "group_type": "g_web_1", "duration": "dur_quarter" }, price: "149", originalPrice: "199", stock: 200 },
      { id: "sku_gc_6", specValues: { "group_type": "g_web_1", "duration": "dur_year" }, price: "449", originalPrice: "599", stock: 100 },
      { id: "sku_gc_7", specValues: { "group_type": "g_web_2", "duration": "dur_quarter" }, price: "149", originalPrice: "199", stock: 200 },
      { id: "sku_gc_8", specValues: { "group_type": "g_web_2", "duration": "dur_year" }, price: "449", originalPrice: "599", stock: 100 }
    ]
  },
  {
    id: "p_virtual_3",
    title: "京东E卡 电子礼品卡 购物卡 充值卡",
    price: "100", // min price
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/jd_card/300x300.png",
    sales: "已售 50w+",
    description: "自营商品通用通用现金卡，送礼首选。",
    categoryId: "卡券",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "value",
        name: "面值",
        options: [
          { id: "v_100", name: "100元" },
          { id: "v_500", name: "500元" },
          { id: "v_1000", name: "1000元" },
        ]
      }
    ],
    skus: [
      { id: "sku_v1", specValues: { "value": "v_100" }, price: "100", stock: 9999, description: "100元电子礼品卡，可在下单时抵扣任意自营商品金额，有效期36个月", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_v1/300x300.png" },
      { id: "sku_v2", specValues: { "value": "v_500" }, price: "500", stock: 9999, description: "500元电子礼品卡，可绑定账户后使用，适合企业发放福利", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_v2/300x300.png" },
      { id: "sku_v3", specValues: { "value": "v_1000" }, price: "1000", stock: 9999, description: "1000元大面额电子礼品卡，尊贵体验，馈赠佳品", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_v3/300x300.png" }
    ]
  },
  {
    id: "p_virtual_4",
    title: "滴滴出行 快车/优享/专车 50元通用打车定额代金券",
    price: "45", // min price
    originalPrice: "50",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/didi_card/300x300.png",
    sales: "已售 2w+",
    description: "滴滴快车专车通用代金券，安全出行。",
    categoryId: "卡券",
    shopId: "shop_2",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "type",
        name: "券类型",
        options: [
          { id: "t_kuaiche", name: "快车券" },
          { id: "t_zhuanche", name: "专车券" },
        ]
      }
    ],
    skus: [
      { id: "sku_d1", specValues: { "type": "t_kuaiche" }, price: "45", originalPrice: "50", stock: 500, description: "滴滴快车、优享通用50元代金券，限一次性使用", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_d1/300x300.png" },
      { id: "sku_d2", specValues: { "type": "t_zhuanche" }, price: "48", originalPrice: "50", stock: 200, description: "滴滴专车通用50元高端出行代金券，舒适体验", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_d2/300x300.png" }
    ]
  },
  {
    id: "p_virtual_5",
    title: "腾讯视频VIP会员 视频会员充值 电子秒冲卡",
    price: "15", // min price
    originalPrice: "25",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/tencent_vip/300x300.png",
    sales: "已售 15w+",
    description: "正版影视VIP，海量大片抢先看，购买后自动充值到填写的账号。",
    categoryId: "卡券",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "duration",
        name: "会员时长",
        options: [
          { id: "dur_1m", name: "月卡 (31天)" },
          { id: "dur_3m", name: "季卡 (93天)" },
          { id: "dur_12m", name: "年卡 (372天)" },
        ]
      }
    ],
    skus: [
      { id: "sku_tv_1", specValues: { "duration": "dur_1m" }, price: "15", originalPrice: "25", stock: 9999, description: "腾讯视频VIP月卡，手机/电脑/Pad通用，不支持电视机端", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_tv1/300x300.png" },
      { id: "sku_tv_2", specValues: { "duration": "dur_3m" }, price: "45", originalPrice: "68", stock: 9999, description: "腾讯视频VIP季卡，畅享93天尊贵特权，免广告", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_tv2/300x300.png" },
      { id: "sku_tv_3", specValues: { "duration": "dur_12m" }, price: "158", originalPrice: "258", stock: 5000, description: "腾讯视频VIP年卡，限时特惠，全网热播剧抢先看", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_tv3/300x300.png" }
    ]
  },
  {
    id: "p_virtual_6",
    title: "移动联通电信 100元手机话费 快充秒到账",
    price: "98.5", 
    originalPrice: "100",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/mobile_recharge/300x300.png",
    sales: "已售 100w+",
    description: "全国三网通用，官方直充，1-3分钟内极速到账。",
    categoryId: "卡券",
    shopId: "shop_2",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "operator",
        name: "运营商",
        options: [
          { id: "op_cmcc", name: "中国移动" },
          { id: "op_cucc", name: "中国联通" },
          { id: "op_ctcc", name: "中国电信" },
        ]
      }
    ],
    skus: [
      { id: "sku_hf_1", specValues: { "operator": "op_cmcc" }, price: "99", originalPrice: "100", stock: 99999, description: "中国移动100元话费快充，支持全国号段，不含空号", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_hf1/300x300.png" },
      { id: "sku_hf_2", specValues: { "operator": "op_cucc" }, price: "98.5", originalPrice: "100", stock: 99999, description: "中国联通100元话费快充，官方直充，方便快捷", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_hf2/300x300.png" },
      { id: "sku_hf_3", specValues: { "operator": "op_ctcc" }, price: "98.5", originalPrice: "100", stock: 99999, description: "中国电信100元话费快充，24小时自动充值系统", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_hf3/300x300.png" }
    ]
  },
  {
    id: "p_virtual_7",
    title: "官方积分充值 游戏道具/商城通用积分卡",
    price: "10", 
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/points_recharge/300x300.png",
    sales: "已售 50w+",
    description: "官方通用积分，可用于兑换各类游戏道具、商城虚拟商品等。",
    categoryId: "卡券",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "points",
        name: "面值",
        options: [
          { id: "pts_100", name: "10元 (100积分)" },
          { id: "pts_200", name: "20元 (200积分)" },
          { id: "pts_500", name: "50元 (500积分)" },
          { id: "pts_1000", name: "100元 (1000积分)" },
        ]
      }
    ],
    skus: [
      { id: "sku_pts_1", specValues: { "points": "pts_100" }, price: "10", stock: 99999, description: "充值10元，立即到账100积分", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_pts1/300x300.png" },
      { id: "sku_pts_2", specValues: { "points": "pts_200" }, price: "20", stock: 99999, description: "充值20元，立即到账200积分", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_pts2/300x300.png" },
      { id: "sku_pts_3", specValues: { "points": "pts_500" }, price: "50", stock: 99999, description: "充值50元，立即到账500积分，额外赠送5积分", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_pts3/300x300.png" },
      { id: "sku_pts_4", specValues: { "points": "pts_1000" }, price: "100", stock: 99999, description: "充值100元，立即到账1000积分，额外赠送20积分", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_pts4/300x300.png" }
    ]
  },
  {
    id: "p_virtual_8",
    title: "每日积分订阅福利套餐 尊享VIP特权",
    price: "1.99",
    originalPrice: "5",
    image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/subscription_vip/300x300.png",
    sales: "已售 8w+",
    description: "超值积分订阅，每天自动到账，订阅越久越划算！",
    categoryId: "卡券",
    shopId: "shop_1",
    isVirtual: true,
    virtualType: "coupon",
    specs: [
      {
        id: "sub_type",
        name: "套餐类型",
        options: [
          { id: "sub_1d", name: "按日订阅" },
          { id: "sub_7d", name: "7天订阅" },
          { id: "sub_1m", name: "一月订阅" },
          { id: "sub_1y", name: "按年订阅" },
        ]
      }
    ],
    skus: [
      { id: "sku_sub_1", specValues: { "sub_type": "sub_1d" }, price: "1.99", originalPrice: "5", stock: 999, description: "1天体验版。开通后当天立得30积分，额外送炫酷头像框体验。", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_sub1/300x300.png" },
      { id: "sku_sub_2", specValues: { "sub_type": "sub_7d" }, price: "9.9", originalPrice: "25", stock: 999, description: "7天短期包。连续7天，每天赠送40积分(共280积分)，并解锁专属表情包。", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_sub2/300x300.png" },
      { id: "sku_sub_3", specValues: { "sub_type": "sub_1m" }, price: "29.9", originalPrice: "68", stock: 999, description: "连续包月套餐。连续30天，每天赠送50积分(共1500积分)，专属特权铭牌。", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_sub3/300x300.png" },
      { id: "sku_sub_4", specValues: { "sub_type": "sub_1y" }, price: "199", originalPrice: "588", stock: 500, description: "年度尊享黑卡。连续365天，每天赠送100积分(共36500积分)，年度绝版装扮。", image: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sku_sub4/300x300.png" }
    ]
  }
];

const CATEGORIES = [
  "推荐",
  "卡券",
  "数码家电",
  "生活日用",
  "服饰箱包",
  "食品饮料",
  "美妆个护",
];

const STORAGE_KEY_PRODUCTS = "sdkwork_im_h5_products_v3";
const STORAGE_KEY_SHOPS = "sdkwork_im_h5_shops_v3";

let MOCK_PRODUCTS: Product[] = [];
let MOCK_SHOPS: Shop[] = [];
let MOCK_CUSTOMER_SERVICE_MESSAGES: Record<string, CustomerServiceMessage[]> = {};

const loadData = () => {
  if (MOCK_PRODUCTS.length === 0 || MOCK_SHOPS.length === 0) {
    try {
      const productsData = localStorage.getItem(STORAGE_KEY_PRODUCTS);
      const shopsData = localStorage.getItem(STORAGE_KEY_SHOPS);
      if (productsData && shopsData) {
        MOCK_PRODUCTS = JSON.parse(productsData);
        MOCK_SHOPS = JSON.parse(shopsData);
        
        // Ensure all products have their latest specs and skus merged
        MOCK_PRODUCTS = MOCK_PRODUCTS.map(p => {
          const initialP = INITIAL_PRODUCTS.find(ip => ip.id === p.id);
          if (initialP) {
            return {
              ...p,
              ...initialP, // Overwrite with all initial properties if they are updated
              // Preserve any properties that should be modifiable by the user, if applicable.
              // For a demo, it's safer to just overwrite with initialP to ensure latest definition.
            };
          }
          return p;
        });
      } else {
        MOCK_PRODUCTS = [...INITIAL_PRODUCTS];
        MOCK_SHOPS = [...INITIAL_SHOPS];
      }
    } catch (e) {
      MOCK_PRODUCTS = [...INITIAL_PRODUCTS];
      MOCK_SHOPS = [...INITIAL_SHOPS];
    }
  }

  // Ensure new virtual products exist
  for (const p of INITIAL_PRODUCTS) {
    if (!MOCK_PRODUCTS.find(existing => existing.id === p.id)) {
      MOCK_PRODUCTS.push(p);
    }
  }

  // Ensure unique IDs to avoid React key errors
  const uniqueProducts = [];
  const seenIds = new Set();
  for (const p of MOCK_PRODUCTS) {
    if (!seenIds.has(p.id)) {
      seenIds.add(p.id);
      uniqueProducts.push(p);
    }
  }
  MOCK_PRODUCTS = uniqueProducts;

  // Always save the clean data
  saveData();

  return { products: MOCK_PRODUCTS, shops: MOCK_SHOPS };
};

const saveData = () => {
  try {
    localStorage.setItem(STORAGE_KEY_PRODUCTS, JSON.stringify(MOCK_PRODUCTS));
    localStorage.setItem(STORAGE_KEY_SHOPS, JSON.stringify(MOCK_SHOPS));
  } catch (e) {
    console.error("Failed to save products data", e);
  }
};

export const ProductService = {
  getProducts: async (): Promise<Product[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadData().products]), 300),
    );
  },
  getProductById: async (id: string): Promise<Product | null> => {
    return new Promise((resolve) =>
      setTimeout(() => {
        resolve(loadData().products.find((p) => p.id === id) || null);
      }, 200),
    );
  },
  getProductsByShop: async (shopId: string): Promise<Product[]> => {
    return new Promise((resolve) =>
      setTimeout(() => {
        resolve(loadData().products.filter((p) => p.shopId === shopId));
      }, 200),
    );
  },
  getShopById: async (id: string): Promise<Shop | null> => {
    return new Promise((resolve) =>
      setTimeout(() => {
        resolve(loadData().shops.find((s) => s.id === id) || null);
      }, 200),
    );
  },
  getCategories: async (): Promise<string[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...CATEGORIES]), 100),
    );
  },
  getCustomerServiceMessages: async (shopId: string): Promise<CustomerServiceMessage[]> => {
    if (!MOCK_CUSTOMER_SERVICE_MESSAGES[shopId]) {
      MOCK_CUSTOMER_SERVICE_MESSAGES[shopId] = [
        {
          id: "msg_1",
          content: "您好！欢迎来到我们的店铺，请问有什么可以帮助您的吗？",
          senderId: "agent",
          senderType: "agent",
          timestamp: Date.now() - 60000,
        }
      ];
    }
    return MOCK_CUSTOMER_SERVICE_MESSAGES[shopId];
  },
  sendCustomMessage: async (shopId: string, msg: CustomerServiceMessage): Promise<void> => {
    if (!MOCK_CUSTOMER_SERVICE_MESSAGES[shopId]) {
      MOCK_CUSTOMER_SERVICE_MESSAGES[shopId] = [];
    }
    MOCK_CUSTOMER_SERVICE_MESSAGES[shopId].push(msg);
  }
};

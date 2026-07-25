import { useTranslation } from "react-i18next";
export interface Product {
  id: string;
  title: string;
  price: number;
  image: string;
  sales: string;
}

const MOCK_PRODUCTS: Product[] = [
  {
    id: "p1",
    title: "2026新款 智能降噪蓝牙耳机 续航50小时",
    price: 299,
    image: "https://picsum.photos/seed/product1/300/300",
    sales: "已售 1.2w",
  },
  {
    id: "p2",
    title: "便携式迷你筋膜枪 肌肉放松 按摩仪",
    price: 159,
    image: "https://picsum.photos/seed/product2/300/300",
    sales: "已售 8000+",
  },
  {
    id: "p3",
    title: "全棉亲肤四季法兰绒毛毯 宿舍单人",
    price: 89,
    image: "https://picsum.photos/seed/product3/300/300",
    sales: "已售 2000+",
  },
  {
    id: "p4",
    title: "智能温控养生壶 不锈钢玻璃材质 1.5L",
    price: 129,
    image: "https://picsum.photos/seed/product4/300/300",
    sales: "已售 5.5w",
  },
];

const CATEGORIES = [
  "推荐",
  "数码家电",
  "生活日用",
  "服饰箱包",
  "食品饮料",
  "美妆个护",
];

export const ProductService = {
  getProducts: async (): Promise<Product[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...MOCK_PRODUCTS]), 300),
    );
  },
  getCategories: async (): Promise<string[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...CATEGORIES]), 100),
    );
  },
};

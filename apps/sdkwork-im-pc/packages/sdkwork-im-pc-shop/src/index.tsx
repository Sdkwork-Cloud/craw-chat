import React, { useState, useEffect } from "react";
import { ShoppingCart, Search, Store, Globe, FileText } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { toast } from "@sdkwork/im-pc-chat";
import {
  shopService,
  ShopProduct,
  ShopCategory,
  CartItem,
} from "./services/ShopService";
import { useTranslation } from "react-i18next";
import "./i18n";

import { CheckoutView } from "./components/CheckoutView";
import { ShopHome } from "./components/ShopHome";
import { ShopCart } from "./components/ShopCart";
import { ProductDetail } from "./components/ProductDetail";
import { OrderDetailView } from "./components/OrderDetailView";
import { chatService } from "@sdkwork/im-pc-chat";

export interface ShopViewProps {
  onNavigateToOrders?: () => void;
}

export const ShopView: React.FC<ShopViewProps> = ({ onNavigateToOrders }) => {
  const { t, i18n } = useTranslation(["common", "checkout", "product"]);
  const [activeTab, setActiveTab] = useState<"home" | "cart" | "product">(
    "home",
  );
  const [categories, setCategories] = useState<ShopCategory[]>([]);
  const [products, setProducts] = useState<ShopProduct[]>([]);
  const [cartCount, setCartCount] = useState(0);
  const [selectedProduct, setSelectedProduct] = useState<ShopProduct | null>(
    null,
  );
  const [activeCategoryId, setActiveCategoryId] = useState<string | null>(null);
  const [orderResult, setOrderResult] = useState<{
    orderId: string;
    items: CartItem[];
    generatedCoupons?: { item: CartItem; product: ShopProduct; code: string }[];
  } | null>(null);

  useEffect(() => {
    shopService.getCategories().then(setCategories);
    shopService.getProducts().then(setProducts);
    updateCartCount();
  }, []);

  const [searchQuery, setSearchQuery] = useState("");

  const updateCartCount = () => {
    shopService.getCart().then((items) => {
      setCartCount(items.reduce((sum, item) => sum + item.quantity, 0));
    });
  };

  const handleOpenProduct = (product: ShopProduct) => {
    setSelectedProduct(product);
    setActiveTab("product");
  };

  const handleAddToCart = async (product: ShopProduct, skuId?: string) => {
    await shopService.addToCart(product.id, 1, skuId);
    toast(
      t("product:addedToCart", { defaultValue: "已加入购物车" }),
      "success",
    );
    updateCartCount();
  };

  const filteredProducts = products.filter(
    (p) =>
      p.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      p.description.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  const [directCheckoutItem, setDirectCheckoutItem] = useState<{
    product: ShopProduct;
    quantity: number;
    skuId?: string;
  } | null>(null);

  const handleBuyNow = (product: ShopProduct, skuId?: string) => {
    setDirectCheckoutItem({ product, quantity: 1, skuId });
  };

  const handleCheckoutSuccess = async (
    orderId: string,
    items: CartItem[],
    generatedCoupons: {
      item: CartItem;
      product: ShopProduct;
      code: string;
    }[] = [],
  ) => {
    // Check for coupons and send messages
    if (generatedCoupons && generatedCoupons.length > 0) {
      generatedCoupons.forEach((coupon) => {
        chatService.sendMessage(
          "shop_assistant",
          `您购买的 [${coupon.product.title}] 兑换码：${coupon.code}`,
          "text",
        );
      });
    }

    setOrderResult({ orderId, items, generatedCoupons });
    setDirectCheckoutItem(null);
  };

  const handleDirectCheckoutComplete = async () => {
    try {
      const checkoutLineItem = {
        id: "DIRECT_BUY",
        productId: directCheckoutItem!.product.id,
        skuId: directCheckoutItem!.skuId,
        quantity: directCheckoutItem!.quantity,
        selected: true,
      };
      const result = await shopService.checkout([checkoutLineItem]);
      toast(
        t("checkout:checkoutSuccess", {
          orderId: result.orderId,
          total: result.total.toLocaleString(
            i18n.language === "en-US" ? "en-US" : "zh-CN",
          ),
          currencySymbol: t("common:currencySymbol"),
        }),
        "success",
      );
      handleCheckoutSuccess(
        result.orderId,
        result.items,
        result.generatedCoupons,
      );
    } catch (e) {
      toast(
        t("checkout:checkoutFailed", { defaultValue: "结算失败" }),
        "error",
      );
    }
  };

  const toggleLanguage = () => {
    const newLang = i18n.language === "zh-CN" ? "en-US" : "zh-CN";
    i18n.changeLanguage(newLang);
  };

  return (
    <div className="flex-1 flex flex-col h-full bg-[#1e1e20] text-gray-200 overflow-hidden relative">
      {/* Header */}
      {activeTab !== "product" && !directCheckoutItem && !orderResult && (
        <div className="h-16 border-b border-white/5 flex items-center justify-between px-6 shrink-0 bg-[#2b2b2d]/50 backdrop-blur-md">
          <div
            className="flex items-center gap-2 cursor-pointer"
            onClick={() => setActiveTab("home")}
          >
            <Store className="text-pink-500" size={24} />
            <h1 className="text-xl font-medium text-gray-100 tracking-tight">
              {t("common:shopCenter", { defaultValue: "购物中心" })}
            </h1>
          </div>
          <div className="flex-1 max-w-md mx-8 relative">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => {
                setSearchQuery(e.target.value);
                if (activeTab !== "home") setActiveTab("home");
              }}
              placeholder={t("common:searchPlaceholder")}
              className="w-full bg-[#1e1e20] border-2 border-white/5 rounded-full py-2.5 pl-12 pr-4 text-sm text-gray-200 focus:outline-none focus:border-pink-500/50 focus:bg-[#252527] transition-all placeholder:text-gray-500 shadow-inner"
            />
            <Search
              size={18}
              className="absolute left-5 top-1/2 -translate-y-1/2 text-gray-400"
            />
          </div>
          <div className="flex items-center gap-4">
            <button
              className="text-gray-400 hover:text-gray-200 transition-colors flex items-center gap-2 p-2 rounded-full hover:bg-white/5"
              onClick={toggleLanguage}
              title={t("common:language")}
            >
              <Globe size={20} />
              <span className="text-sm font-medium">
                {i18n.language === "zh-CN" ? "EN" : "中文"}
              </span>
            </button>
            <button
              className="relative p-2 rounded-full transition-colors text-gray-400 hover:text-gray-200 hover:bg-white/5"
              onClick={onNavigateToOrders}
              title={t("common:orders", { defaultValue: "我的订单" })}
            >
              <FileText size={20} />
            </button>
            <button
              className={`relative p-2 rounded-full transition-colors ${activeTab === "cart" ? "bg-pink-500/10 text-pink-500" : "text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
              onClick={() => setActiveTab("cart")}
            >
              <ShoppingCart size={20} />
              {cartCount > 0 && (
                <span className="absolute top-0 right-0 w-4 h-4 bg-pink-500 text-white text-[10px] flex items-center justify-center rounded-full transform translate-x-1/4 -translate-y-1/4 ring-2 ring-[#1e1e20]">
                  {cartCount}
                </span>
              )}
            </button>
          </div>
        </div>
      )}

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto overflow-x-hidden relative custom-scrollbar">
        <AnimatePresence mode="wait">
          {orderResult ? (
            <React.Fragment key="order-detail">
              <OrderDetailView
                orderId={orderResult.orderId}
                items={orderResult.items}
                products={products}
                generatedCoupons={orderResult.generatedCoupons}
                onReturn={() => {
                  setOrderResult(null);
                  setActiveTab("home");
                }}
              />
            </React.Fragment>
          ) : directCheckoutItem ? (
            <CheckoutView
              key="direct-checkout"
              products={products}
              selectedItems={[
                {
                  id: "DIRECT_BUY",
                  productId: directCheckoutItem.product.id,
                  skuId: directCheckoutItem.skuId,
                  quantity: directCheckoutItem.quantity,
                  selected: true,
                },
              ]}
              totalPrice={
                (directCheckoutItem.product.skus?.find(
                  (s: any) => s.id === directCheckoutItem.skuId,
                )?.price || directCheckoutItem.product.price) *
                directCheckoutItem.quantity
              }
              onBack={() => setDirectCheckoutItem(null)}
              onComplete={handleDirectCheckoutComplete}
            />
          ) : activeTab === "home" ? (
            <React.Fragment key="home">
              <ShopHome
                categories={categories}
                products={filteredProducts}
                onOpenProduct={handleOpenProduct}
                onAddToCart={handleAddToCart}
                activeCategoryId={activeCategoryId}
                setActiveCategoryId={setActiveCategoryId}
              />
            </React.Fragment>
          ) : activeTab === "cart" ? (
            <React.Fragment key="cart">
              <ShopCart
                products={products}
                onUpdateCount={updateCartCount}
                onReturn={() => setActiveTab("home")}
                onCheckoutSuccess={handleCheckoutSuccess}
              />
            </React.Fragment>
          ) : activeTab === "product" && selectedProduct ? (
            <React.Fragment key="product">
              <ProductDetail
                product={selectedProduct}
                onClose={() => setActiveTab("home")}
                onAddToCart={(skuId) => handleAddToCart(selectedProduct, skuId)}
                onBuyNow={(skuId) => handleBuyNow(selectedProduct, skuId)}
              />
            </React.Fragment>
          ) : null}
        </AnimatePresence>
      </div>
    </div>
  );
};

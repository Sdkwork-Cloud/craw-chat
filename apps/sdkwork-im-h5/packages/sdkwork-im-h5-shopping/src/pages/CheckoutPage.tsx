import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { ChevronLeft, ChevronRight, Store } from "lucide-react";
import { ProductService } from "../services/ProductService";
import { useCartStore } from "../store/useCartStore";
import { useAddressStore } from "../store/useAddressStore";
import { Product, Shop, CartItem } from "../types";
import { OrderService } from "@sdkwork/im-h5-orders";
import { AddressSelector } from "../components/AddressSelector";
import { CheckoutOrderItems } from "../components/Checkout/CheckoutOrderItems";
import { CheckoutSummaryCard } from "../components/Checkout/CheckoutSummaryCard";

export const CheckoutPage = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const productId = searchParams.get("productId");
  const skuId = searchParams.get("skuId");
  const quantity = parseInt(searchParams.get("quantity") || "1", 10);
  const isFromCart = searchParams.get("from") === "cart";

  const { getCheckedItems, clearCart } = useCartStore();
  const cartItems = getCheckedItems();

  const [singleProduct, setSingleProduct] = useState<Product | null>(null);
  const [shop, setShop] = useState<Shop | null>(null);

  // Items to checkout
  const [displayItems, setDisplayItems] = useState<any[]>([]);

  useEffect(() => {
    if (productId && !isFromCart) {
      ProductService.getProductById(productId).then((p) => {
        setSingleProduct(p);
        if (p) {
          const sku = p.skus?.find(s => s.id === skuId);
          setDisplayItems([{ product: p, quantity, sku }]);
        }
        if (p?.shopId) {
          ProductService.getShopById(p.shopId).then(setShop);
        }
      });
    } else if (isFromCart) {
      setDisplayItems(cartItems);
      // Mock single shop for simplicity of UI if from cart
      if (cartItems.length > 0 && cartItems[0].product.shopId) {
        ProductService.getShopById(cartItems[0].product.shopId).then(setShop);
      }
    }
  }, [productId, skuId, isFromCart]);

  const { getDefaultOrSelectedAddress } = useAddressStore();

  const handleSubmitOrder = async () => {
    try {
      const selectedAddress = getDefaultOrSelectedAddress();
      // 1. Create a real pending payment order in the databases/localstorage
      const newOrder = await OrderService.createOrder({
        shopName: shop?.name || "官方推荐自营店",
        isVirtual: displayItems.some(item => item.product.isVirtual),
        items: displayItems.map((item, idx) => ({
          id: item.product.id || `${Date.now()}_${idx}`,
          image: item.sku?.image || item.product.image,
          title: item.product.title,
          specs: item.sku ? Object.values(item.sku.specValues || {}).map(vId => {
            const spec = item.product.specs?.find(s => s.options.some(o => o.id === vId));
            return spec?.options.find(o => o.id === vId)?.name || vId;
          }).join(", ") : "默认规格",
          price: parseFloat(item.sku?.price || item.product.price),
          quantity: item.quantity,
          virtualType: item.product.virtualType,
        })),
        totalAmount: parseFloat(totalPrice),
        shippingFee: 0,
        address: displayItems.some(item => item.product.isVirtual) ? undefined : selectedAddress ? {
          name: selectedAddress.name,
          phone: selectedAddress.phone,
          detail: `${selectedAddress.province} ${selectedAddress.city} ${selectedAddress.district} ${selectedAddress.detail}`,
        } : undefined,
      });

      // 2. Clear checked items from cart if checked out from cart
      if (isFromCart) {
        clearCart();
      }

      // 3. Navigate to Cashier with the actual order ID
      navigate(`/cashier?orderId=${newOrder.id}&amount=${totalPrice}`);
    } catch (e) {
      console.error("Failed to create order on checkout:", e);
      // Fallback in case of failure
      navigate(`/cashier?amount=${totalPrice}`);
    }
  };

  if (displayItems.length === 0) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
        <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
        <span className="text-[14px]">{t('shopping.auto_n17176819', '加载确认订单中...')}</span>
      </div>
    );
  }

  const totalQuantity = displayItems.reduce(
    (acc, item) => acc + item.quantity,
    0,
  );
  const totalPrice = displayItems
    .reduce(
      (acc, item) => acc + parseFloat(item.sku?.price || item.product.price) * item.quantity,
      0,
    )
    .toFixed(2);

  const isVirtualOrder = displayItems.some(i => i.product.isVirtual);

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] border-b border-border-color bg-chat-other-bg shrink-0">
        <div
          className="w-10 h-10 flex items-center justify-center cursor-pointer"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <span className="text-[17px] font-medium text-text-main">{t('shopping.auto_38dbf769', '确认订单')}</span>
        <div className="w-10 h-10" />
      </header>

      <div className="flex-1 overflow-y-auto px-3 py-3 pb-[80px]">
        {/* Address Component */}
        {!isVirtualOrder ? (
          <AddressSelector />
        ) : (
          <div className="bg-chat-other-bg rounded-xl p-4 mb-3 flex flex-col gap-2">
             <span className="text-[14px] text-text-main font-medium">{t('shopping.auto_53ad0050', '充值账号/接收手机号')}</span>
             <input type="tel" placeholder={t('shopping.auto_prop_4b3c055e', '请输入充值账号或接收短信手机号')} className="w-full bg-transparent border-b border-border-color py-2 outline-none text-[15px] text-text-main placeholder:text-text-sub" />
             <span className="text-[11px] text-[#FA5151] mt-1">{t('shopping.auto_3cbcc5c0', '* 虚拟资产，充值成功后无法退换，请仔细核对账号')}</span>
          </div>
        )}

        {/* Order Items */}
        <CheckoutOrderItems
          shop={shop}
          displayItems={displayItems}
          isVirtualOrder={isVirtualOrder}
        />

        {/* Total Section */}
        <CheckoutSummaryCard totalPrice={totalPrice} />
      </div>

      {/* Bottom Bar */}
      <div className="absolute bottom-0 left-0 right-0 bg-chat-other-bg border-t border-border-color pb-safe px-4 py-2 flex items-center justify-end h-[60px]">
        <div className="flex items-center mr-4">
          <span className="text-[13px] text-text-sub mr-1">{t('shopping.auto_16a1d40e', '共{totalQuantity}件,')}</span>
          <span className="text-[14px] text-text-main">{t('shopping.auto_14c5ac1', '合计:')}</span>
          <span className="text-[18px] font-bold text-[#FA5151] ml-1">
            <span className="text-[13px]">¥</span>
            {totalPrice}
          </span>
        </div>
        <button
          className="px-6 h-[40px] rounded-full text-[14px] font-medium bg-[#FA5151] text-white active:scale-95 transition-transform"
          onClick={handleSubmitOrder}
        >{t('shopping.auto_2e97bbc7', '提交订单')}</button>
      </div>
    </div>
  );
};

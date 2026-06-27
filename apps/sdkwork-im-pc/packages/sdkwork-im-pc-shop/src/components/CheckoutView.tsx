import React, { useState, useEffect } from "react";
import { motion } from "motion/react";
import {
  ArrowLeft,
  MapPin,
  ChevronRight,
  Smartphone,
} from "lucide-react";
import { toast } from "@sdkwork/im-pc-chat";
import { readAppSdkSessionTokens } from "@sdkwork/im-pc-core";
import { useTranslation } from "react-i18next";
import {
  shopService,
  type ShopShippingAddress,
  PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE,
} from "../services/ShopService";
import { CheckoutAddressModal } from "./CheckoutAddressModal";
import { CheckoutProductList } from "./CheckoutProductList";
import { CheckoutOrderSummary } from "./CheckoutOrderSummary";

export const CheckoutView = ({
  products,
  selectedItems,
  totalPrice,
  onBack,
  onComplete,
}: any) => {
  const { t } = useTranslation(["checkout", "common"]);
  const sessionUser = readAppSdkSessionTokens()?.user;
  const accountLabel = sessionUser?.phone ?? sessionUser?.displayName ?? t("checkout:accountUnavailable", { defaultValue: "当前账号暂未绑定" });
  const [addresses, setAddresses] = useState<ShopShippingAddress[]>([]);
  const [addressesLoading, setAddressesLoading] = useState(true);
  const [selectedAddressId, setSelectedAddressId] = useState("");
  const [showAddressModal, setShowAddressModal] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [addressForm, setAddressForm] = useState<any>(null);

  useEffect(() => {
    let mounted = true;
    const loadAddresses = async () => {
      setAddressesLoading(true);
      try {
        const items = await shopService.getShippingAddresses();
        if (!mounted) {
          return;
        }
        setAddresses(items);
        const defaultAddress = items.find((item) => item.isDefault) ?? items[0];
        setSelectedAddressId(defaultAddress?.id ?? "");
      } catch {
        if (mounted) {
          setAddresses([]);
          setSelectedAddressId("");
        }
      } finally {
        if (mounted) {
          setAddressesLoading(false);
        }
      }
    };

    void loadAddresses();
    return () => {
      mounted = false;
    };
  }, []);

  const addressObj = addresses.find((address) => address.id === selectedAddressId);

  const handleSaveAddress = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const saved = await shopService.saveShippingAddress(addressForm);
      setAddresses((prev) => {
        const next = addressForm.id
          ? prev.map((item) => (item.id === saved.id ? saved : item))
          : [...prev, saved];
        return saved.isDefault
          ? next.map((item) => ({ ...item, isDefault: item.id === saved.id }))
          : next;
      });
      setSelectedAddressId(saved.id);
      setAddressForm(null);
    } catch (error) {
      toast(
        error instanceof Error ? error.message : PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE,
        "error",
      );
    }
  };

  const handleDeleteAddress = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    try {
      await shopService.deleteShippingAddress(id);
      const nextAddresses = addresses.filter((address) => address.id !== id);
      setAddresses(nextAddresses);
      if (selectedAddressId === id) {
        setSelectedAddressId(nextAddresses[0]?.id ?? "");
      }
    } catch (error) {
      toast(
        error instanceof Error ? error.message : PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE,
        "error",
      );
    }
  };

  const handleSetDefault = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    const target = addresses.find((address) => address.id === id);
    if (!target) {
      return;
    }
    try {
      const saved = await shopService.saveShippingAddress({ ...target, isDefault: true });
      setAddresses((prev) =>
        prev.map((item) => ({
          ...item,
          isDefault: item.id === saved.id,
        })),
      );
    } catch (error) {
      toast(
        error instanceof Error ? error.message : PC_SHOP_SHIPPING_ADDRESS_UNAVAILABLE,
        "error",
      );
    }
  };

  const handleSubmitOrder = async () => {
    setIsSubmitting(true);
    try {
      await onComplete();
    } finally {
      setIsSubmitting(false);
    }
  };

  const isAllVirtual =
    selectedItems.length > 0 &&
    selectedItems.every((item: any) => {
      const product = products.find((prod: any) => prod.id === item.productId);
      return product?.isVirtual;
    });

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="absolute inset-0 bg-[#1e1e20] z-20 flex flex-col"
    >
      {showAddressModal ? (
        <CheckoutAddressModal
          showAddressModal={showAddressModal}
          setShowAddressModal={setShowAddressModal}
          addressForm={addressForm}
          setAddressForm={setAddressForm}
          handleSaveAddress={handleSaveAddress}
          addresses={addresses}
          selectedAddressId={selectedAddressId}
          setSelectedAddressId={setSelectedAddressId}
          handleSetDefault={handleSetDefault}
          handleDeleteAddress={handleDeleteAddress}
        />
      ) : null}

      <div className="h-16 border-b border-white/5 flex items-center px-8 shrink-0 bg-[#1e1e20] shadow-sm z-10 sticky top-0">
        <button
          type="button"
          onClick={onBack}
          className="flex items-center gap-2 hover:bg-white/5 px-3 py-1.5 rounded-full transition-colors text-gray-400 hover:text-white"
        >
          <ArrowLeft size={18} />
          <span className="text-sm font-medium">返回购物中心</span>
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-6 flex justify-center custom-scrollbar bg-[#1e1e20]">
        <div className="w-full max-w-6xl flex flex-col lg:flex-row gap-8">
          <div className="flex-1 space-y-6">
            <h2 className="text-xl font-bold text-gray-100 mb-6 hidden lg:block tracking-tight">
              确认订单信息
            </h2>
            {!isAllVirtual ? (
              <div
                onClick={() => setShowAddressModal(true)}
                className="bg-[#2b2b2d] rounded-3xl p-8 border border-white/5 relative overflow-hidden group cursor-pointer hover:border-pink-500/50 transition-all shadow-xl shadow-black/10 hover:shadow-pink-500/5"
              >
                <div className="flex items-center gap-6 mt-2">
                  <div className="w-14 h-14 rounded-full bg-gradient-to-br from-pink-500/20 to-pink-500/5 flex items-center justify-center shrink-0 shadow-inner">
                    <MapPin size={24} className="text-pink-500" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-end gap-3 mb-2">
                      <span className="text-xl font-bold text-gray-100">
                        {addressObj?.name || t("checkout:selectAddress", { defaultValue: "选择收货地址" })}
                      </span>
                      {addressObj?.phone ? (
                        <span className="text-base text-gray-400 font-mono">
                          {addressObj.phone}
                        </span>
                      ) : null}
                    </div>
                    {addressesLoading ? (
                      <p className="text-base text-gray-400 leading-relaxed truncate">
                        {t("checkout:loadingAddresses", { defaultValue: "加载收货地址..." })}
                      </p>
                    ) : addressObj?.address ? (
                      <p className="text-base text-gray-300 leading-relaxed truncate">
                        {addressObj.address}
                      </p>
                    ) : (
                      <p className="text-base text-gray-400 leading-relaxed truncate">
                        {t("checkout:shippingAddressUnavailable", { defaultValue: "收货地址合约暂未开放，请稍后再试" })}
                      </p>
                    )}
                  </div>
                  <ChevronRight
                    size={24}
                    className="text-gray-500 group-hover:text-pink-400 transform group-hover:translate-x-1 transition-all"
                  />
                </div>
              </div>
            ) : (
              <div className="bg-[#2b2b2d] rounded-3xl p-8 border border-white/5 relative overflow-hidden shadow-xl shadow-black/10">
                <div className="flex items-center gap-6">
                  <div className="w-14 h-14 rounded-full bg-gradient-to-br from-blue-500/20 to-blue-500/5 flex items-center justify-center shrink-0">
                    <Smartphone size={24} className="text-blue-500" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-3 mb-1">
                      <span className="text-xl font-bold text-gray-100">
                        {t("checkout:rechargeAccount")}
                      </span>
                    </div>
                    <p className="text-base text-gray-400 leading-relaxed truncate">
                      {t("checkout:currentAccount", {
                        account: accountLabel,
                      })}
                    </p>
                  </div>
                </div>
              </div>
            )}

            <CheckoutProductList selectedItems={selectedItems} products={products} />
          </div>

          <CheckoutOrderSummary
            totalPrice={totalPrice}
            isAllVirtual={isAllVirtual}
            isSubmitting={isSubmitting}
            onSubmitOrder={handleSubmitOrder}
          />
        </div>
      </div>
    </motion.div>
  );
};

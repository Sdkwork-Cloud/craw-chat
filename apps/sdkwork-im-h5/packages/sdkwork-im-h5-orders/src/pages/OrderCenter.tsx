import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Search, MoreHorizontal, Package, ScanLine } from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { OrderService, type Order } from "../services/OrderService";
import { OrderReviewModal } from "../components/OrderReviewModal";
import { OrderCard } from "../components/OrderCard";
import { VoucherRedeemModal } from "../components/VoucherRedeemModal";
import { OrderActionButtons } from "../components/OrderActionButtons";
import { OrderTabsNav } from "../components/OrderTabsNav";

export const OrderCenter: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("all");
  const [tabs, setTabs] = useState<{ id: string; label: string }[]>([]);
  const [orders, setOrders] = useState<Order[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [reviewOrder, setReviewOrder] = useState<Order | null>(null);
  const [showRedeem, setShowRedeem] = useState(false);

  const fetchOrders = async () => {
    setIsLoading(true);
    const data = await OrderService.getOrders();
    setOrders(data);
    setIsLoading(false);
  };

  useEffect(() => {
    OrderService.getOrderTabs().then(setTabs);
    fetchOrders();
  }, []);

  const filteredOrders =
    activeTab === "all" ? orders : orders.filter((o) => o.status === activeTab);

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="bg-bg-color sticky top-0 z-10 shrink-0 pt-safe">
        <div className="h-[44px] px-1 flex items-center justify-between relative">
          <div className="flex items-center z-10 flex-1">
            <IconButton
              icon={
                <ChevronLeft
                  className="w-6 h-6 text-text-main"
                  strokeWidth={2.5}
                />
              }
              onClick={() => navigate(-1)}
            />
          </div>
          <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
            <h2 className="text-[17px] font-semibold text-text-main">
              {t("orders.auto_2e6239c6", "我的订单")}
            </h2>
          </div>
          <div className="flex items-center justify-end z-10 flex-1 pr-1">
            <IconButton
              icon={<ScanLine className="w-5 h-5 text-text-main" />}
              onClick={() => setShowRedeem(true)}
            />
            <IconButton icon={<Search className="w-5 h-5 text-text-main" />} />
            <IconButton
              icon={<MoreHorizontal className="w-5 h-5 text-text-main" />}
            />
          </div>
        </div>

        {/* Tabs */}
        <OrderTabsNav
          tabs={tabs}
          activeTab={activeTab}
          onTabChange={setActiveTab}
        />
      </header>

      {/* Order List */}
      <div className="flex-1 overflow-y-auto bg-[#F2F2F2] dark:bg-[#121212]">
        <div className="p-3 flex flex-col gap-3 pb-12">
          <AnimatePresence mode="popLayout">
            {isLoading ? (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70"
              >
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
                <p className="text-[14px]">{t("orders.auto_7f6f37e", "加载中...")}</p>
              </motion.div>
            ) : filteredOrders.length > 0 ? (
              filteredOrders.map((order) => (
                <OrderCard
                  key={order.id}
                  order={order}
                  onClick={() => navigate(`/me/orders/${order.id}`)}
                  renderActionButtons={(o) => (
                    <OrderActionButtons
                      order={o}
                      onRefresh={fetchOrders}
                      onReview={setReviewOrder}
                    />
                  )}
                />
              ))
            ) : (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70"
              >
                <Package
                  className="w-12 h-12 mb-3 opacity-40 stroke-current"
                  strokeWidth={2}
                />
                <p className="text-[14px]">{t("orders.auto_n37817831", "暂无订单数据")}</p>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>

      <OrderReviewModal
        order={reviewOrder}
        onClose={() => setReviewOrder(null)}
        onSubmit={async (rating, reviewText) => {
          if (!reviewOrder) return;
          await OrderService.reviewOrder(reviewOrder.id);
          showToast(t("orders.auto_fn_768dc96", "评价提交成功"));
          fetchOrders();
        }}
      />

      <VoucherRedeemModal
        isOpen={showRedeem}
        onClose={() => {
          setShowRedeem(false);
          fetchOrders();
        }}
      />
    </div>
  );
};

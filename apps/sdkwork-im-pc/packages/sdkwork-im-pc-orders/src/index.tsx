import React, { useState, useEffect } from "react";
import {
  Search,
  Filter,
  Download,
  ChevronLeft,
  ChevronRight,
  ReceiptText,
  CreditCard,
  CheckCircle2,
  AlertCircle,
  Banknote,
  Package,
  Trash2,
} from "lucide-react";
import { toast } from "@sdkwork/im-pc-chat";
import { ordersService, Order, OrderStats } from "./services/OrdersService";
import { motion, AnimatePresence } from "framer-motion";

const getStatusBadge = (status: Order["status"]) => {
  switch (status) {
    case "PENDING_PAY":
      return <span className="text-orange-400 font-medium">待支付</span>;
    case "PENDING_SHIP":
      return <span className="text-blue-400 font-medium">待发货/处理</span>;
    case "SHIPPED":
      return <span className="text-indigo-400 font-medium">服务中/已发货</span>;
    case "COMPLETED":
      return <span className="text-green-400 font-medium">已完成</span>;
    case "CANCELLED":
      return <span className="text-gray-400 font-medium">已取消</span>;
  }
};

const TABS = [
  { id: "ALL", label: "全部订单" },
  { id: "PENDING_PAY", label: "待支付" },
  { id: "PENDING_SHIP", label: "待发货/处理" },
  { id: "SHIPPED", label: "服务中/已发货" },
  { id: "COMPLETED", label: "已完成" },
  { id: "CANCELLED", label: "已取消" },
];

import { OrderDetailView } from "./components/OrderDetailView";

export const OrdersView: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("ALL");
  const [pageSize, setPageSize] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);
  const [orders, setOrders] = useState<Order[]>([]);
  const [selectedOrder, setSelectedOrder] = useState<Order | null>(null);
  const [showExportModal, setShowExportModal] = useState(false);
  const [exportFormat, setExportFormat] = useState<"excel" | "csv">("excel");

  useEffect(() => {
    ordersService.getOrders().then(setOrders);
  }, []);

  const filteredOrders = orders.filter((order) => {
    const matchSearch =
      order.id.toLowerCase().includes(searchTerm.toLowerCase()) ||
      order.customerName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      order.productInfo.toLowerCase().includes(searchTerm.toLowerCase());
    const matchStatus = statusFilter === "ALL" || order.status === statusFilter;
    return matchSearch && matchStatus;
  });

  const totalPages = Math.ceil(filteredOrders.length / pageSize) || 1;

  useEffect(() => {
    if (currentPage > totalPages) setCurrentPage(totalPages);
  }, [filteredOrders.length, pageSize, totalPages, currentPage]);

  const paginatedOrders = filteredOrders.slice(
    (currentPage - 1) * pageSize,
    currentPage * pageSize,
  );

  const handleAction = (
    orderId: string,
    action: "CANCELLED" | "SHIPPED" | "COMPLETED",
    message: string,
  ) => {
    setOrders(
      orders.map((o) => (o.id === orderId ? { ...o, status: action } : o)),
    );
    toast(message, "success");
  };

  if (selectedOrder) {
    return (
      <OrderDetailView
        order={selectedOrder}
        onBack={() => setSelectedOrder(null)}
        onAction={(id, action, msg) => {
          handleAction(id, action as any, msg);
          setSelectedOrder((prev) =>
            prev ? { ...prev, status: action as any } : null,
          );
        }}
      />
    );
  }

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e20] min-w-0 min-h-0 custom-scrollbar overflow-y-auto">
      <div className="flex-1 w-full p-6 lg:p-8 flex flex-col gap-6 min-h-0">
        {/* Header */}
        <div className="flex items-center justify-between shrink-0">
          <h1 className="text-2xl font-medium text-gray-100 flex items-center gap-2 tracking-tight">
            <ReceiptText className="text-blue-500" />
            订单中心
          </h1>
          <button
            onClick={() => setShowExportModal(true)}
            className="px-4 py-2 bg-white/5 hover:bg-white/10 text-gray-200 text-sm rounded-lg flex items-center gap-2 transition-colors border border-white/10"
          >
            <Download size={16} />
            导出报表
          </button>
        </div>

        {/* Filter Bar & Tabs */}
        <div className="bg-[#2b2b2d] rounded-t-xl border border-white/5 flex flex-col shrink-0 mt-4">
          <div className="flex items-center gap-6 px-6 pt-4 border-b border-white/5">
            {TABS.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setStatusFilter(tab.id)}
                className={`pb-4 text-sm font-medium transition-all relative ${
                  statusFilter === tab.id
                    ? "text-blue-400"
                    : "text-gray-400 hover:text-gray-200"
                }`}
              >
                {tab.label}
                {statusFilter === tab.id && (
                  <motion.div
                    layoutId="activeTabOrder"
                    className="absolute bottom-0 left-0 right-0 h-0.5 bg-blue-500 rounded-t-full"
                  />
                )}
              </button>
            ))}
          </div>

          <div className="p-4 flex items-center gap-4 bg-black/10">
            <div className="relative flex-1 min-w-[240px]">
              <Search
                className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
                size={16}
              />
              <input
                type="text"
                placeholder="搜索订单号 / 客户名称 / 产品信息..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full bg-[#181818] border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-all placeholder:text-gray-500"
              />
            </div>
            <button className="px-4 py-2 bg-[#181818] border border-white/10 hover:bg-white/5 text-gray-200 text-sm rounded-lg flex items-center gap-2 transition-colors">
              <Filter size={16} /> 功能筛选
            </button>
          </div>
        </div>

        {/* Order Cards List */}
        <div className="flex-1 flex flex-col gap-4">
          {/* Table Header like Taobao (aligned correctly) */}
          <div className="bg-[#252527]/80 px-6 py-3 border border-white/5 flex text-xs text-gray-400 font-medium rounded-lg shrink-0 w-full min-w-[700px]">
            <div className="flex-1">商品详情 (Product Details)</div>
            <div className="w-32 text-center shrink-0">单价 (Price)</div>
            <div className="w-20 text-center shrink-0">数量 (Qty)</div>
            <div className="w-36 text-center border-l border-white/5 mx-2 pl-2 shrink-0">
              实付款 (Total)
            </div>
            <div className="w-32 text-center border-l border-white/5 mx-2 pl-2 shrink-0">
              交易状态 (Status)
            </div>
            <div className="w-28 text-center border-l border-white/5 mx-2 pl-2 shrink-0">
              操作 (Actions)
            </div>
          </div>

          {paginatedOrders.length > 0 ? (
            paginatedOrders.map((order) => (
              <div
                key={order.id}
                className="bg-[#2b2b2d] rounded-xl overflow-hidden border border-white/5 hover:border-blue-500/20 transition-colors shadow-lg min-w-[700px] flex-shrink-0"
              >
                {/* Order Header */}
                <div className="bg-[#252527] px-6 py-3 flex justify-between items-center border-b border-white/5 flex-wrap gap-2">
                  <div className="flex items-center gap-6 text-[13px]">
                    <span className="text-gray-400 font-medium shrink-0">
                      {order.createTime}
                    </span>
                    <span className="text-gray-400 shrink-0">
                      订单号:{" "}
                      <span className="text-gray-200 font-mono tracking-tight font-medium">
                        {order.id}
                      </span>
                    </span>
                    <span className="text-gray-400 flex items-center gap-1.5 shrink-0">
                      客户:{" "}
                      <span className="font-semibold text-gray-200">
                        {order.customerName}
                      </span>
                    </span>
                  </div>
                  <button
                    className="p-1.5 text-gray-500 hover:text-red-400 hover:bg-red-500/10 rounded-md transition-colors shrink-0"
                    title="删除订单"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>

                {/* Order Items & Summary Table */}
                <div className="flex w-full">
                  {/* Items List */}
                  <div className="flex-1 min-w-0">
                    {order.items?.map((item, idx) => (
                      <div
                        key={item.id}
                        className={`flex py-4 px-6 group ${idx !== (order.items?.length || 0) - 1 ? "border-b border-white/5" : ""}`}
                      >
                        <div className="flex-1 flex gap-4 min-w-0 pr-4">
                          <div className="w-20 h-20 shrink-0 bg-white/5 rounded-xl overflow-hidden border border-white/5 group-hover:border-blue-500/30 transition-colors">
                            <img
                              src={item.imageUrl}
                              alt={item.productName}
                              referrerPolicy="no-referrer"
                              className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
                            />
                          </div>
                          <div className="flex-1 flex flex-col pt-1 min-w-0">
                            <div className="text-[14px] font-bold text-gray-200 truncate leading-snug group-hover:text-blue-400 transition-colors" title={item.productName}>
                              {item.productName}
                            </div>
                            <div className="mt-1.5 flex gap-2">
                               <span className="text-[11px] font-medium text-gray-400 bg-white/5 px-2 py-0.5 rounded border border-white/5">极简包装组合</span>
                            </div>
                          </div>
                        </div>

                        <div className="w-32 shrink-0 flex flex-col pt-1 items-center justify-start text-[14px] tracking-tight">
                          <span className="font-bold text-gray-200"><span className="text-[12px] font-normal mr-0.5">¥</span>{item.price.toLocaleString("zh-CN", { minimumFractionDigits: 2 })}</span>
                        </div>

                        <div className="w-20 shrink-0 flex pt-1 justify-center text-[13px] font-medium text-gray-400">
                          x {item.quantity}
                        </div>
                      </div>
                    ))}
                  </div>

                  {/* Summary Columns */}
                  <div className="flex shrink-0 border-l border-white/5 text-sm">
                    <div className="w-36 shrink-0 flex flex-col items-center justify-center border-r border-white/5 px-2 text-center row-span-full">
                      <div className="flex flex-col items-center justify-center w-full h-full">
                        <div className="text-[15px] font-bold text-gray-100 tracking-tight flex items-baseline">
                          <span className="text-[12px] font-normal mr-0.5 text-gray-300">¥</span>
                          {order.amount.toLocaleString("zh-CN", {
                            minimumFractionDigits: 2,
                          })}
                        </div>
                        <div className="text-[11px] text-gray-500 mt-2 flex items-center gap-1 font-medium bg-white/5 px-2 py-0.5 rounded-full border border-white/5">
                          <Package size={12} /> 包邮
                        </div>
                      </div>
                    </div>

                    <div className="w-32 shrink-0 flex flex-col items-center justify-center border-r border-white/5 px-2 text-center">
                      <div className="flex flex-col items-center gap-2 w-full h-full justify-center">
                        {getStatusBadge(order.status)}
                        <button
                          onClick={() => setSelectedOrder(order)}
                          className="text-gray-400 text-[13px] font-medium hover:text-blue-400 transition-colors mt-1 hover:underline"
                        >
                          订单详情
                        </button>
                      </div>
                    </div>

                    <div className="w-28 shrink-0 flex flex-col items-center justify-center px-4 gap-2">
                      <div className="flex flex-col items-center gap-2.5 w-full">
                        {order.status === "PENDING_PAY" && (
                          <>
                            <button className="w-full py-1.5 bg-blue-600 hover:bg-blue-500 text-white font-medium rounded shadow-sm hover:shadow transition-all text-xs">
                              立即付款
                            </button>
                            <button
                              onClick={() =>
                                handleAction(
                                  order.id,
                                  "CANCELLED",
                                  "订单已取消",
                                )
                              }
                              className="w-full text-gray-400 hover:text-gray-200 text-xs transition-colors"
                            >
                              取消订单
                            </button>
                          </>
                        )}
                        {order.status === "PENDING_SHIP" && (
                          <>
                            <button
                              onClick={() =>
                                handleAction(order.id, "SHIPPED", "订单已发货")
                              }
                              className="w-full py-1.5 border border-blue-500/50 text-blue-400 bg-blue-500/10 hover:bg-blue-500 hover:text-white font-medium rounded shadow-sm hover:shadow transition-all text-xs"
                            >
                              发货处理
                            </button>
                            <button
                              onClick={() => handleAction(order.id, "CANCELLED", "退款申请已提交，等待卖家处理")}
                              className="w-full text-gray-400 hover:text-gray-200 text-xs transition-colors"
                            >
                              申请退款
                            </button>
                          </>
                        )}
                        {order.status === "SHIPPED" && (
                          <button
                            onClick={() =>
                              handleAction(order.id, "COMPLETED", "交易成功")
                            }
                            className="w-full py-1.5 bg-green-600 hover:bg-green-500 text-white font-medium rounded shadow-sm hover:shadow transition-all text-xs"
                          >
                            确认完成
                          </button>
                        )}
                        {order.status === "COMPLETED" && (
                          <>
                            <button className="w-full py-1.5 border border-white/10 hover:bg-white/5 hover:border-white/20 text-gray-300 font-medium rounded transition-colors text-xs">
                              评价
                            </button>
                            <button
                              onClick={() => toast("已发起售后申请", "success")}
                              className="w-full text-gray-400 hover:text-gray-200 text-xs transition-colors"
                            >
                              申请售后
                            </button>
                          </>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            ))
          ) : (
            <div className="bg-[#2b2b2d] rounded-xl border border-white/5 h-64 flex items-center justify-center text-gray-500">
              暂无符合条件的订单
            </div>
          )}
        </div>

        {/* Pagination (Simplified Desktop Style) */}
        {filteredOrders.length > 0 && (
          <div className="py-4 flex items-center justify-end gap-4 text-sm text-gray-400 shrink-0">
            <div className="flex items-center gap-2">
              <button
                onClick={() => setCurrentPage((p) => Math.max(1, p - 1))}
                disabled={currentPage === 1}
                className="px-3 py-1.5 border border-white/10 rounded hover:bg-white/5 disabled:opacity-50 transition-colors"
              >
                上一页
              </button>
              <div className="px-4 text-gray-300">
                {currentPage} / {totalPages}
              </div>
              <button
                onClick={() =>
                  setCurrentPage((p) => Math.min(totalPages, p + 1))
                }
                disabled={currentPage === totalPages}
                className="px-3 py-1.5 border border-white/10 rounded hover:bg-white/5 disabled:opacity-50 transition-colors"
              >
                下一页
              </button>
            </div>
          </div>
        )}
      </div>

      {showExportModal && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in">
          <div className="bg-[#1e1e1e] border border-white/10 rounded-2xl w-full max-w-sm shadow-xl p-6 relative">
            <h3 className="text-lg font-medium text-white mb-4">导出报表</h3>
            <select
              value={exportFormat}
              onChange={(e) =>
                setExportFormat(e.target.value as "excel" | "csv")
              }
              className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-blue-500/50 mb-6"
            >
              <option value="excel">Excel (.xlsx)</option>
              <option value="csv">CSV (.csv)</option>
            </select>
            <div className="flex justify-end gap-3">
              <button
                className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                onClick={() => setShowExportModal(false)}
              >
                取消
              </button>
              <button
                className="px-5 py-2 text-sm bg-blue-600 hover:bg-blue-500 text-white rounded-xl transition-colors font-medium"
                onClick={() => {
                  setShowExportModal(false);
                  toast(
                    `报表 (格式: ${exportFormat}) 已开始生成下载`,
                    "success",
                  );
                }}
              >
                导出
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

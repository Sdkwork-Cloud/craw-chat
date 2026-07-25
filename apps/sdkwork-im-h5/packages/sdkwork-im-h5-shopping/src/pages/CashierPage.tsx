import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { ChevronLeft } from "lucide-react";
import { showToast } from "@sdkwork/im-h5-commons";
import { OrderService } from "@sdkwork/im-h5-orders";
import { PaymentPasswordModal } from "../components/PaymentPasswordModal";
import { PaymentMethodSelector } from "../components/PaymentMethodSelector";

export const CashierPage = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const amount = searchParams.get("amount") || "0.00";
  const orderId = searchParams.get("orderId");

  const [selectedPayment, setSelectedPayment] = useState("wechat");
  const [showPassword, setShowPassword] = useState(false);
  const [password, setPassword] = useState("");
  const [isWeChat, setIsWeChat] = useState(false);
  const [isAlipay, setIsAlipay] = useState(false);

  React.useEffect(() => {
    const ua = navigator.userAgent.toLowerCase();
    const isWx = ua.includes('micromessenger');
    const isAli = ua.includes('alipayclient');
    setIsWeChat(isWx);
    setIsAlipay(isAli);
    if (isWx) {
      setSelectedPayment("wechat");
    } else if (isAli) {
      setSelectedPayment("alipay");
    }
  }, []);

  const processPayment = () => {
  const paymentName = selectedPayment === 'wechat' ? '微信' : selectedPayment === 'alipay' ? '支付宝' : '余额';
    showToast(`${paymentName}支付处理中...`);
    
    setTimeout(async () => {
      if (orderId) {
        try {
          await OrderService.payOrder(orderId);
          const order = await OrderService.getOrderById(orderId);
          
          // Send automatic voucher to chat
          if (order && order.isVirtual) {
            const { ProductService } = await import("../services/ProductService");
            const { ChatService } = await import("@sdkwork/im-h5-chat");
            let targetShopChatId = "shop_1";
            let navigateToChatId = targetShopChatId;
            let groupChatCreated = false;

            for (const item of order.items) {
              if (item.virtualType === 'coupon') {
                const voucherCodeStr = item.voucherCodes ? item.voucherCodes.map(v => v.code).join(", ") : '请联系客服获取';
                ProductService.sendCustomMessage(targetShopChatId, {
                  id: `msg_${Date.now()}_${Math.random()}`,
                  content: `[系统发货] 您购买的【${item.title}】\n规格: ${item.specs}\n券码: ${voucherCodeStr}`,
                  senderId: targetShopChatId,
                  senderType: "agent",
                  timestamp: Date.now()
                });
              } else if (item.virtualType === 'group_chat') {
                const groupName = item.specs ? item.specs : item.title;
                const newChat = await ChatService.joinOrCreateGroupChat(groupName);
                if (newChat) {
                   navigateToChatId = newChat.id;
                   groupChatCreated = true;
                }
              }
            }

            if (groupChatCreated) {
              navigate(`/chat/${navigateToChatId}`, { replace: true });
              return;
            }
          }
        } catch (e) {
          console.error("Failed to mark order as paid", e);
        }
      }
      showToast(t('shopping.auto_fn_n48d189d7', '支付成功！'));
      setTimeout(async () => {
        if (orderId) {
          const order = await OrderService.getOrderById(orderId);
          if (order && order.isVirtual) {
            navigate(`/shop-chat/shop_1`, { replace: true });
            return;
          }
        }
        navigate("/me/orders", { replace: true });
      }, 1000);
    }, 1500);
  };

  const handleConfirmPayment = () => {
  if (selectedPayment === "wechat" || selectedPayment === "alipay") {
      processPayment();
    } else {
      setShowPassword(true);
    }
  };

  const handlePasswordSubmit = () => {
  if (password.length !== 6) {
      showToast(t('shopping.auto_fn_4514e444', '请输入6位支付密码'));
      return;
    }
    setShowPassword(false);
    processPayment();
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] border-b border-border-color bg-chat-other-bg shrink-0">
        <div
          className="w-10 h-10 flex items-center justify-center cursor-pointer"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <span className="text-[17px] font-medium text-text-main">{t('shopping.auto_18e4d70', '收银台')}</span>
        <div className="w-10 h-10" />
      </header>

      <div className="flex-1 overflow-y-auto pb-safe">
        <div className="flex flex-col items-center py-10 bg-chat-other-bg mb-3 border-b border-border-color/50">
          <span className="text-[14px] text-text-main mb-2">{t('shopping.auto_2436689', '需支付')}</span>
          <span className="text-[36px] font-bold text-text-main">
            <span className="text-[24px] mr-1">¥</span>
            {amount}
          </span>
        </div>

        <div className="px-4">
          {!isWeChat && !isAlipay && (
            <PaymentMethodSelector
              selectedPayment={selectedPayment}
              setSelectedPayment={setSelectedPayment}
            />
          )}

          <div className="mt-8">
            <button
              className="w-full py-[14px] rounded-xl text-[16px] font-medium bg-[#07C160] text-white active:scale-[0.98] transition-transform"
              onClick={handleConfirmPayment}
            >{t('shopping.auto_n6e652df2', '确认支付 ¥{amount}')}</button>
          </div>
        </div>
      </div>

      {showPassword && (
        <PaymentPasswordModal
          amount={amount}
          password={password}
          setPassword={setPassword}
          onClose={() => setShowPassword(false)}
          onSubmit={handlePasswordSubmit}
        />
      )}

    </div>
  );
};

import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { cn, IconButton } from "@sdkwork/im-h5-commons";
import { MessageSquare, Check, X, Lock } from "lucide-react";
import { AuthService } from "@sdkwork/im-h5-user";
import { useNavigate, useLocation } from "react-router";

interface PaymentSheetProps {
  communityName: string;
  communityPrice: number;
  communityCoverImage: string;
  onClose: () => void;
  onConfirm: () => void;
}

export const PaymentSheet: React.FC<PaymentSheetProps> = ({
  communityName,
  communityPrice,
  communityCoverImage,
  onClose,
  onConfirm,
}) => {
  const { t } = useTranslation();
const [selectedPayment, setSelectedPayment] = useState<'wechat'|'alipay'|null>(null);
  const [isWeChat, setIsWeChat] = useState(false);
  const [isAlipay, setIsAlipay] = useState(false);
  const [isAuthChecking, setIsAuthChecking] = useState(true);
  
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    // 检测是否登录
    const user = AuthService.getCurrentUser();
    if (!user) {
      // 未登录，检测是否在微信内（可支持授权/静默登录Mock）
      const ua = navigator.userAgent.toLowerCase();
      const isWx = ua.includes('micromessenger');
      
      const currentPath = encodeURIComponent(location.pathname + location.search);
      if (isWx) {
        // Mock 微信授权登录流程
        // 实际开发中会重定向到微信授权URL
      }
      navigate(`/login?redirect=${currentPath}`);
    } else {
      setIsAuthChecking(false);
    }
  }, [navigate, location]);

  useEffect(() => {
    // 环境检测：微信/支付宝/网页
    const ua = navigator.userAgent.toLowerCase();
    const isWx = ua.includes('micromessenger');
    const isAli = ua.includes('alipayclient');
    
    setIsWeChat(isWx);
    setIsAlipay(isAli);
    
    // 如果在微信内，仅保留微信支付
    // 如果在支付宝内，仅保留支付宝支付
    // 否则两者或者默认均可，这里做个默认值选取
    if (isWx) {
      setSelectedPayment('wechat');
    } else if (isAli) {
      setSelectedPayment('alipay');
    } else {
      setSelectedPayment('wechat'); // 网页版默认
    }
  }, []);

  if (isAuthChecking) {
    return null; // 或者返回一个Loading状态
  }

  return (
    <div className="absolute inset-0 z-[100] flex flex-col justify-end pointer-events-auto overflow-hidden">
      <div 
        className="absolute inset-0 bg-black/40 transition-opacity"
        onClick={onClose}
      />
      <div className="bg-white dark:bg-[#1C1C1E] rounded-t-2xl w-full relative z-10 p-5 pb-[calc(20px+env(safe-area-inset-bottom))] animate-in slide-in-from-bottom duration-300 max-h-[85vh] flex flex-col">
        <div className="flex items-center justify-between mb-6 shrink-0">
          <h3 className="text-[18px] font-bold text-text-main">{t('community.auto_38dbf769', '确认订单')}</h3>
          <IconButton 
            icon={<X className="w-5 h-5 text-text-main" />}
            className="w-8 h-8 -mr-1"
            onClick={onClose}
          />
        </div>

        <div className="flex-1 overflow-y-auto pb-4 scrollbar-none">
          <div className="flex items-center gap-4 mb-6 bg-[#F8F9FA] dark:bg-[#2C2C2E] p-4 rounded-xl shadow-sm border border-black/5 dark:border-white/5">
            <img src={communityCoverImage} alt="" className="w-16 h-16 rounded-lg object-cover border border-black/10 dark:border-white/10" />
            <div className="flex flex-col flex-1">
              <span className="text-[16px] font-semibold text-text-main mb-1 line-clamp-1">{communityName}</span>
              <span className="text-[13px] text-text-sub line-clamp-1">{t('community.auto_n150cf9c5', '付费圈子买断（永久有效）')}</span>
            </div>
            <div className="text-[20px] font-bold text-text-main">¥{communityPrice}</div>
          </div>

          {!isWeChat && !isAlipay && (
            <div className="mb-6">
              <h4 className="text-[14px] font-medium text-text-sub mb-3">{t('community.auto_2f3381bf', '支付方式')}</h4>
              <div className="flex flex-col gap-3">
                <div 
                  className={cn("flex items-center justify-between p-3 rounded-xl border transition-colors cursor-pointer", selectedPayment === 'wechat' ? "border-emerald-500 bg-emerald-500/5" : "border-black/5 dark:border-white/5")}
                  onClick={() => setSelectedPayment('wechat')}
                >
                  <div className="flex items-center gap-2">
                    <div className="w-8 h-8 rounded-full bg-emerald-500/10 flex items-center justify-center">
                      <MessageSquare className="w-5 h-5 text-emerald-500 fill-emerald-500" />
                    </div>
                    <span className="text-[15px] font-medium text-text-main">{t('community.auto_2cb6c4bc', '微信支付')}</span>
                  </div>
                  <div className={cn("w-5 h-5 rounded-full border-2 flex items-center justify-center", selectedPayment === 'wechat' ? "border-emerald-500" : "border-text-sub/30")}>
                    {selectedPayment === 'wechat' && <div className="w-2.5 h-2.5 rounded-full bg-emerald-500" />}
                  </div>
                </div>
                <div 
                  className={cn("flex items-center justify-between p-3 rounded-xl border transition-colors cursor-pointer", selectedPayment === 'alipay' ? "border-blue-500 bg-blue-500/5" : "border-black/5 dark:border-white/5")}
                  onClick={() => setSelectedPayment('alipay')}
                >
                  <div className="flex items-center gap-2">
                    <div className="w-8 h-8 rounded-full bg-blue-500/10 flex items-center justify-center">
                      <Check className="w-5 h-5 text-blue-500" />
                    </div>
                    <span className="text-[15px] font-medium text-text-main">{t('community.auto_185bd34', '支付宝')}</span>
                  </div>
                  <div className={cn("w-5 h-5 rounded-full border-2 flex items-center justify-center", selectedPayment === 'alipay' ? "border-blue-500" : "border-text-sub/30")}>
                    {selectedPayment === 'alipay' && <div className="w-2.5 h-2.5 rounded-full bg-blue-500" />}
                  </div>
                </div>
              </div>
            </div>
          )}

          <div className="flex items-center justify-between mb-8 px-1">
            <span className="text-[15px] text-text-main font-medium">{t('community.auto_2c3862b0', '应付金额')}</span>
            <span className="text-[28px] font-bold text-red-500 leading-none">¥{communityPrice}</span>
          </div>
        </div>

        <div className="shrink-0 pt-2 mt-auto">
          <button 
            onClick={onConfirm}
            className="w-full bg-blue-500 text-white rounded-full py-3.5 font-bold text-[16px] shadow-lg shadow-blue-500/30 active:scale-95 transition-transform flex items-center justify-center gap-1.5"
          >
            <Lock className="w-5 h-5" />{t('community.auto_39175f91', '立即支付')}</button>
        </div>
      </div>
    </div>
  );
};

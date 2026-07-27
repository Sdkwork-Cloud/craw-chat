import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import { ChevronLeft, CheckCircle2, Ticket, ChevronRight } from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { CourseService, CourseData } from "../services/CourseService";

export const CoursePurchase: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const [selectedPayment, setSelectedPayment] = useState<"wechat" | "alipay">("wechat");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [agreed, setAgreed] = useState(false);
  const [course, setCourse] = useState<CourseData | null>(null);
  const [loading, setLoading] = useState(true);
  const [isWeChat, setIsWeChat] = useState(false);
  const [isAlipay, setIsAlipay] = useState(false);

  useEffect(() => {
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

  useEffect(() => {
    const fetchCourse = async () => {
      if (!id) return;
      setLoading(true);
      try {
        const data = await CourseService.getCourseDetail(id);
        if (data) {
          setCourse(data);
        }
      } catch (error) {
        console.error("Failed to fetch course details", error);
      } finally {
        setLoading(false);
      }
    };
    fetchCourse();
  }, [id]);

  const handlePurchase = async () => {
    if (!agreed) {
       showToast(t('course.auto_fn_160bed03', '请先阅读并同意用户购买协议'));
       return;
    }
    if (!id) return;

    setIsSubmitting(true);
    try {
      const success = await CourseService.purchaseCourse(id, selectedPayment);
      if (success) {
         showToast(t('course.auto_fn_2f3303d8', '支付成功'));
         if (course?.type === 'live') {
            navigate(`/course/${id}/live`, { replace: true });
         } else {
            navigate(`/course/${id}/play`, { replace: true });
         }
      } else {
         showToast(t('course.auto_fn_1f52e830', '支付失败，请重试'));
      }
    } catch (error) {
      showToast(t('course.auto_fn_2f32b0bf', '支付异常'));
      console.error(error);
    } finally {
      setIsSubmitting(false);
    }
  };

  if (loading || !course) {
     return (
        <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-hidden relative items-center justify-center">
           <span className="text-[14px] text-text-sub">{loading ? "加载中..." : "未找到课程信息"}</span>
        </div>
     );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-hidden relative">
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-bg-color shrink-0 shadow-sm border-b border-black/5 dark:border-white/5">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="bg-transparent w-10 h-10 -ml-2"
          onClick={() => navigate(-1)}
        />
        <h1 className="text-[17px] font-semibold text-text-main">{t('course.auto_38dbf769', '确认订单')}</h1>
        <div className="w-10" />
      </header>

      <div className="flex-1 overflow-y-auto w-full pb-[110px]">
         <div className="p-4 flex flex-col gap-4">
            {/* Order Info */}
            <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl p-4 shadow-sm border border-black/5 dark:border-white/5">
               <h3 className="text-[15px] font-medium text-text-main mb-4">{t('course.auto_280b9229', '商品信息')}</h3>
               <div className="flex items-start gap-4">
                  <div className="w-[100px] aspect-[4/3] rounded-lg overflow-hidden shrink-0 border border-black/5 dark:border-white/5">
                     <img src={course.cover} alt="cover" className="w-full h-full object-cover" />
                  </div>
                  <div className="flex-1 min-w-0 flex flex-col justify-between h-[75px]">
                     <h4 className="text-[14px] font-medium text-text-main line-clamp-2 leading-snug">{course.title}</h4>
                     <div className="text-[16px] text-red-500 font-bold">¥{course.price}</div>
                  </div>
               </div>
            </div>

            {/* Coupons */}
            <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl shadow-sm border border-black/5 dark:border-white/5 px-4">
               <div className="flex items-center justify-between py-4 active:opacity-70 transition-opacity cursor-pointer">
                  <div className="flex items-center gap-2">
                     <Ticket className="w-5 h-5 text-orange-500" />
                     <span className="text-[15px] font-medium text-text-main">{t('course.auto_134f670', '优惠券')}</span>
                  </div>
                  <div className="flex items-center gap-1 text-[14px] text-text-sub">
                     <span>{t('course.auto_3021ff37', '暂无可用')}</span>
                     <ChevronRight className="w-4 h-4" />
                  </div>
               </div>
            </div>

            {/* Payment Method */}
            {!isWeChat && !isAlipay && (
              <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl shadow-sm border border-black/5 dark:border-white/5 overflow-hidden">
                 <div className="p-4 border-b border-black/5 dark:border-white/5">
                    <h3 className="text-[15px] font-medium text-text-main">{t('course.auto_2f3381bf', '支付方式')}</h3>
                 </div>
                 <div className="flex flex-col">
                    {/* WeChat Pay */}
                    <div 
                      className="flex flex-row items-center justify-between p-4 cursor-pointer active:bg-black/5 dark:active:bg-white/5 transition-colors border-b border-black/5 dark:border-white/5"
                      onClick={() => setSelectedPayment("wechat")}
                    >
                      <div className="flex flex-row items-center gap-3">
                        <div className="w-8 h-8 flex items-center justify-center shrink-0">
                          <img 
                            src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/brand/wechat-pay/64x64.png"
                            alt={t('course.auto_prop_2cb6c4bc', '微信支付')}
                            className="w-full h-full object-contain"
                          />
                        </div>
                        <span className="text-[15px] text-text-main font-medium">{t('course.auto_2cb6c4bc', '微信支付')}</span>
                      </div>
                      {selectedPayment === "wechat" ? (
                        <CheckCircle2 className="w-5 h-5 text-green-500 fill-green-500/20" />
                      ) : (
                        <div className="w-5 h-5 rounded-full border border-black/20 dark:border-white/20" />
                      )}
                    </div>
                    {/* Alipay */}
                    <div 
                      className="flex flex-row items-center justify-between p-4 cursor-pointer active:bg-black/5 dark:active:bg-white/5 transition-colors"
                      onClick={() => setSelectedPayment("alipay")}
                    >
                      <div className="flex flex-row items-center gap-3">
                        <div className="w-8 h-8 flex items-center justify-center shrink-0">
                          <img 
                            src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/brand/alipay/64x64.png"
                            alt={t('course.auto_prop_185bd34', '支付宝')}
                            className="w-full object-contain"
                          />
                        </div>
                        <span className="text-[15px] text-text-main font-medium">{t('course.auto_185bd34', '支付宝')}</span>
                      </div>
                      {selectedPayment === "alipay" ? (
                        <CheckCircle2 className="w-5 h-5 text-green-500 fill-green-500/20" />
                      ) : (
                        <div className="w-5 h-5 rounded-full border border-black/20 dark:border-white/20" />
                      )}
                    </div>
                 </div>
              </div>
            )}
         </div>
      </div>

      <div className="fixed bottom-0 left-0 right-0 pt-2 pb-safe bg-gradient-to-t from-white via-white/95 to-transparent dark:from-[#1C1C1E] dark:via-[#1C1C1E]/95 z-20 pointer-events-none">
         {/* Agreement */}
         <div className="mb-2 flex items-center justify-center gap-2 pointer-events-auto">
            <div 
              className="w-4 h-4 rounded border border-black/20 dark:border-white/20 flex items-center justify-center shrink-0 cursor-pointer"
              onClick={() => setAgreed(!agreed)}
            >
               {agreed && <CheckCircle2 className="w-4 h-4 text-blue-500 bg-white dark:bg-black rounded-full" />}
            </div>
            <div className="text-[12px] text-text-sub">{t('course.auto_78ba3bc2', '我已阅读并同意')}<span className="text-blue-500 cursor-pointer ml-1">{t('course.auto_4768cff0', '《用户购买协议》')}</span>
            </div>
         </div>
         {/* Bottom Action */}
         <div className="mx-4 mb-4 bg-white/80 dark:bg-[#2A2A2D]/80 backdrop-blur-xl border border-black/5 dark:border-white/10 rounded-full pl-6 pr-2 py-2 flex items-center justify-between shadow-xl pointer-events-auto">
            <div className="flex items-center gap-2">
               <span className="text-[13px] text-text-sub">{t('course.auto_161e384', '实付款')}</span>
               <div className="flex items-baseline gap-0.5">
                  <span className="text-[14px] text-red-500 font-bold">¥</span>
                  <span className="text-[22px] text-red-500 font-bold leading-none">{course.price}</span>
               </div>
            </div>
            <button 
              onClick={handlePurchase}
              disabled={isSubmitting}
              className="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white font-medium px-8 py-3 rounded-full active:scale-95 transition-all outline-none text-[15px] shadow-blue-500/20 shadow-lg"
            >{isSubmitting ? "支付中..." : "立刻支付"}</button>
         </div>
      </div>

      {isSubmitting && (
         <div className="absolute inset-0 z-50 flex items-center justify-center bg-black/20 backdrop-blur-sm animate-in fade-in">
            <div className="bg-white dark:bg-[#2C2C2E] p-6 rounded-2xl flex flex-col items-center gap-4 shadow-xl">
               <div className="w-8 h-8 rounded-full border-2 border-blue-500 border-t-transparent animate-spin" />
               <span className="text-[15px] font-medium text-text-main">{t('course.auto_n5c741986', '正在发起支付...')}</span>
            </div>
         </div>
      )}
    </div>
  );
};

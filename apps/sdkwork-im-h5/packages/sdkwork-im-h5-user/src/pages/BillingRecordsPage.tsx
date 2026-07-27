import { useTranslation } from "react-i18next";
import React, { useState, useMemo } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { ArrowDownLeft, ArrowUpRight, ReceiptText, Search } from "lucide-react";

interface Record {
  id: string;
  title: string;
  date: string;
  amount: string;
  type: "expense" | "income";
  status: "success" | "pending" | "failed";
}

const MOCK_RECORDS: Record[] = [
  {
    id: "1",
    title: "Token 充值 (1000T)",
    date: "2026-05-25 14:30:00",
    amount: "-95.00",
    type: "expense",
    status: "success",
  },
  {
    id: "2",
    title: "VIP 连续包年",
    date: "2026-05-24 10:15:00",
    amount: "-188.00",
    type: "expense",
    status: "success",
  },
  {
    id: "3",
    title: "邀请好友奖励",
    date: "2026-05-23 09:00:00",
    amount: "+50.00",
    type: "income",
    status: "success",
  },
  {
    id: "4",
    title: "Token Plan (专业全能包)",
    date: "2026-05-20 16:45:00",
    amount: "-1200.00",
    type: "expense",
    status: "success",
  },
  {
    id: "5",
    title: "系统退款",
    date: "2026-05-18 11:20:00",
    amount: "+20.00",
    type: "income",
    status: "success",
  }
];

export const BillingRecordsPage = () => {
  const { t } = useTranslation();
const [filter, setFilter] = useState<"all" | "expense" | "income">("all");
  const [searchQuery, setSearchQuery] = useState("");

  const records = useMemo(() => {
    return MOCK_RECORDS.filter(
      (record) => {
        const matchFilter = filter === "all" || record.type === filter;
        const matchSearch = record.title.toLowerCase().includes(searchQuery.toLowerCase());
        return matchFilter && matchSearch;
      }
    );
  }, [filter, searchQuery]);

  return (
    <PageLayout title="账单记录" bgClass="bg-[#F8F9FA] dark:bg-black">
      <div className="bg-white dark:bg-[#1A1A1A] sticky top-0 z-20 shadow-sm border-b border-border-color">
         <div className="px-4 py-3 pb-1">
            <div className="bg-gray-100 dark:bg-white/5 rounded-full flex items-center h-9 px-3 gap-2 border border-transparent focus-within:border-primary-blue/30 transition-colors">
              <Search className="w-4 h-4 text-text-sub shrink-0" />
              <input 
                 value={searchQuery}
                 onChange={e => setSearchQuery(e.target.value)}
                 className="flex-1 bg-transparent text-[14px] text-text-main outline-none placeholder:text-text-sub"
                 placeholder="搜索账单标题"
              />
            </div>
         </div>
         <div className="flex">
          <div 
             className={`flex-1 text-center py-3.5 text-[14px] font-medium transition-colors border-b-2 cursor-pointer ${filter === "all" ? "text-primary-blue border-primary-blue" : "text-text-sub border-transparent"}`}
             onClick={() => setFilter("all")}
          >全部</div>
          <div 
             className={`flex-1 text-center py-3.5 text-[14px] font-medium transition-colors border-b-2 cursor-pointer ${filter === "expense" ? "text-primary-blue border-primary-blue" : "text-text-sub border-transparent"}`}
             onClick={() => setFilter("expense")}
          >支出</div>
          <div 
             className={`flex-1 text-center py-3.5 text-[14px] font-medium transition-colors border-b-2 cursor-pointer ${filter === "income" ? "text-primary-blue border-primary-blue" : "text-text-sub border-transparent"}`}
             onClick={() => setFilter("income")}
          >收入</div>
        </div>
      </div>

      <div className="p-4 space-y-3">
        {records.length > 0 ? (
          records.map((record) => (
            <div key={record.id} className="bg-white dark:bg-[#1A1A1A] rounded-xl p-4 flex items-center justify-between shadow-sm border border-border-color cursor-pointer active:bg-gray-50 dark:active:bg-[#2A2A2D] transition-colors">
              <div className="flex items-center gap-3">
                <div className={`w-10 h-10 rounded-full flex items-center justify-center ${record.type === 'expense' ? 'bg-red-50 text-red-500 dark:bg-red-900/20' : 'bg-green-50 text-green-500 dark:bg-green-900/20'}`}>
                   {record.type === 'expense' ? <ArrowUpRight className="w-5 h-5" /> : <ArrowDownLeft className="w-5 h-5" />}
                </div>
                <div>
                  <div className="text-[15px] font-medium text-text-main mb-1">{record.title}</div>
                  <div className="text-[12px] text-text-sub">{record.date}</div>
                </div>
              </div>
              <div className="text-right">
                <div className={`text-[16px] font-bold font-mono tracking-tight ${record.type === 'expense' ? 'text-text-main' : 'text-green-500'}`}>
                  {record.amount}
                </div>
                <div className="text-[11px] text-text-sub mt-1">{record.status === 'success' ? '交易成功' : record.status === 'pending' ? '处理中' : '交易失败'}</div>
              </div>
            </div>
          ))
        ) : (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub">
            <ReceiptText className="w-12 h-12 mb-4 opacity-20" />
            <p className="text-[14px]">暂无账单记录</p>
          </div>
        )}
      </div>
    </PageLayout>
  );
};

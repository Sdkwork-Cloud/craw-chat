import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { PageLayout, showToast } from "@sdkwork/im-h5-commons";
import {
  FileText,
  Target,
  AlertCircle,
  FilePlus,
  CloudUpload,
  Clock,
  Calendar,
} from "lucide-react";
import { useNavigate } from "react-router";
import { ReportService } from "../services/ReportService";
import { motion } from "motion/react";

export const CreateReport = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [formData, setFormData] = useState({
    type: "日报",
    summary: "",
    plan: "",
    issues: "",
  });

  const handleSubmit = async () => {
    if (!formData.summary) return showToast("请输入汇报内容");

    await ReportService.submitReport({
      type: formData.type,
      reporter: "王小明",
      date: new Date().toLocaleString(),
      summary: formData.summary,
    });

    showToast("提交成功");
    navigate(-1);
  };

  return (
    <PageLayout
      title="写汇报"
      rightElement={
        <span
          className="text-[16px] text-accent-blue font-medium active:opacity-60 cursor-pointer"
          onClick={handleSubmit}
        >提交</span>
      }
    >
      <div className="p-4 space-y-4">
        {/* Report Type */}
        <div className="flex gap-2">{["日报", "周报", "月报"].map((type) => (<button
              key={type}
              onClick={() => setFormData({ ...formData, type })}
              className={`flex-1 py-2 rounded-lg font-medium text-[15px] transition-colors ${
                formData.type === type
                  ? "bg-primary-blue text-white"
                  : "bg-white dark:bg-[#2c2d2e] border border-border-color/30 text-text-sub"
              }`}
            >
              {type}
            </button>
          ))}
        </div>

        {/* Content */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl overflow-hidden shadow-sm border border-border-color/30 flex flex-col">
          <div className="px-4 py-3 bg-bg-color/50 border-b border-border-color/30 flex items-center gap-2">
            <FileText className="w-4 h-4 text-primary-blue" />
            <span className="text-[14px] font-bold text-text-main">已完成工作</span>
          </div>
          <textarea
            className="w-full bg-transparent border-none outline-none text-[15px] text-text-main placeholder:text-text-sub/50 p-4 resize-none min-h-[120px]"
            placeholder="请输入今日(本周/本月)已完成的工作内容..."
            value={formData.summary}
            onChange={(e) =>
              setFormData({ ...formData, summary: e.target.value })
            }
          />
        </div>

        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl overflow-hidden shadow-sm border border-border-color/30 flex flex-col">
          <div className="px-4 py-3 bg-bg-color/50 border-b border-border-color/30 flex items-center gap-2">
            <Target className="w-4 h-4 text-orange-500" />
            <span className="text-[14px] font-bold text-text-main">工作计划</span>
          </div>
          <textarea
            className="w-full bg-transparent border-none outline-none text-[15px] text-text-main placeholder:text-text-sub/50 p-4 resize-none min-h-[100px]"
            placeholder="请输入下一步工作计划..."
            value={formData.plan}
            onChange={(e) => setFormData({ ...formData, plan: e.target.value })}
          />
        </div>

        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl overflow-hidden shadow-sm border border-border-color/30 flex flex-col">
          <div className="px-4 py-3 bg-bg-color/50 border-b border-border-color/30 flex items-center gap-2">
            <AlertCircle className="w-4 h-4 text-rose-500" />
            <span className="text-[14px] font-bold text-text-main">需协调问题</span>
            <span className="text-[12px] text-text-sub font-normal ml-auto">选填</span>
          </div>
          <textarea
            className="w-full bg-transparent border-none outline-none text-[15px] text-text-main placeholder:text-text-sub/50 p-4 resize-none min-h-[80px]"
            placeholder="是否有需要协调解决的问题..."
            value={formData.issues}
            onChange={(e) =>
              setFormData({ ...formData, issues: e.target.value })
            }
          />
        </div>

        {/* Attachments */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm border border-border-color/30">
          <h3 className="text-[14px] font-bold text-text-main mb-3">附件</h3>
          <div className="flex gap-2">
            <div
              className="w-16 h-16 rounded-xl bg-bg-color flex flex-col items-center justify-center cursor-pointer border border-dashed border-border-color active:bg-border-color/30"
              onClick={() => showToast("已选择附件")}
            >
              <FilePlus className="w-5 h-5 text-text-sub mb-1" />
              <span className="text-[10px] text-text-sub">添加</span>
            </div>
          </div>
        </div>
      </div>
    </PageLayout>
  );
};

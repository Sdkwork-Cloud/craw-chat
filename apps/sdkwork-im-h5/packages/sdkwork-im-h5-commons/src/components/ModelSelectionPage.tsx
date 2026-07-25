import { useTranslation } from "react-i18next";
import React, { useState, useMemo } from "react";
import { ChevronLeft, Check, Search, Cpu } from "lucide-react";
import { IconButton } from "./IconButton";
import { cn } from "../utils/cn";

export interface AIModel {
  id: string;
  name: string;
  description?: string;
  tags?: string[];
  icon?: React.ReactNode;
}

export interface ModelVendor {
  id: string;
  name: string;
  icon?: React.ReactNode;
  models: AIModel[];
}

export const ModelSelectionPage = ({
  title = "选择模型",
  currentModelId,
  vendors,
  onSelect,
  onClose,
}: {
  title?: string;
  currentModelId: string;
  vendors: ModelVendor[];
  onSelect: (model: AIModel, vendor: ModelVendor) => void;
  onClose: () => void;
}) => {
  const { t } = useTranslation();
  const [searchQuery, setSearchQuery] = useState("");
  const initialVendor = vendors.find((v) =>
    v.models.some((m) => m.id === currentModelId)
  );
  const [activeVendorId, setActiveVendorId] = useState<string>(
    initialVendor?.id || vendors[0]?.id || ""
  );

  const filteredVendors = useMemo(() => {
    if (!searchQuery.trim()) return vendors;
    const lowerQuery = searchQuery.toLowerCase();
    return vendors
      .map((v) => ({
        ...v,
        models: v.models.filter(
          (m) =>
            m.name.toLowerCase().includes(lowerQuery) ||
            m.description?.toLowerCase().includes(lowerQuery)
        ),
      }))
      .filter((v) => v.name.toLowerCase().includes(lowerQuery) || v.models.length > 0);
  }, [vendors, searchQuery]);

  // If active vendor is filtered out, select the first available one
  React.useEffect(() => {
    if (
      filteredVendors.length > 0 &&
      !filteredVendors.find((v) => v.id === activeVendorId)
    ) {
      setActiveVendorId(filteredVendors[0].id);
    }
  }, [filteredVendors, activeVendorId]);

  const activeVendor = filteredVendors.find((v) => v.id === activeVendorId);

  return (
    <div className="fixed inset-0 z-50 bg-bg-color flex flex-col animate-in slide-in-from-bottom">
      {/* Header */}
      <div className="flex-none flex items-center h-14 px-2 bg-bg-color border-b border-border-color/30">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="w-10 h-10"
          onClick={onClose}
        />
        <h1 className="flex-1 text-[16px] font-medium text-center mr-10 text-text-main">
          {title}
        </h1>
      </div>

      {/* Search Bar */}
      <div className="p-3 bg-bg-color border-b border-border-color/30">
        <div className="flex items-center bg-gray-100 dark:bg-[#2c2d2e] rounded-full px-3 py-2">
          <Search className="w-4 h-4 text-text-sub mr-2" />
          <input
            type="text"
            placeholder={t('commons.auto_prop_n459b5466', '搜索服务商或模型...')}
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="flex-1 bg-transparent text-[14px] text-text-main outline-none placeholder:text-text-sub"
          />
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Left Sidebar: Vendors */}
        <div className="w-24 flex-none bg-gray-50 dark:bg-[#1a1b1c] overflow-y-auto border-r border-border-color/30">
          {filteredVendors.map((vendor) => (
            <div
              key={vendor.id}
              onClick={() => setActiveVendorId(vendor.id)}
              className={cn(
                "py-4 px-2 text-center text-[13px] transition-colors relative cursor-pointer select-none",
                activeVendorId === vendor.id
                  ? "bg-bg-color text-primary-blue font-medium"
                  : "text-text-sub hover:bg-gray-100 dark:hover:bg-[#2c2d2e]"
              )}
            >
              {activeVendorId === vendor.id && (
                <div className="absolute left-0 top-0 bottom-0 w-1 bg-primary-blue" />
              )}
              <div className="flex flex-col items-center gap-1.5">
                {vendor.icon || <Cpu className="w-5 h-5 opacity-70" />}
                <span className="truncate w-full">{vendor.name}</span>
              </div>
            </div>
          ))}
        </div>

        {/* Right Content: Models */}
        <div className="flex-1 overflow-y-auto bg-bg-color p-3">
          {activeVendor ? (
            <div className="flex flex-col gap-3">
              {activeVendor.models.map((model) => (
                <div
                  key={model.id}
                  onClick={() => onSelect(model, activeVendor)}
                  className={cn(
                    "p-3 rounded-xl border flex flex-col gap-2 cursor-pointer transition-all active:scale-[0.98]",
                    currentModelId === model.id
                      ? "border-primary-blue bg-primary-blue/5 shadow-sm"
                      : "border-border-color/50 bg-white dark:bg-[#2c2d2e] shadow-sm hover:border-border-color"
                  )}
                >
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      {model.icon}
                      <span className="text-[15px] font-medium text-text-main">
                        {model.name}
                      </span>
                    </div>
                    {currentModelId === model.id && (
                      <Check className="w-5 h-5 text-primary-blue" />
                    )}
                  </div>
                  {model.description && (
                    <span className="text-[13px] text-text-sub leading-snug">
                      {model.description}
                    </span>
                  )}
                  {model.tags && model.tags.length > 0 && (
                    <div className="flex flex-wrap gap-1.5 mt-1">
                      {model.tags.map((tag) => (
                        <span
                          key={tag}
                          className="text-[11px] px-2 py-0.5 bg-gray-100 dark:bg-[#3a3b3c] text-text-sub rounded-md font-medium text-center"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  )}
                </div>
              ))}
              {activeVendor.models.length === 0 && (
                <div className="text-center text-text-sub text-[13px] py-10">{t('commons.auto_n430a3663', '没有找到符合条件的模型')}</div>
              )}
            </div>
          ) : (
            <div className="text-center text-text-sub text-[13px] py-10">{t('commons.auto_13d09251', '请选择一个服务商')}</div>
          )}
        </div>
      </div>
    </div>
  );
};

import React from "react";
import { Package, Building2, ChevronRight } from "lucide-react";

export interface SupplyItem {
  title: string;
  company: string;
  type: string;
  price: string;
}

export interface SupplyListTabProps {
  supplies: SupplyItem[];
}

export const SupplyListTab: React.FC<SupplyListTabProps> = ({ supplies }) => {
  return (
    <>
      {supplies.map((sup, i) => (
        <div
          key={i}
          className="px-4 py-3 border-b border-border-color/50 flex flex-col gap-2 active:bg-chat-active-bg transition-colors cursor-pointer group hover:bg-hover-bg relative overflow-hidden"
        >
          <div className="absolute inset-0 bg-primary-blue/5 opacity-0 group-active:opacity-100 transition-opacity" />
          <div className="flex justify-between items-start">
            <h3 className="text-[15px] font-bold text-text-main leading-snug w-3/4 group-hover:text-primary-blue transition-colors">
              {sup.title}
            </h3>
            <span className="text-[15px] font-extrabold text-[#FF7D00]">{sup.price}</span>
          </div>
          <div className="flex items-center text-[12px] text-text-sub">
            <Package className="w-3.5 h-3.5 mr-1 text-primary-blue/70" /> {sup.type}
          </div>
          <div className="flex items-center justify-between mt-1 pt-2 border-t border-border-color/30">
            <div className="flex items-center gap-1.5 opacity-90">
              <Building2 className="w-4 h-4 text-text-sub shrink-0" />
              <span className="text-[12px] text-text-sub font-medium">{sup.company}</span>
            </div>
            <ChevronRight className="w-4 h-4 text-text-sub opacity-50" />
          </div>
        </div>
      ))}
    </>
  );
};

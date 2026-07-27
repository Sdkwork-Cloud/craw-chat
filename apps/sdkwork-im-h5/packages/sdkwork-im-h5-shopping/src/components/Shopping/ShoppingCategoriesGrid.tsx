import React from "react";
import { useNavigate } from "react-router";
import {
  Flame,
  Ticket,
  Smartphone,
  Home,
  Shirt,
  Coffee,
  Sparkles,
  LayoutGrid,
} from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

interface ShoppingCategoriesGridProps {
  categories: string[];
}

export const ShoppingCategoriesGrid: React.FC<ShoppingCategoriesGridProps> = ({ categories }) => {
  const navigate = useNavigate();

  return (
    <div className="grid grid-cols-5 gap-y-4 px-2 py-3 mb-4 mx-2 bg-transparent">
      {categories.map((cat, i) => {
        let Icon = LayoutGrid;
        if (cat === "推荐") Icon = Flame;
        if (cat === "卡券") Icon = Ticket;
        if (cat === "数码家电") Icon = Smartphone;
        if (cat === "生活日用") Icon = Home;
        if (cat === "服饰箱包") Icon = Shirt;
        if (cat === "食品饮料") Icon = Coffee;
        if (cat === "美妆个护") Icon = Sparkles;

        return (
          <div
            key={i}
            className="flex flex-col items-center justify-center gap-1.5 cursor-pointer active:scale-95 transition-transform group"
            onClick={() => navigate(`/category/${encodeURIComponent(cat)}`)}
          >
            <div
              className={cn(
                "w-11 h-11 rounded-2xl flex items-center justify-center shadow-sm transition-all duration-300",
                cat === "卡券"
                  ? "bg-gradient-to-br from-[#FA5151] to-[#FF8C8C] text-white shadow-[#FA5151]/20 group-hover:shadow-md"
                  : "bg-gray-100 dark:bg-white/10 text-text-main group-hover:bg-gray-200 dark:group-hover:bg-white/15"
              )}
            >
              <Icon
                className={cn(
                  "w-[22px] h-[22px]",
                  cat === "卡券" ? "text-white" : "text-text-main"
                )}
              />
            </div>
            <span className="text-[11px] font-medium transition-colors text-text-main">
              {cat}
            </span>
          </div>
        );
      })}
    </div>
  );
};

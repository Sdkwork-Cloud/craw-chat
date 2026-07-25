import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { Search } from "lucide-react";
import { cn, showToast, ActionSheet } from "@sdkwork/im-h5-commons";
import { PageLayout } from "../components/PageLayout";
import { FavoriteCard, type FavoriteItem } from "../components/FavoriteCard";

export const FavoritesPage = () => {
  const { t } = useTranslation();
const [searchQuery, setSearchQuery] = useState("");
  const [activeFilter, setActiveFilter] = useState("all");
  const [actionSheetItem, setActionSheetItem] = useState<FavoriteItem | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);

  const startLongPress = (item: FavoriteItem) => {
  const handlePressStart = () => {
  setIsLongPressed(false);
      (window as any).longPressTimeout = setTimeout(() => {
        setIsLongPressed(true);
        setActionSheetItem(item);
      }, 500);
    };

    const handlePressEnd = () => {
  clearTimeout((window as any).longPressTimeout);
    };

    return {
      onPointerDown: handlePressStart,
      onPointerUp: handlePressEnd,
      onPointerLeave: () => {
        handlePressEnd();
        setIsLongPressed(false);
      },
      onContextMenu: (e: React.MouseEvent) => {
        e.preventDefault();
        handlePressStart();
        setIsLongPressed(true);
        setActionSheetItem(item);
        handlePressEnd();
      }
    };
  };

  const handleFilterClick = (id: string, e: React.MouseEvent<HTMLDivElement>) => {
  setActiveFilter(id);
    const target = e.currentTarget;
    const parent = target.parentElement;
    if (parent) {
      // Calculate position relative to scroll container
      const scrollLeft = target.offsetLeft - parent.clientWidth / 2 + target.clientWidth / 2 - parent.offsetLeft;
      parent.scrollTo({ left: scrollLeft, behavior: 'smooth' });
    }
  };

  const handleActionSheetSelect = (action: string) => {
  if (!actionSheetItem) return;
    if (action === 'delete') {
       showToast(t('user.auto_fn_5372638f', '已删除收藏'));
    } else if (action === 'share') {
       showToast(t('user.auto_fn_16ae6d7', '已分享'));
    }
    setActionSheetItem(null);
  };

  const FILTERS = [
    { id: "all", label: "全部" },
    { id: "link", label: "链接" },
    { id: "article", label: "文章" },
    { id: "image", label: "图片与视频" },
    { id: "file", label: "文件" },
    { id: "voice", label: "语音" },
    { id: "chat", label: "聊天记录" },
  ];

  const FAVORITES: FavoriteItem[] = [
    {
      id: "1",
      title: "如何高效利用时间工作？(深度好文)",
      type: "article",
      typeLabel: "文章",
      time: "昨天",
      source: "效率黑客",
      preview: "时间管理不是为了让你做更多的事情，而是为了让你能够做最重要的事情...",
      icon: "FileText",
      color: "text-blue-500",
    },
    {
      id: "2",
      title: "公司年度旅游照片合集",
      type: "image",
      typeLabel: "相册",
      time: "2023-10-01",
      source: "HR 部门",
      preview: "[9张图片]",
      icon: "Image",
      color: "text-green-500",
    },
    {
      id: "3",
      title: "王总语音记录 (关于项目调整)",
      type: "voice",
      typeLabel: "语音",
      time: "2023-09-15",
      source: "微信聊天",
      preview: "[语音 45秒]",
      icon: "Mic",
      color: "text-orange-500",
    },
    {
      id: "4",
      title: "Sdkwork IM H5 Q3 研发计划.pdf",
      type: "file",
      typeLabel: "文件",
      time: "2023-09-10",
      source: "工作群",
      preview: "4.2 MB",
      icon: "File",
      color: "text-purple-500",
    },
    {
      id: "5",
      title: "GitHub - facebook/react",
      type: "link",
      typeLabel: "链接",
      time: "2023-09-01",
      source: "张三",
      preview: "A declarative, efficient, and flexible JavaScript library for building user interfaces.",
      icon: "Link",
      color: "text-blue-400",
    },
    {
      id: "6",
      title: "关于系统架构升级的讨论",
      type: "chat",
      typeLabel: "聊天记录",
      time: "2023-08-20",
      source: "研发一组",
      preview: "李四: 我们需要重构网关...\n王五: 赞同，当前性能瓶颈明显。",
      icon: "MessageCircle",
      color: "text-emerald-500",
    }
  ];

  const filteredFavorites = FAVORITES.filter((item) => {
    const matchSearch = item.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
                        item.source.toLowerCase().includes(searchQuery.toLowerCase()) ||
                        item.preview.toLowerCase().includes(searchQuery.toLowerCase());
    const matchFilter = activeFilter === "all" || item.type === activeFilter;
    return matchSearch && matchFilter;
  });

  return (
    <PageLayout title={t('user.auto_prop_2e5dc52c', '我的收藏')} bgClass="bg-[#F8F9FA] dark:bg-black">
      <div className="bg-white dark:bg-[#1A1A1A] sticky top-0 z-20 shadow-sm border-b border-border-color flex flex-col">
         <div className="px-4 py-3 pb-2">
            <div className="bg-gray-100 dark:bg-white/5 rounded-full flex items-center h-10 px-3.5 gap-2 border border-transparent focus-within:border-primary-blue/30 transition-colors">
              <Search className="w-[18px] h-[18px] text-text-sub shrink-0" strokeWidth={2} />
              <input 
                 value={searchQuery}
                 onChange={e => setSearchQuery(e.target.value)}
                 className="flex-1 bg-transparent text-[15px] text-text-main outline-none placeholder:text-text-sub"
                 placeholder={t('user.auto_prop_n75c995cb', '搜索收藏的内容...')}
              />
            </div>
         </div>
         <div className="flex overflow-x-auto hide-scrollbar px-3 pb-1 pt-1 gap-2">
           {FILTERS.map(f => (
             <div 
               key={f.id}
               onClick={(e) => handleFilterClick(f.id, e)}
               className={cn(
                 "px-4 py-1.5 rounded-full text-[13px] font-medium whitespace-nowrap transition-colors cursor-pointer border",
                 activeFilter === f.id 
                   ? "bg-primary-blue/10 text-primary-blue border-primary-blue/30 dark:bg-blue-900/30 dark:border-blue-800" 
                   : "bg-transparent text-text-sub border-transparent hover:bg-gray-100 dark:hover:bg-[#2A2A2D]"
               )}
             >
               {f.label}
             </div>
           ))}
         </div>
      </div>
      
      <div className="flex-1 overflow-y-auto w-full relative">
        <div className="flex flex-col bg-white dark:bg-[#1C1C1E] min-h-full">
          {filteredFavorites.length > 0 ? (
            filteredFavorites.map((item) => (
              <FavoriteCard
                key={item.id}
                item={item}
                onClick={() => {
                  if (isLongPressed) {
                    setIsLongPressed(false);
                    return;
                  }
                  showToast(`打开: ${item.title}`);
                }}
                onLongPressProps={startLongPress(item)}
              />
            ))
          ) : (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub bg-[#F8F9FA] dark:bg-black h-full">
               <div className="w-16 h-16 bg-gray-100 dark:bg-[#1A1A1A] rounded-full flex items-center justify-center mb-4">
                  <Search className="w-8 h-8 opacity-40" />
               </div>
               <p className="text-[15px] font-medium">{t('user.auto_n5f1732f2', '没有找到相关收藏')}</p>
               <p className="text-[13px] mt-1 opacity-70">{t('user.auto_2109d03d', '换个关键词试试吧')}</p>
            </div>
          )}
        </div>
        
        {actionSheetItem && (
          <ActionSheet
            isOpen={true}
            title={`${actionSheetItem.title} - 操作`}
            options={[
              { label: '分享', onClick: () => handleActionSheetSelect('share') },
              { label: '删除', danger: true, onClick: () => handleActionSheetSelect('delete') }
            ]}
            onClose={() => setActionSheetItem(null)}
          />
        )}
      </div>
    </PageLayout>
  );
};

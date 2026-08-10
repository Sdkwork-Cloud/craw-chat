import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { Search } from "lucide-react";
import { cn, showToast, ActionSheet } from "@sdkwork/im-h5-commons";
import { PageLayout } from "../components/PageLayout";
import { FavoriteCard, type FavoriteItem } from "../components/FavoriteCard";
import { favoriteService } from "../services/FavoriteService";

export const FavoritesPage = () => {
  const { t } = useTranslation();
const [searchQuery, setSearchQuery] = useState("");
  const [activeFilter, setActiveFilter] = useState("all");
  const [actionSheetItem, setActionSheetItem] = useState<FavoriteItem | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);
  const [favorites, setFavorites] = useState<FavoriteItem[]>([]);
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    let cancelled = false;
    favoriteService
      .getFavorites()
      .then((items) => {
        if (!cancelled) {
          setFavorites(items);
          setLoaded(true);
        }
      })
      .catch((error) => {
        console.error("Failed to load favorites", error);
        if (!cancelled) {
          setLoaded(true);
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

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
       const item = actionSheetItem;
       favoriteService
         .removeFavorite(item.id)
         .then(() => {
           setFavorites((current) => current.filter((entry) => entry.id !== item.id));
           showToast(t('user.auto_fn_5372638f', '已删除收藏'));
         })
         .catch((error) => {
           console.error("Failed to remove favorite", error);
           showToast(t('user.auto_2109d03d', '删除失败，请稍后重试'));
         });
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

  const filteredFavorites = favorites.filter((item) => {
    const matchSearch = item.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
                        item.source.toLowerCase().includes(searchQuery.toLowerCase()) ||
                        item.preview.toLowerCase().includes(searchQuery.toLowerCase());
    const matchFilter = activeFilter === "all" || item.type === activeFilter;
    return matchSearch && matchFilter;
  });

  return (
    <PageLayout title={t('user.auto_prop_2e5dc52c', '我的收藏')} bgClass="bg-bg-color">
      <div className="bg-chat-other-bg sticky top-0 z-20 shadow-sm border-b border-border-color flex flex-col">
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
                   : "bg-transparent text-text-sub border-transparent hover:bg-hover-bg"
               )}
             >
               {f.label}
             </div>
           ))}
         </div>
      </div>
      
      <div className="flex-1 overflow-y-auto w-full relative">
        <div className="flex flex-col bg-chat-other-bg min-h-full">
          {loaded && filteredFavorites.length > 0 ? (
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
          ) : loaded ? (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub bg-bg-color h-full">
               <div className="w-16 h-16 bg-hover-bg rounded-full flex items-center justify-center mb-4">
                  <Search className="w-8 h-8 opacity-40" />
               </div>
               <p className="text-[15px] font-medium">{t('user.auto_n5f1732f2', '没有找到相关收藏')}</p>
               <p className="text-[13px] mt-1 opacity-70">{t('user.auto_2109d03d', '换个关键词试试吧')}</p>
            </div>
          ) : null}
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

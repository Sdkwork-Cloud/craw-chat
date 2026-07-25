import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { IconButton } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Search, Filter, ArrowUp, ArrowDown } from "lucide-react";
import { ProductService } from "../services/ProductService";
import { ProductCard, getAspectRatio } from "../components/ProductCard";
import { CouponCard } from "../components/CouponCard";
import { Product } from "../types";

type SortType = 'comprehensive' | 'sales' | 'price';

export const CategoryPage = () => {
  const { t } = useTranslation();
const { categoryName } = useParams<{ categoryName: string }>();
  const navigate = useNavigate();
  const [products, setProducts] = useState<Product[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [sortType, setSortType] = useState<SortType>('comprehensive');
  const [priceAsc, setPriceAsc] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    ProductService.getProducts().then((allProducts) => {
      // Basic logic to filter
      const decodedCategory = decodeURIComponent(categoryName || "");
      const filtered = decodedCategory === "全部" || decodedCategory === "推荐" 
        ? allProducts 
        : allProducts.filter(p => p.categoryId === decodedCategory || p.title.includes(decodedCategory.substring(0, 1)));
      setProducts(filtered);
      setIsLoading(false);
    });
  }, [categoryName]);

  const decodedCategory = decodeURIComponent(categoryName || "");

  const displayProducts = [...products]
    .filter(p => p.title.toLowerCase().includes(searchQuery.toLowerCase()))
    .sort((a, b) => {
      if (sortType === 'sales') {
        const salesA = parseInt(a.sales.replace(/[^0-9]/g, '')) || 0;
        const salesB = parseInt(b.sales.replace(/[^0-9]/g, '')) || 0;
        return salesB - salesA;
      }
      if (sortType === 'price') {
        const priceA = parseFloat(a.price);
        const priceB = parseFloat(b.price);
        return priceAsc ? priceA - priceB : priceB - priceA;
      }
      return 0; // comprehensive
    });

  const handlePriceClick = () => {
  if (sortType === 'price') {
      setPriceAsc(!priceAsc);
    } else {
      setSortType('price');
      setPriceAsc(true);
    }
  };

  const leftColumn: Product[] = [];
  const rightColumn: Product[] = [];
  let leftHeight = 0;
  let rightHeight = 0;

  displayProducts.forEach((p) => {
    const ratio = getAspectRatio(p.image);
    // Estimated height: inverse of aspect ratio * 100 for percentage width
    // Add ~70 for the text and padding height
    const estimatedHeight = parseInt(((1 / ratio) * 100).toString()) + 70;
    
    if (leftHeight <= rightHeight) {
      leftColumn.push(p);
      leftHeight += estimatedHeight;
    } else {
      rightColumn.push(p);
      rightHeight += estimatedHeight;
    }
  });

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Search Header */}
      <header className="h-[52px] flex items-center px-2 bg-chat-other-bg sticky top-0 z-10 shrink-0 pt-safe">
        <div 
          className="w-10 h-10 flex items-center justify-center shrink-0 cursor-pointer active:opacity-70"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <div className="flex-1 flex items-center bg-black/5 dark:bg-white/10 rounded-full px-3 py-1.5 h-[34px] ml-1 mr-2">
          <Search className="w-[15px] h-[15px] text-text-sub mr-1.5 shrink-0" />
          <input 
            type="text" 
            placeholder={`搜索${decodedCategory}商品`}
            className="flex-1 bg-transparent border-none outline-none text-[13px] text-text-main placeholder:text-text-sub/70 min-w-0"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          {searchQuery && (
            <div 
              className="w-4 h-4 ml-2 rounded-full bg-black/20 dark:bg-white/20 flex items-center justify-center cursor-pointer shrink-0"
              onClick={() => setSearchQuery("")}
            >
              <div className="w-2 h-[1px] bg-white rotate-45 absolute" />
              <div className="w-2 h-[1px] bg-white -rotate-45 absolute" />
            </div>
          )}
        </div>
        <div className="text-[14px] text-text-main font-medium pl-1 pr-2 shrink-0 cursor-pointer active:opacity-70">{t('shopping.auto_c9c86', '搜索')}</div>
      </header>

      {/* Filter Tabs */}
      <div className="flex items-center px-2 h-10 bg-chat-other-bg border-b border-border-color sticky top-[calc(env(safe-area-inset-top)+52px)] z-10 shrink-0 select-none">
        <div 
          className="flex-1 flex justify-center items-center cursor-pointer active:opacity-70 transition-opacity"
          onClick={() => setSortType('comprehensive')}
        >
          <span className={`text-[14px] font-medium transition-colors ${sortType === 'comprehensive' ? 'text-primary-blue' : 'text-text-sub'}`}>{t('shopping.auto_fb48c', '综合')}</span>
        </div>
        <div 
          className="flex-1 flex justify-center items-center cursor-pointer active:opacity-70 transition-opacity"
          onClick={() => setSortType('sales')}
        >
          <span className={`text-[14px] font-medium transition-colors ${sortType === 'sales' ? 'text-primary-blue' : 'text-text-sub'}`}>{t('shopping.auto_129ccf', '销量')}</span>
        </div>
        <div 
          className="flex-1 flex justify-center items-center cursor-pointer active:opacity-70 transition-opacity"
          onClick={handlePriceClick}
        >
          <div className="flex items-center gap-0.5">
            <span className={`text-[14px] font-medium transition-colors ${sortType === 'price' ? 'text-primary-blue' : 'text-text-sub'}`}>{t('shopping.auto_9f825', '价格')}</span>
            <div className="flex flex-col ml-0.5 -space-y-1">
              <ArrowUp className={`w-3 h-3 ${sortType === 'price' && priceAsc ? 'text-primary-blue' : 'text-text-sub/50'}`} />
              <ArrowDown className={`w-3 h-3 ${sortType === 'price' && !priceAsc ? 'text-primary-blue' : 'text-text-sub/50'}`} />
            </div>
          </div>
        </div>
        <div 
          className="flex-1 flex justify-center items-center cursor-pointer active:opacity-70 transition-opacity gap-1"
        >
          <span className="text-[14px] font-medium text-text-sub">{t('shopping.auto_f800e', '筛选')}</span>
          <Filter className="w-3.5 h-3.5 text-text-sub" />
        </div>
      </div>

      <div className="flex-1 overflow-y-auto w-full px-2 pt-2 pb-10 bg-bg-color">
        {isLoading ? (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
            <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
            <p className="text-[14px]">{t('shopping.auto_7f6f37e', '加载中...')}</p>
          </div>
        ) : displayProducts.length > 0 ? (
          decodedCategory === '卡券' ? (
            <div className="flex flex-col gap-2">
              {displayProducts.map((p) => (
                <CouponCard 
                  key={p.id} 
                  product={p} 
                  onClick={() => navigate(`/product/${p.id}`)} 
                />
              ))}
            </div>
          ) : (
            <div className="flex items-start gap-2">
              <div className="flex-1 flex flex-col gap-2">
                {leftColumn.map((p) => (
                  <ProductCard 
                    key={p.id} 
                    product={p} 
                    onClick={() => navigate(`/product/${p.id}`)} 
                  />
                ))}
              </div>
              <div className="flex-1 flex flex-col gap-2">
                {rightColumn.map((p) => (
                  <ProductCard 
                    key={p.id} 
                    product={p} 
                    onClick={() => navigate(`/product/${p.id}`)} 
                  />
                ))}
              </div>
            </div>
          )
        ) : (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub">
            <p className="text-[14px]">{t('shopping.auto_2c8c005d', '{searchQuery ? "未找到匹配的商品" : `暂无${decodedCategory}商品`}')}</p>
          </div>
        )}
      </div>
    </div>
  );
};


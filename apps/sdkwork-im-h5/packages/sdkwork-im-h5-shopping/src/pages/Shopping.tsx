import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ShoppingCart, ChevronLeft } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { ProductService } from "../services/ProductService";
import { Product } from "../types";
import { useCartStore } from "../store/useCartStore";
import { ShoppingSearchBarAndBanner } from "../components/Shopping/ShoppingSearchBarAndBanner";
import { ShoppingCategoriesGrid } from "../components/Shopping/ShoppingCategoriesGrid";
import { ShoppingWaterfallGrid } from "../components/Shopping/ShoppingWaterfallGrid";

const PageLayout = ({
  title,
  children,
  rightElement = null,
}: {
  title?: string;
  children: React.ReactNode;
  rightElement?: React.ReactNode;
}) => {
  const navigate = useNavigate();
  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      <header className="flex items-center px-2 pt-safe h-[56px] shrink-0 sticky top-0 bg-bg-color/80 backdrop-blur-md z-10">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{title}</h2>
        </div>
        <div className="flex-1 flex justify-end pr-1">{rightElement}</div>
      </header>
      <div className="flex flex-col px-0 sm:px-4 pb-12 mt-2">{children}</div>
    </div>
  );
};

export const ShoppingPage = () => {
  const [products, setProducts] = useState<Product[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const navigate = useNavigate();
  const { items, loadCart } = useCartStore();

  useEffect(() => {
    setIsLoading(true);
    Promise.all([
      ProductService.getProducts().then(setProducts),
      ProductService.getCategories().then((cats) => {
        setCategories(["全部", ...cats]);
      })
    ]).then(() => setIsLoading(false));
    loadCart();
  }, []);

  const cartItemCount = items.reduce((acc, item) => acc + item.quantity, 0);

  return (
    <PageLayout
      rightElement={
        <div
          className="relative cursor-pointer"
          onClick={() => navigate("/cart")}
        >
          <IconButton
            icon={<ShoppingCart className="w-[22px] h-[22px] text-text-main" />}
          />
          {cartItemCount > 0 && (
            <span className="absolute top-1 right-1 bg-[#FA5151] text-white text-[10px] scale-90 px-1.5 py-0.5 rounded-full border border-white pointer-events-none">
              {cartItemCount}
            </span>
          )}
        </div>
      }
    >
      <ShoppingSearchBarAndBanner />
      <ShoppingCategoriesGrid categories={categories} />
      <div className="px-2 pb-10 min-h-[300px]">
        <ShoppingWaterfallGrid isLoading={isLoading} products={products} />
      </div>
    </PageLayout>
  );
};

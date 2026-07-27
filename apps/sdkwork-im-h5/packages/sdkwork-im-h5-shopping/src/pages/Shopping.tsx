import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ShoppingCart } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { ProductService } from "../services/ProductService";
import { Product } from "../types";
import { useCartStore } from "../store/useCartStore";
import { ShoppingSearchBarAndBanner } from "../components/Shopping/ShoppingSearchBarAndBanner";
import { ShoppingCategoriesGrid } from "../components/Shopping/ShoppingCategoriesGrid";
import { ShoppingWaterfallGrid } from "../components/Shopping/ShoppingWaterfallGrid";
import { ShoppingPageLayout } from "../components/Shopping/ShoppingPageLayout";

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
    <ShoppingPageLayout
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
    </ShoppingPageLayout>
  );
};

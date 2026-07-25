import React from "react";
import { ProductCard, getAspectRatio } from "../ProductCard";
import { Product } from "../../types";

interface ShoppingWaterfallGridProps {
  isLoading: boolean;
  products: Product[];
}

export const ShoppingWaterfallGrid: React.FC<ShoppingWaterfallGridProps> = ({
  isLoading,
  products,
}) => {
  const leftColumn: Product[] = [];
  const rightColumn: Product[] = [];
  let leftHeight = 0;
  let rightHeight = 0;

  products.forEach((p) => {
    const ratio = getAspectRatio(p.image);
    const estimatedHeight = parseInt(((1 / ratio) * 100).toString()) + 70;

    if (leftHeight <= rightHeight) {
      leftColumn.push(p);
      leftHeight += estimatedHeight;
    } else {
      rightColumn.push(p);
      rightHeight += estimatedHeight;
    }
  });

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
        <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin mb-2" />
        <span className="text-xs">加载好物中...</span>
      </div>
    );
  }

  return (
    <div className="flex gap-2 items-start">
      <div className="flex-1 flex flex-col gap-2">
        {leftColumn.map((product) => (
          <ProductCard key={product.id} product={product} />
        ))}
      </div>
      <div className="flex-1 flex flex-col gap-2">
        {rightColumn.map((product) => (
          <ProductCard key={product.id} product={product} />
        ))}
      </div>
    </div>
  );
};

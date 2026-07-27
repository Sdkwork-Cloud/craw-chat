import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import {
  ChevronLeft,
  ShoppingCart,
  Share2,
  Store,
  Headphones,
  ChevronRight,
  X,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ProductService } from "../services/ProductService";
import { useCartStore } from "../store/useCartStore";
import { Product, Shop } from "../types";
import { ProductSkuPanel } from "../components/Shopping/ProductSkuPanel";
import { ProductHeader } from "../components/ProductDetails/ProductHeader";
import { ProductInfo } from "../components/ProductDetails/ProductInfo";
import { ProductShopCard } from "../components/ProductDetails/ProductShopCard";
import { ProductDetailsMock } from "../components/ProductDetails/ProductDetailsMock";
import { ProductBottomActions } from "../components/ProductDetails/ProductBottomActions";
import { ProductSpecBar } from "../components/ProductDetails/ProductSpecBar";

export const ProductDetails = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [product, setProduct] = useState<Product | null>(null);
  const [shop, setShop] = useState<Shop | null>(null);
  const { addToCart, items, loadCart } = useCartStore();
  const cartItemCount = items.reduce((acc, item) => acc + item.quantity, 0);

  // SKU Panel State
  const [showSkuPanel, setShowSkuPanel] = useState(false);
  const [skuAction, setSkuAction] = useState<"cart" | "buy">("buy");
  const [quantity, setQuantity] = useState(1);
  const [selectedSpecs, setSelectedSpecs] = useState<Record<string, string>>({});

  useEffect(() => {
    if (id) {
      ProductService.getProductById(id).then((p) => {
        setProduct(p);
        if (p?.shopId) {
          ProductService.getShopById(p.shopId).then(setShop);
        }
        if (p?.specs && p.specs.length > 0) {
          const defaultSpecs: Record<string, string> = {};
          p.specs.forEach(s => {
            if (s.options.length > 0) defaultSpecs[s.id] = s.options[0].id;
          });
          setSelectedSpecs(defaultSpecs);
        }
      });
    }
    loadCart();
  }, [id]);

  const currentSku = product?.skus?.find(sku => 
    Object.keys(selectedSpecs).every(key => sku.specValues[key] === selectedSpecs[key])
  );

  const displayPrice = currentSku?.price || product?.price;
  const displayOriginalPrice = currentSku?.originalPrice || product?.originalPrice;

  const handleAddToCartClick = () => {
  setSkuAction("cart");
    setShowSkuPanel(true);
  };

  const handleBuyNowClick = () => {
  setSkuAction("buy");
    setShowSkuPanel(true);
  };

  const handleConfirmSku = async () => {
    if (!product) return;
    
    if (product.specs && !currentSku) {
      showToast("请选择完整的商品规格");
      return;
    }

    setShowSkuPanel(false);

    if (skuAction === "cart") {
      await addToCart(product, quantity, currentSku, selectedSpecs);
      showToast("已加入购物车");
    } else {
      const skuQuery = currentSku ? `&skuId=${currentSku.id}` : '';
      navigate(`/checkout?productId=${product.id}&quantity=${quantity}${skuQuery}`);
    }
  };

  if (!product) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
        <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
        <span className="text-[14px]">加载中...</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <ProductHeader />

      <div className="flex-1 overflow-y-auto pb-[70px]">
        {/* Product Image */}
        <div className="w-full aspect-square bg-chat-other-bg">
          <img
            src={product.image}
            className="w-full h-full object-cover"
            alt={product.title}
          />
        </div>

        <ProductInfo 
          product={product} 
          displayPrice={displayPrice} 
          displayOriginalPrice={displayOriginalPrice} 
        />

        <ProductSpecBar 
          product={product} 
          currentSku={currentSku} 
          quantity={quantity} 
          onClick={() => {
            setSkuAction("buy"); // default action
            setShowSkuPanel(true);
          }}
        />

        {shop && <ProductShopCard shop={shop} />}

        <ProductDetailsMock product={product} />
      </div>

      <ProductBottomActions 
        shop={shop} 
        cartItemCount={cartItemCount} 
        onAddToCartClick={handleAddToCartClick} 
        onBuyNowClick={handleBuyNowClick} 
      />

      {/* SKU Panel Overlay */}
      {showSkuPanel && (
        <ProductSkuPanel
          product={product}
          currentSku={currentSku}
          displayPrice={displayPrice}
          quantity={quantity}
          setQuantity={setQuantity}
          selectedSpecs={selectedSpecs}
          setSelectedSpecs={setSelectedSpecs}
          setShowSkuPanel={setShowSkuPanel}
          skuAction={skuAction}
          handleConfirmSku={handleConfirmSku}
        />
      )}
    </div>
  );
};

import React, { useRef, useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import { ChevronLeft, RotateCcw, Crop, Palette, Check } from "lucide-react";
import SignatureCanvas from "react-signature-canvas";
import { GLOBAL_STORE } from "./CreateNotaryProcess";
import { showToast } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";
import { ColorSettingsModal } from "../components/ColorSettingsModal";
import { RatioSelectionSheet } from "../components/RatioSelectionSheet";

export const NotaryPartySignature: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id } = useParams();
  const sigCanvas = useRef<SignatureCanvas>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  const ratioOptions = [
    { label: t("notary.signature.ratio_16_9"), value: 16 / 9, desc: t("notary.signature.desc_16_9") },
    { label: t("notary.signature.ratio_2_1"), value: 2, desc: t("notary.signature.desc_2_1") },
    { label: t("notary.signature.ratio_4_3"), value: 4 / 3, desc: t("notary.signature.desc_4_3") },
    { label: t("notary.signature.ratio_1_1"), value: 1, desc: t("notary.signature.desc_1_1") },
  ];

  const colorOptions = [
    { label: t("notary.signature.color_black"), value: "#1a1a1a" },
    { label: t("notary.signature.color_ink_blue"), value: "#1e3a8a" },
    { label: t("notary.signature.color_royal_blue"), value: "#2563eb" },
    { label: t("notary.signature.color_red"), value: "#dc2626" },
    { label: t("notary.signature.color_green"), value: "#16a34a" },
  ];

  const [party, setParty] = useState<any>(null);
  const [canvasSize, setCanvasSize] = useState({ width: 300, height: 400 });
  const [containerDims, setContainerDims] = useState({ width: 300, height: 400 });
  const wrapperRef = useRef<HTMLDivElement>(null);
  
  const [penColor, setPenColor] = useState<string>("#1a1a1a");
  const [penWidth, setPenWidth] = useState<number>(2.5); // mid weight
  const [showColorPopup, setShowColorPopup] = useState(false);
  const [showRatioSheet, setShowRatioSheet] = useState(false);
  const [ratio, setRatio] = useState<number>(2);

  useEffect(() => {
    if (id) {
      const found = GLOBAL_STORE.parties.find((p) => p.id === id);
      if (found) {
        setParty(found);
      }
    }
  }, [id]);

  useEffect(() => {
    const updateSize = () => {
  if (wrapperRef.current) {
        setContainerDims({
          width: wrapperRef.current.offsetWidth,
          height: wrapperRef.current.offsetHeight,
        });
      }
    };

    updateSize();
    const observer = new ResizeObserver(updateSize);
    if (wrapperRef.current) observer.observe(wrapperRef.current);
    window.addEventListener("resize", updateSize);
    return () => {
      observer.disconnect();
      window.removeEventListener("resize", updateSize);
    };
  }, []);

  useEffect(() => {
    const padding = 0; // Maximize available width
    const maxW = containerDims.width - padding;
    const maxH = containerDims.height - padding;

    if (maxW <= 0 || maxH <= 0) return;

    if (ratio !== null) {
      let newW = maxW;
      let newH = newW / ratio;
      if (newH > maxH) {
        newH = maxH;
        newW = newH * ratio;
      }
      setCanvasSize({ width: newW, height: newH });
    } else {
      setCanvasSize({ width: maxW, height: maxH });
    }
  }, [ratio, containerDims]);

  useEffect(() => {
    if (party && party.signature && sigCanvas.current && canvasSize.width > 0) {
      setTimeout(() => {
        sigCanvas.current?.fromDataURL(party.signature, { ratio: 1 });
      }, 50);
    }
  }, [party, canvasSize]);

  const handleClear = () => {
  sigCanvas.current?.clear();
  };

  const handleConfirm = () => {
  if (sigCanvas.current?.isEmpty() && !party?.signature) {
      showToast(t("notary.signature.error_empty"));
      return;
    }

    let canvas = sigCanvas.current?.getCanvas();
    if (!canvas) return;

    let dataUrl = canvas.toDataURL("image/png");

    if (ratio !== null) {
      let targetWidth = canvas.width;
      let targetHeight = canvas.height;

      const padding = Math.max(targetWidth, targetHeight) * 0.1;
      targetWidth += padding * 2;
      targetHeight += padding * 2;

      if (targetWidth / targetHeight > ratio) {
        targetHeight = targetWidth / ratio;
      } else {
        targetWidth = targetHeight * ratio;
      }

      const outCanvas = document.createElement("canvas");
      const pixelRatio = window.devicePixelRatio || 2;
      outCanvas.width = targetWidth * pixelRatio;
      outCanvas.height = targetHeight * pixelRatio;
      const ctx = outCanvas.getContext("2d");

      if (ctx) {
        const offsetX = (outCanvas.width - canvas.width * pixelRatio) / 2;
        const offsetY = (outCanvas.height - canvas.height * pixelRatio) / 2;
        ctx.drawImage(
          canvas,
          offsetX,
          offsetY,
          canvas.width * pixelRatio,
          canvas.height * pixelRatio
        );
        dataUrl = outCanvas.toDataURL("image/png");
      }
    }

    GLOBAL_STORE.parties = GLOBAL_STORE.parties.map((p) => {
      if (p.id === id) {
        return { ...p, signature: dataUrl };
      }
      return p;
    });

    showToast(t("notary.signature.success"));
    navigate(-1);
  };

  return (
    <div className="flex flex-col h-full bg-[#f8f9fa] dark:bg-black text-text-main fixed inset-0 z-[100] animate-in slide-in-from-right font-sans">
      <header className="h-[44px] flex items-center justify-between shrink-0 pt-safe px-2 z-20 bg-white/80 dark:bg-black/80 backdrop-blur-md border-b border-border-color shadow-sm">
        <div className="flex items-center z-10 w-16">
          <button onClick={() => navigate(-1)} className="p-2 active:opacity-70 transition-opacity">
            <ChevronLeft className="w-6 h-6 text-text-main" strokeWidth={2.5} />
          </button>
        </div>
        <div className="flex flex-col items-center justify-center font-semibold text-[17px] pointer-events-none tracking-tight">
          {t("notary.signature.title")}
          {party && (
            <span className="text-[11px] font-normal text-text-sub leading-tight">
              {party.name} · {party.role || t("notary.signature.signatory")}
            </span>
          )}
        </div>
        <div className="flex items-center justify-end z-10 w-16 px-2"></div>
      </header>

      <div className="flex-1 flex flex-col pb-[100px] relative">
        <div className="flex items-center justify-between mb-2 mt-4 px-4">
          <div className="text-[14px] text-text-sub font-medium relative pl-3">
            <span className="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-3 bg-primary-blue rounded-full"></span>
            {t("notary.signature.write_prompt")}
          </div>
          <button 
            onClick={handleClear} 
            className="flex items-center gap-1.5 active:opacity-70 text-text-sub transition-opacity bg-white dark:bg-[#1a1b1c] px-3 py-1.5 rounded-full shadow-sm border border-border-color/50"
          >
            <RotateCcw className="w-4 h-4" strokeWidth={2} />
            <span className="text-[13px] font-medium">{t("notary.signature.rewrite")}</span>
          </button>
        </div>

        <div
          ref={wrapperRef}
          className="flex-1 w-full flex items-center justify-center min-h-0 overflow-hidden relative bg-[#eef1f6] dark:bg-[#151617] border-y border-border-color/50 shadow-inner"
        >
          <motion.div
            ref={containerRef}
            layout
            transition={{ type: "spring", stiffness: 300, damping: 30 }}
            className="bg-white rounded-none shadow-xl overflow-hidden relative flex-shrink-0"
            style={{ width: canvasSize.width, height: canvasSize.height }}
          >
            {/* Elegant paper texture background */}
            <div className="absolute inset-0 z-0 bg-[linear-gradient(to_right,#e5e7eb_1px,transparent_1px),linear-gradient(to_bottom,#e5e7eb_1px,transparent_1px)] dark:bg-[linear-gradient(to_right,#e5e7eb_1px,transparent_1px),linear-gradient(to_bottom,#e5e7eb_1px,transparent_1px)] bg-[size:20px_20px] opacity-60" />
            
            <SignatureCanvas
              // @ts-ignore react-signature-canvas typings issue with ref
              ref={sigCanvas}
              penColor={penColor}
              velocityFilterWeight={0.7}
              minWidth={penWidth * 0.4}
              maxWidth={penWidth * 1.5}
              dotSize={penWidth * 0.6}
              canvasProps={{
                width: canvasSize.width,
                height: canvasSize.height,
                className: "w-full h-full absolute inset-0 z-10 cursor-crosshair",
                style: { touchAction: "none" },
              }}
            />

          </motion.div>
        </div>

      </div>

      <div className="fixed bottom-0 left-0 right-0 px-3 py-3 bg-white/95 dark:bg-black/95 backdrop-blur-xl border-t border-border-color/50 pb-safe z-20 flex items-center justify-between gap-2 shadow-[0_-8px_30px_rgba(0,0,0,0.04)]">
        <button
          onClick={() => navigate(-1)}
          className="w-[60px] h-[48px] rounded-2xl font-bold text-[15px] flex items-center justify-center bg-[#f1f2f4] dark:bg-[#1a1b1c] text-text-main active:scale-95 transition-all outline-none shrink-0"
        >
          {t("notary.signature.cancel")}
        </button>

        <div className="flex-1 h-[48px] bg-[#f8f9fa] dark:bg-[#1a1b1c] rounded-2xl flex items-center justify-evenly px-1 border border-black/5 dark:border-white/5">
            <button
              onClick={() => setShowColorPopup(true)}
              className="flex items-center justify-center gap-1.5 h-full px-3 rounded-xl active:scale-90 transition-all hover:bg-black/5 dark:hover:bg-white/5"
            >
              <div 
                className="w-4 h-4 rounded-full border border-white dark:border-gray-800 shadow-sm ring-1 ring-border-color/50 shrink-0" 
                style={{ backgroundColor: penColor }} 
              />
              <span className="text-[13px] font-medium text-text-main truncate">{t("notary.signature.pen")}</span>
            </button>

            <div className="w-[1px] h-4 bg-border-color/60 shrink-0 mx-1"></div>

            <button
              onClick={() => setShowRatioSheet(true)}
              className="flex items-center justify-center gap-1.5 h-full px-3 rounded-xl active:scale-90 transition-all hover:bg-black/5 dark:hover:bg-white/5 overflow-hidden"
            >
              <Crop className="w-3.5 h-3.5 text-primary-blue shrink-0" strokeWidth={2.5} />
              <span className="text-[13px] font-medium text-text-main truncate">
                 {ratioOptions.find((r) => r.value === ratio)?.label.split(" ")[0] || t("notary.signature.ratio")}
              </span>
            </button>
        </div>

        <button
          onClick={handleConfirm}
          className="w-[96px] h-[48px] rounded-2xl font-bold text-[15px] flex items-center justify-center transition-all bg-primary-blue text-white active:scale-95 shadow-lg shadow-blue-500/30 outline-none shrink-0"
        >
          {t("notary.signature.confirm")}
        </button>
      </div>

      {/* Extreme Polish Color & Stroke Popup (Center Modal Style) */}
      <ColorSettingsModal
        show={showColorPopup}
        onClose={() => setShowColorPopup(false)}
        penColor={penColor}
        setPenColor={setPenColor}
        penWidth={penWidth}
        setPenWidth={setPenWidth}
        colorOptions={colorOptions}
      />

      {/* Extreme Polish Ratio ActionSheet */}
      <RatioSelectionSheet
        show={showRatioSheet}
        onClose={() => setShowRatioSheet(false)}
        ratio={ratio}
        setRatio={setRatio}
        ratioOptions={ratioOptions}
      />
    </div>
  );
};


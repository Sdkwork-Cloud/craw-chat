import React from "react";
import { Cpu, ChevronRight, Settings2, Wand2, Loader2 } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { cn } from "@sdkwork/im-h5-commons";
import { AIVideoOptions } from "../services/AIVideoService";

interface SliderFieldProps {
  label: string;
  min: number;
  max: number;
  step: number;
  value: number;
  onChange: (val: number) => void;
  format?: (val: number) => string;
}

const SliderField = ({ label, min, max, step, value, onChange, format }: SliderFieldProps) => {
  
  return (
  <div className="flex flex-col gap-2 pt-2">
    <div className="flex justify-between items-center text-[13px] text-text-sub">
      <span>{label}</span>
      <span className="font-mono">{format ? format(value) : value}</span>
    </div>
    <input
      type="range"
      min={min}
      max={max}
      step={step}
      value={value}
      onChange={(e) => onChange(Number(e.target.value))}
      className="w-full h-1.5 bg-gray-200 dark:bg-[#3a3b3c] rounded-lg appearance-none cursor-pointer accent-primary-blue"
    />
  </div>
);
};


interface AIVideoSettingsProps {
  t: any;
  prompt: string;
  setPrompt: (v: string) => void;
  style: string;
  setStyle: (v: string) => void;
  aspectRatio: AIVideoOptions["aspectRatio"];
  setAspectRatio: (v: AIVideoOptions["aspectRatio"]) => void;
  styles: string[];
  ratios: AIVideoOptions["aspectRatio"][];
  showAdvanced: boolean;
  setShowAdvanced: (v: boolean) => void;
  cameraMotion: string;
  setCameraMotion: (v: string) => void;
  videoLength: number;
  setVideoLength: (v: number) => void;
  fps: number;
  setFps: (v: number) => void;
  isGenerating: boolean;
  handleGenerate: () => void;
  selectedModelName: string;
  selectedVendorId: string;
  onModelSelectClick: () => void;
}

export const AIVideoSettings: React.FC<AIVideoSettingsProps> = ({
  t,
  prompt,
  setPrompt,
  style,
  setStyle,
  aspectRatio,
  setAspectRatio,
  styles,
  ratios,
  showAdvanced,
  setShowAdvanced,
  cameraMotion,
  setCameraMotion,
  videoLength,
  setVideoLength,
  fps,
  setFps,
  isGenerating,
  handleGenerate,
  selectedModelName,
  selectedVendorId,
  onModelSelectClick,
}) => {
  

return (
    <div className="bg-bg-color p-4 shadow-sm flex flex-col gap-4">
      <div
        onClick={onModelSelectClick}
        className="flex items-center justify-between bg-input-bg border border-border-color rounded-xl p-3 cursor-pointer active:bg-active-bg transition-colors"
      >
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-full bg-primary-blue/10 flex items-center justify-center text-primary-blue">
            <Cpu className="w-4 h-4" />
          </div>
          <div className="flex flex-col">
            <span className="text-[14px] font-medium text-text-main">{t('settings.model_selection')}</span>
            <span className="text-[12px] text-text-sub">{selectedModelName}</span>
          </div>
        </div>
        <ChevronRight className="w-5 h-5 text-text-sub" />
      </div>

      <div>
        <label className="text-sm font-medium text-text-main flex items-center justify-between mb-2">
          <span>{t('settings.prompt')}</span>
        </label>
        <div className="bg-input-bg border border-border-color rounded-xl p-3 focus-within:border-primary-blue transition-colors">
          <textarea
            className="w-full bg-transparent outline-none resize-none text-[15px] text-text-main min-h-[80px] placeholder-text-sub"
            placeholder={t('settings.prompt_placeholder')}
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
          />
        </div>
      </div>

      <div>
        <label className="text-sm font-medium text-text-main block mb-2">
          {t('settings.style')}
        </label>
        <div className="flex overflow-x-auto no-scrollbar gap-2 pb-1 -mx-2 px-2">
          {styles.map((s) => (
            <button
              key={s}
              onClick={() => setStyle(s)}
              className={`px-4 py-1.5 rounded-full text-sm shrink-0 whitespace-nowrap transition-colors ${style === s ? "bg-primary-blue text-white font-medium" : "bg-input-bg text-text-main border border-border-color active:bg-active-bg"}`}
            >
              {s}
            </button>
          ))}
        </div>
      </div>

      <div>
        <label className="text-sm font-medium text-text-main block mb-2">
          {t('settings.format')}
        </label>
        <div className="flex gap-2">
          {ratios.map((r) => (
            <button
              key={r}
              onClick={() => setAspectRatio(r)}
              className={`flex-1 py-1.5 rounded-[10px] text-sm font-medium transition-colors ${aspectRatio === r ? "bg-primary-blue/10 text-primary-blue border border-primary-blue/30" : "bg-input-bg text-text-sub border border-border-color"}`}
            >
              {r}
            </button>
          ))}
        </div>
      </div>

      <div className="pt-2 border-t border-border-color">
        <button
          className="flex items-center justify-between w-full text-sm text-text-main font-medium active:opacity-70 transition-opacity pb-1"
          onClick={() => setShowAdvanced(!showAdvanced)}
        >
          <div className="flex items-center gap-1.5">
            <Settings2 className="w-4 h-4 text-primary-blue" />
            {t('settings.professional_settings')}
          </div>
          <ChevronRight className={cn("w-4 h-4 transition-transform", showAdvanced && "rotate-90")} />
        </button>

        <AnimatePresence>
          {showAdvanced && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: "auto", opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              className="overflow-hidden"
            >
              <div className="pt-3 flex flex-col gap-4 pb-2">
                <div>
                  <label className="text-[13px] text-text-main mb-1.5 block">{t('settings.camera_motion')}</label>
                  <div className="flex gap-2">
                    {['none', 'pan', 'tilt', 'zoom', 'roll'].map(m => (
                      <button
                        key={m}
                        onClick={() => setCameraMotion(m)}
                        className={cn(
                          "flex-1 py-1 rounded text-[12px] border transition-colors capitalize",
                          cameraMotion === m ? "bg-primary-blue/10 text-primary-blue border-primary-blue/30 font-medium" : "bg-input-bg text-text-sub border-border-color"
                        )}
                      >
                        {m}
                      </button>
                    ))}
                  </div>
                </div>
                {(selectedVendorId === "runway" || selectedVendorId === "kling") && (
                  <SliderField label={t('settings.duration')} min={5} max={10} step={5} value={videoLength} onChange={setVideoLength} format={(v: any) => v + "s"} />
                )}
                <SliderField label={t('settings.fps')} min={24} max={60} step={6} value={fps} onChange={setFps} format={(v: any) => v + " fps"} />
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      <button
        disabled={isGenerating || !prompt.trim()}
        onClick={handleGenerate}
        className="w-full h-[46px] rounded-xl bg-primary-blue text-white font-bold flex items-center justify-center gap-2 disabled:opacity-50 active:scale-[0.98] transition-all mt-2 shadow-sm"
      >
        {isGenerating ? (
          <Loader2 className="w-5 h-5 animate-spin" />
        ) : (
          <Wand2 className="w-5 h-5" />
        )}
        {isGenerating ? t('settings.generating') : t('settings.generate_video')}
      </button>
    </div>
  );
};

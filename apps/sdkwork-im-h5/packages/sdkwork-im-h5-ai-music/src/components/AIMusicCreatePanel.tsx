import { useTranslation } from "react-i18next";
import React from "react";
import { Cpu, ChevronRight, Settings2, Sparkles, Wand2, TextSelect, Disc3, Loader2, Music } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { cn } from "@sdkwork/im-h5-commons";

interface AIMusicCreatePanelProps {
  t: any;
  showModelSelection: boolean;
  setShowModelSelection: (b: boolean) => void;
  selectedModelName: string;
  isInstrumental: boolean;
  setIsInstrumental: (b: boolean) => void;
  prompt: string;
  setPrompt: (s: string) => void;
  lyrics: string;
  setLyrics: (s: string) => void;
  style: string;
  setStyle: (s: string) => void;
  styles: string[];
  showAdvanced: boolean;
  setShowAdvanced: (b: boolean) => void;
  vocalType: string;
  setVocalType: (s: string) => void;
  tempo: string;
  setTempo: (s: string) => void;
  isGenerating: boolean;
  handleGenerate: () => void;
}

export const AIMusicCreatePanel: React.FC<AIMusicCreatePanelProps> = ({
  t,
  showModelSelection,
  setShowModelSelection,
  selectedModelName,
  isInstrumental,
  setIsInstrumental,
  prompt,
  setPrompt,
  lyrics,
  setLyrics,
  style,
  setStyle,
  styles,
  showAdvanced,
  setShowAdvanced,
  vocalType,
  setVocalType,
  tempo,
  setTempo,
  isGenerating,
  handleGenerate,
}) => {
  

return (
    <div className="p-4 flex flex-col gap-5">
      <div
        onClick={() => setShowModelSelection(true)}
        className="flex items-center justify-between bg-[#222] border border-white/10 rounded-xl p-3 cursor-pointer active:scale-[0.98] transition-all shadow-sm"
      >
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-full bg-blue-500/10 flex items-center justify-center text-blue-400">
            <Cpu className="w-4 h-4" />
          </div>
          <div className="flex flex-col">
            <span className="text-[14px] font-medium text-white/90">{t('create.model_selection')}</span>
            <span className="text-[12px] text-white/50">{selectedModelName}</span>
          </div>
        </div>
        <ChevronRight className="w-5 h-5 text-white/40" />
      </div>

      {/* Prompt Input */}
      <div>
        <div className="flex justify-between items-center mb-3">
          <h2 className="text-[16px] font-medium flex items-center gap-2">
            <Sparkles className="w-4 h-4 text-purple-400" />
            {t('create.prompt_title')}
          </h2>
          <div className="flex items-center gap-2 bg-[#222] px-2.5 py-1 rounded-full border border-white/10">
            <span className="text-[12px] text-white/70">{t('create.instrumental')}</span>
            <button
              onClick={() => setIsInstrumental(!isInstrumental)}
              className={cn("w-8 h-4 rounded-full relative transition-colors", isInstrumental ? "bg-purple-500" : "bg-white/20")}
            >
              <div className={cn("w-3 h-3 bg-white rounded-full absolute top-[2px] transition-all", isInstrumental ? "left-[18px]" : "left-[2px]")} />
            </button>
          </div>
        </div>
        <div className="bg-[#222] border border-white/10 rounded-2xl p-4 focus-within:border-purple-500/50 transition-colors shadow-inner relative">
          <textarea
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder={t('create.prompt_placeholder')}
            className="w-full h-20 bg-transparent outline-none text-[15px] resize-none placeholder:text-white/30"
          />
          <button
            className="absolute bottom-3 right-3 p-2 bg-[#333] hover:bg-[#444] rounded-full active:scale-95 transition-all text-white/80"
            onClick={() => setPrompt("一首深沉的Lo-Fi音乐，带有夜晚都市的霓虹氛围，适合安静地学习")}
          >
            <Wand2 className="w-4 h-4" />
          </button>
        </div>
      </div>
      
      {/* Lyrics Section */}
      <AnimatePresence>
        {!isInstrumental && (
          <motion.div initial={{ opacity: 0, height: 0 }} animate={{ opacity: 1, height: 'auto' }} exit={{ opacity: 0, height: 0 }}>
            <div className="flex justify-between items-center mb-3 mt-1">
                <h2 className="text-[16px] font-medium flex items-center gap-2">
                  <TextSelect className="w-4 h-4 text-pink-400" />
                  {t('create.lyrics_title')}
                </h2>
                <span className="text-[11px] text-white/40">{t('create.optional')}</span>
            </div>
            <div className="bg-[#222] border border-white/10 rounded-2xl p-4 focus-within:border-pink-500/50 transition-colors shadow-inner">
                <textarea
                  value={lyrics}
                  onChange={(e) => setLyrics(e.target.value)}
                  placeholder={t('create.lyrics_placeholder')}
                  className="w-full h-24 bg-transparent outline-none text-[15px] resize-none placeholder:text-white/30"
                />
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Quick Styles */}
      <div className="mt-1">
        <div className="flex justify-between items-center mb-3">
          <h2 className="text-[16px] font-medium flex items-center gap-2">
            <Disc3 className="w-4 h-4 text-blue-400" />
            {t('create.style_title')}
          </h2>
        </div>
        <div className="flex flex-wrap gap-2">
          {styles.map((s) => (
            <button
              key={s}
              onClick={() => setStyle(s)}
              className={cn(
                "px-4 py-2 rounded-xl text-[13px] transition-all border",
                style === s
                  ? "bg-white text-black border-white font-medium"
                  : "bg-[#222] text-white/70 border-white/10 hover:border-white/30"
              )}
            >
              {t(`styles.${s}`, { defaultValue: s })}
            </button>
          ))}
        </div>
      </div>

      <div className="border border-white/5 rounded-xl mt-2 overflow-hidden bg-[#222]/50">
        <button
          className="flex items-center justify-between w-full p-4 active:bg-white/5 transition-colors"
          onClick={() => setShowAdvanced(!showAdvanced)}
        >
          <div className="flex items-center gap-2 text-[14px]">
            <Settings2 className="w-4 h-4 text-white/70" />
            {t('create.professional_settings')}
          </div>
          <ChevronRight className={cn("w-4 h-4 text-white/40 transition-transform", showAdvanced && "rotate-90")} />
        </button>
        <AnimatePresence>
          {showAdvanced && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: "auto", opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
            >
              <div className="p-4 pt-0 flex flex-col gap-4 border-t border-white/5 mt-2">
                <div>
                  <label className="text-[12px] text-white/50 mb-2 block">{t('create.vocal_style')}</label>
                  <div className="flex flex-wrap gap-2">
                    {['auto', 'male', 'female', 'robot'].map((val: string) => {
                      return (
                        <button
                          key={val}
                          onClick={() => setVocalType(val)}
                          className={cn(
                            "px-3 py-1.5 rounded-[8px] text-[12px] border transition-colors",
                            vocalType === val ? "bg-white/10 border-white/30 text-white" : "border-white/5 text-white/50 bg-[#111]"
                          )}
                        >
                          {t(`vocal_styles.${val}`, { defaultValue: val })}
                        </button>
                      );
                    })}
                  </div>
                </div>
                <div>
                  <label className="text-[12px] text-white/50 mb-2 block">{t('create.tempo')}</label>
                  <div className="flex gap-2">
                    {['auto', 'slow', 'medium', 'fast'].map(tpm => (
                      <button
                        key={tpm}
                        onClick={() => setTempo(tpm)}
                        className={cn(
                          "flex-1 py-1.5 rounded-[8px] text-[12px] border transition-colors capitalize",
                          tempo === tpm ? "bg-white/10 border-white/30 text-white" : "border-white/5 text-white/50 bg-[#111]"
                        )}
                      >
                        {tpm}
                      </button>
                    ))}
                  </div>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* Action Bar */}
      <div className="pt-2">
        <button
          disabled={isGenerating}
          onClick={handleGenerate}
          className="w-full py-4 bg-white text-black rounded-full font-bold text-[16px] flex flex-col items-center justify-center active:scale-[0.98] transition-transform disabled:opacity-50"
        >
          {isGenerating ? (
            <span className="flex items-center gap-2">
              <Loader2 className="w-5 h-5 animate-spin" /> {t('create.generating')}
            </span>
          ) : (
            <span className="flex items-center gap-2">
              <Music className="w-5 h-5" /> {t('create.generate_button')}
            </span>
          )}
        </button>
      </div>
    </div>
  );
};

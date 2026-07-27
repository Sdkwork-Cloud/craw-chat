import React from "react";
import { MessageSquareMore, Languages, Edit3, Loader2 } from "lucide-react";
import { AIWritingOptions } from "../services/AIWritingService";

interface AIWritingSettingsPanelProps {
  t: any;
  topic: string;
  setTopic: (s: string) => void;
  style: string;
  setStyle: (s: string) => void;
  length: AIWritingOptions["length"];
  setLength: (s: AIWritingOptions["length"]) => void;
  language: AIWritingOptions["language"];
  setLanguage: (s: AIWritingOptions["language"]) => void;
  styles: string[];
  lengths: AIWritingOptions["length"][];
  languages: AIWritingOptions["language"][];
  isGenerating: boolean;
  handleGenerate: () => void;
}

export const AIWritingSettingsPanel: React.FC<AIWritingSettingsPanelProps> = ({
  t,
  topic,
  setTopic,
  style,
  setStyle,
  length,
  setLength,
  language,
  setLanguage,
  styles,
  lengths,
  languages,
  isGenerating,
  handleGenerate,
}) => {
  

return (
    <div className="bg-bg-color p-4 shadow-sm">
      <div className="flex flex-col gap-5">
        <div>
          <label className="text-sm font-medium text-text-main flex items-center gap-1.5 mb-2">
            <MessageSquareMore className="w-4 h-4 text-primary-blue" />
            {t('settings.topic_title')}
          </label>
          <div className="bg-input-bg border border-border-color rounded-xl p-3 focus-within:border-primary-blue transition-all shadow-sm">
            <textarea
              className="w-full bg-transparent outline-none resize-none text-[15px] text-text-main min-h-[80px] placeholder-text-sub"
              placeholder={t('settings.topic_placeholder')}
              value={topic}
              onChange={(e) => setTopic(e.target.value)}
            />
          </div>
        </div>

        <div className="flex gap-4">
          <div className="flex-1">
            <label className="text-sm font-medium text-text-main block mb-2">
              {t('settings.length')}
            </label>
            <div className="flex bg-input-bg rounded-lg p-1 border border-border-color">
              {lengths.map((l) => (
                <button
                  key={l}
                  onClick={() => setLength(l)}
                  className={`flex-1 py-1.5 rounded-md text-[13px] font-medium capitalize transition-colors ${length === l ? "bg-bg-color shadow-sm text-text-main" : "text-text-sub"}`}
                >
                  {t(`lengths.${l}`, { defaultValue: l })}
                </button>
              ))}
            </div>
          </div>
          <div className="flex-1">
            <label className="text-sm font-medium text-text-main flex items-center gap-1 mb-2">
              <Languages className="w-4 h-4" /> {t('settings.language')}
            </label>
            <div className="flex bg-input-bg rounded-lg p-1 border border-border-color">
              {languages.map((l) => (
                <button
                  key={l}
                  onClick={() => setLanguage(l)}
                  className={`flex-1 py-1.5 rounded-md text-[13px] font-medium transition-colors ${language === l ? "bg-bg-color shadow-sm text-text-main" : "text-text-sub"}`}
                >
                  {t(`languages.${l}`, { defaultValue: l })}
                </button>
              ))}
            </div>
          </div>
        </div>

        <div>
          <label className="text-sm font-medium text-text-main block mb-2">
            {t('settings.tone_style')}
          </label>
          <div className="flex flex-wrap gap-2">
            {styles.map((s) => (
              <button
                key={s}
                onClick={() => setStyle(s)}
                className={`px-3 py-1.5 rounded-xl text-[13px] font-medium transition-colors ${style === s ? "bg-primary-blue text-white shadow-md shadow-primary-blue/20" : "bg-input-bg text-text-main border border-border-color hover:bg-active-bg"}`}
              >
                {t(`styles.${s}`, { defaultValue: s })}
              </button>
            ))}
          </div>
        </div>

        <button
          disabled={isGenerating || !topic.trim()}
          onClick={handleGenerate}
          className="w-full h-[48px] rounded-xl bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-bold flex items-center justify-center gap-2 disabled:opacity-50 active:scale-[0.98] transition-all mt-1 shadow-md"
        >
          {isGenerating ? (
            <Loader2 className="w-5 h-5 animate-spin" />
          ) : (
            <Edit3 className="w-5 h-5" />
          )}
          {isGenerating ? t('settings.generating') : t('settings.generate_button')}
        </button>
      </div>
    </div>
  );
};

import React from "react";
import { cn } from "@sdkwork/im-h5-commons";
import { Settings2, ChevronRight, Globe, Image as ImageIcon } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { useTranslation } from "react-i18next";

interface AdvancedSettingsPanelProps {
  showAdvanced: boolean;
  setShowAdvanced: (show: boolean) => void;
  tools: { webSearch: boolean; imageGen: boolean };
  setTools: (tools: { webSearch: boolean; imageGen: boolean }) => void;
  temperature: number;
  setTemperature: (temp: number) => void;
  maxTokens: number;
  setMaxTokens: (tokens: number) => void;
}

export const AdvancedSettingsPanel: React.FC<AdvancedSettingsPanelProps> = ({
  showAdvanced,
  setShowAdvanced,
  tools,
  setTools,
  temperature,
  setTemperature,
  maxTokens,
  setMaxTokens,
}) => {
  const { t } = useTranslation();
const Switch = ({
    checked,
    onChange,
  }: {
    checked: boolean;
    onChange: (c: boolean) => void;
  }) => (
    <div
      onClick={(e) => {
        e.stopPropagation();
        onChange(!checked);
      }}
      className={cn(
        "w-12 h-6 rounded-full transition-colors relative cursor-pointer shrink-0",
        checked ? "bg-primary-blue" : "bg-gray-300 dark:bg-gray-600",
      )}
    >
      <div
        className={cn(
          "absolute top-1 w-4 h-4 rounded-full bg-white transition-transform shadow-sm",
          checked ? "left-7" : "left-1",
        )}
      />
    </div>
  );

  return (
    <div className="mt-2 flex flex-col">
      <div
        onClick={() => setShowAdvanced(!showAdvanced)}
        className={cn(
          "flex items-center justify-between px-4 py-3.5 bg-chat-other-bg border border-border-color active:bg-active-bg transition-all cursor-pointer",
          showAdvanced
            ? "rounded-t-xl border-b-transparent"
            : "rounded-xl",
        )}
      >
        <div className="flex items-center gap-3">
          <Settings2 className="w-5 h-5 text-text-main" />
          <span className="text-[16px] text-text-main">{t('contacts.advanced_settings', 'Advanced Settings')}</span>
        </div>
        <div className="flex items-center gap-2 text-text-sub">
          <span className="text-[14px]">{t('contacts.advanced_settings_desc', 'Configure specific parameters')}</span>
          <ChevronRight
            className={cn(
              "w-5 h-5 opacity-50 transition-transform duration-300",
              showAdvanced && "rotate-90",
            )}
          />
        </div>
      </div>

      <AnimatePresence>
        {showAdvanced && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: "auto", opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.2, ease: "easeInOut" }}
            className="overflow-hidden bg-chat-other-bg border border-t-0 border-border-color rounded-b-xl"
          >
            <div className="flex flex-col gap-6 p-4 pt-2">
              {/* Tools / Plugins */}
              <div className="flex flex-col gap-3">
                <label className="text-[13px] font-medium text-text-sub">
                  {t('contacts.extend_capabilities', 'Extend Capabilities')}
                </label>

                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Globe className="w-4 h-4 text-text-main" />
                    <span className="text-[14px] text-text-main">
                      {t('contacts.tool_web_search', 'Web Search')}
                    </span>
                  </div>
                  <Switch
                    checked={tools.webSearch}
                    onChange={(c) => setTools({ ...tools, webSearch: c })}
                  />
                </div>

                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <ImageIcon className="w-4 h-4 text-text-main" />
                    <span className="text-[14px] text-text-main">
                      {t('contacts.tool_image_gen', 'Image Generation')}
                    </span>
                  </div>
                  <Switch
                    checked={tools.imageGen}
                    onChange={(c) => setTools({ ...tools, imageGen: c })}
                  />
                </div>
              </div>

              {/* Temperature Slider */}
              <div className="flex flex-col gap-2.5">
                <div className="flex justify-between items-center">
                  <label className="text-[13px] font-medium text-text-sub">
                    {t('contacts.model_temp', 'Temperature')}
                  </label>
                  <span className="text-[14px] text-primary-blue font-medium">
                    {temperature.toFixed(1)}
                  </span>
                </div>
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.1"
                  value={temperature}
                  onChange={(e) =>
                    setTemperature(parseFloat(e.target.value))
                  }
                  className="w-full accent-primary-blue"
                />
                <div className="flex justify-between text-[11px] text-text-sub/70">
                  <span>{t('contacts.temp_precise', 'Precise')}</span>
                  <span>{t('contacts.temp_creative', 'Creative')}</span>
                </div>
              </div>

              {/* Max Tokens */}
              <div className="flex flex-col gap-2.5">
                <div className="flex justify-between items-center">
                  <label className="text-[13px] font-medium text-text-sub">
                    {t('contacts.model_max_tokens', 'Max Tokens')}
                  </label>
                  <span className="text-[14px] text-primary-blue font-medium">
                    {maxTokens}
                  </span>
                </div>
                <input
                  type="range"
                  min="256"
                  max="8192"
                  step="256"
                  value={maxTokens}
                  onChange={(e) => setMaxTokens(parseInt(e.target.value))}
                  className="w-full accent-primary-blue"
                />
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

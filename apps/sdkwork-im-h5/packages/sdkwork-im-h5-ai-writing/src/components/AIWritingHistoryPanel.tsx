import { useTranslation } from "react-i18next";
import React from "react";
import { MessageSquareMore, Loader2, RefreshCw, Copy, Check, Trash2 } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { WritingTask, AIWritingOptions } from "../services/AIWritingService";
import { AIWritingFormattedText } from "./AIWritingFormattedText";
import { AIWritingHistoryItem } from "./AIWritingHistoryItem";

interface AIWritingHistoryPanelProps {
  t: any;
  currentTask: WritingTask | null;
  history: WritingTask[];
  isGenerating: boolean;
  realtimeContent: string;
  copied: boolean;
  handleGenerate: () => void;
  handleCopy: (content?: string) => void;
  handleDelete: (e: React.MouseEvent, id: string) => void;
  setTopic: (s: string) => void;
  setStyle: (s: string) => void;
  setLength: (s: AIWritingOptions["length"]) => void;
  setLanguage: (s: AIWritingOptions["language"]) => void;
  setCurrentTask: (t: WritingTask | null) => void;
  setRealtimeContent: (s: string) => void;
}

export const AIWritingHistoryPanel: React.FC<AIWritingHistoryPanelProps> = ({
  t,
  currentTask,
  history,
  isGenerating,
  realtimeContent,
  copied,
  handleGenerate,
  handleCopy,
  handleDelete,
  setTopic,
  setStyle,
  setLength,
  setLanguage,
  setCurrentTask,
  setRealtimeContent,
}) => {
  

return (
    <div className="px-4 pb-6">
      <AnimatePresence>
        {currentTask && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-bg-color rounded-2xl border border-border-color shadow-sm relative pt-12 overflow-hidden flex flex-col mb-6"
          >
            <div className="absolute top-0 left-0 right-0 h-10 border-b border-border-color flex items-center justify-between px-4 bg-active-bg">
              <div className="flex space-x-1.5">
                <div className="w-2.5 h-2.5 rounded-full bg-red-400"></div>
                <div className="w-2.5 h-2.5 rounded-full bg-amber-400"></div>
                <div className="w-2.5 h-2.5 rounded-full bg-green-400"></div>
              </div>
              <div className="flex items-center gap-3">
                <span className="text-[11px] font-medium text-text-sub uppercase tracking-wider">
                  {t(`styles.${currentTask.options.style}`, { defaultValue: currentTask.options.style })}
                </span>
                {currentTask.status === "completed" && (
                  <>
                    <button
                      onClick={handleGenerate}
                      className="text-text-sub hover:text-primary-blue transition-colors active:scale-95 bg-bg-color p-1.5 rounded-md border border-border-color shadow-sm"
                      title={t('result.regenerate')}
                    >
                      <RefreshCw className="w-3.5 h-3.5" />
                    </button>
                    <button
                      onClick={() => handleCopy(currentTask.content)}
                      className="text-text-sub hover:text-text-main transition-colors active:scale-95 bg-bg-color p-1.5 rounded-md border border-border-color shadow-sm"
                      title={t('result.copy')}
                    >
                      {copied ? (
                        <Check className="w-3.5 h-3.5 text-green-500" />
                      ) : (
                        <Copy className="w-3.5 h-3.5" />
                      )}
                    </button>
                  </>
                )}
              </div>
            </div>

            <div className="p-4 min-h-[120px]">
              {currentTask.status === "generating" && !realtimeContent ? (
                <div className="flex flex-col items-center justify-center py-6 text-text-sub h-full">
                  <Loader2 className="w-8 h-8 animate-spin mb-3 text-primary-blue" />
                  <span className="text-[13px]">
                    {t('result.analyzing')}
                  </span>
                </div>
              ) : (
                <div className="relative">
                  <AIWritingFormattedText content={realtimeContent || currentTask.content || ""} />
                  {currentTask.status === "generating" && (
                    <span className="inline-block w-2 bg-primary-blue h-[15px] animate-pulse ml-1 align-middle" />
                  )}
                </div>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {!isGenerating && (
        <div id="history-section" className="flex flex-col gap-3">
          {history.length > 0 ? (
            <>
              <h3 className="text-[16px] font-bold text-text-main">
                {t('history.title')}
              </h3>
              <div className="flex flex-col gap-3">
                {history.map((item) => (
                  <AIWritingHistoryItem
                    key={item.id}
                    item={item}
                    t={t}
                    onSelect={(selected) => {
                      setTopic(selected.options.topic);
                      setStyle(selected.options.style);
                      setLength(selected.options.length);
                      setLanguage(selected.options.language);
                      setCurrentTask(selected);
                      setRealtimeContent("");
                    }}
                    onDelete={handleDelete}
                  />
                ))}
              </div>
            </>
          ) : !currentTask ? (
            <div className="pt-6 flex flex-col items-center justify-center opacity-70">
              <MessageSquareMore className="w-12 h-12 text-text-sub mb-3 opacity-50" />
              <h3 className="text-sm font-medium text-text-sub mb-4">
                {t('history.empty')}
              </h3>
              <div className="flex flex-wrap gap-2 justify-center px-4">
                {[
                  "A creative story about Mars",
                  "Professional email to a client",
                  "Casual blog about coffee",
                  "The impact of AI on design",
                  "How to learn React fast",
                ].map((suggestion, i) => (
                  <button
                    key={i}
                    onClick={() => setTopic(suggestion)}
                    className="bg-active-bg border border-border-color px-3 py-1.5 rounded-full text-xs text-text-main hover:border-primary-blue transition-colors active:scale-95"
                  >
                    {suggestion}
                  </button>
                ))}
              </div>
            </div>
          ) : null}
        </div>
      )}
    </div>
  );
};

import { useTranslation } from "react-i18next";
import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { type KnowledgeBase } from "@sdkwork/im-h5-knowledge";

interface KnowledgeBaseSelectionModalProps {
  show: boolean;
  onClose: () => void;
  knowledgeBases: KnowledgeBase[];
  selectedKb: KnowledgeBase | null;
  onSelect: (kb: KnowledgeBase | null) => void;
}

export const KnowledgeBaseSelectionModal: React.FC<KnowledgeBaseSelectionModalProps> = ({
  show,
  onClose,
  knowledgeBases,
  selectedKb,
  onSelect,
}) => {
  const { t } = useTranslation();
return (
    <AnimatePresence>
      {show && (
        <motion.div
          initial={{ y: "100%" }}
          animate={{ y: 0 }}
          exit={{ y: "100%" }}
          transition={{ type: "spring", damping: 25, stiffness: 200 }}
          className="fixed inset-0 z-[100] bg-[#f8f9fc] dark:bg-[#121214] flex flex-col"
        >
          <div className="flex-none pt-safe bg-white dark:bg-[#1e1e20] border-b border-border-color/50">
            <div className="h-14 px-4 flex items-center justify-between">
              <div className="w-14"></div>
              <h3 className="text-[17px] font-semibold text-text-main">Select Knowledge Base</h3>
              <button 
                onClick={onClose}
                className="w-14 text-right text-[15px] text-text-sub font-medium"
              >
                Cancel
              </button>
            </div>
          </div>
          
          <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-3 pb-safe">
            {knowledgeBases.length > 0 ? (
              knowledgeBases.map((kb) => (
                <div
                  key={kb.id}
                  onClick={() => {
                    onSelect(kb);
                    onClose();
                  }}
                  className="bg-white dark:bg-[#1e1e20] p-4 rounded-xl shadow-sm border border-border-color/50 flex items-center gap-4 cursor-pointer active:scale-[0.98] transition-all"
                >
                  <div 
                    className="w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-inner shrink-0"
                    style={{ 
                      backgroundColor: kb.color ? `${kb.color}1A` : 'rgba(0, 102, 255, 0.1)', 
                      color: kb.color || '#0066FF'
                    }}
                  >
                    {kb.icon || "📚"}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-[16px] font-semibold text-text-main truncate mb-0.5">{kb.name}</div>
                    <div className="text-[13px] text-text-sub truncate">{kb.description || 'No description'}</div>
                  </div>
                  {selectedKb?.id === kb.id && (
                    <div className="w-6 h-6 rounded-full bg-primary-blue flex items-center justify-center text-white shrink-0">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
                    </div>
                  )}
                </div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-10 text-text-sub opacity-70">
                <span className="text-[15px]">No knowledge bases found</span>
                <span className="text-[13px] mt-1">Please create one first</span>
              </div>
            )}

            {selectedKb && (
              <button
                onClick={() => {
                  onSelect(null);
                  onClose();
                }}
                className="mt-4 py-3.5 bg-red-50 dark:bg-red-500/10 text-red-500 font-medium rounded-xl text-[15px] active:scale-[0.98] transition-transform"
              >
                Remove Knowledge Base
              </button>
            )}
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};

import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';

export const PromptModal = ({ 
  isOpen, 
  title, 
  defaultValue = '', 
  onConfirm, 
  onCancel 
}: { 
  isOpen: boolean; 
  title: string; 
  defaultValue?: string; 
  onConfirm: (val: string) => void; 
  onCancel: () => void;
}) => {
  const [val, setVal] = useState(defaultValue);
  
  useEffect(() => {
    if (isOpen) setVal(defaultValue);
  }, [isOpen, defaultValue]);

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div 
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-50 flex items-center justify-center"
        >
          <div className="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={onCancel} />
          <motion.div 
            initial={{ scale: 0.95, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.95, opacity: 0 }}
            transition={{ type: "spring", damping: 25, stiffness: 300 }}
            className="relative bg-[#282828] border border-white/10 rounded-2xl w-full max-w-sm shadow-xl p-6"
          >
             <h3 className="text-lg font-medium text-white mb-4">{title}</h3>
             <input
               type="text"
               className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 mb-6"
               value={val}
               onChange={(e) => setVal(e.target.value)}
               autoFocus
               onKeyDown={e => {
                 if (e.key === 'Enter' && val.trim()) onConfirm(val);
                 if (e.key === 'Escape') onCancel();
               }}
             />
             <div className="flex justify-end gap-3">
                <button className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors" onClick={onCancel}>取消</button>
                <button className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium disabled:opacity-50" disabled={!val.trim()} onClick={() => onConfirm(val)}>确认</button>
             </div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};

export const usePrompt = () => {
  const [promptConfig, setPromptConfig] = useState<{
    isOpen: boolean;
    title: string;
    defaultValue?: string;
    onConfirm: (val: string) => void;
  }>({ isOpen: false, title: '', onConfirm: () => {} });

  const customPrompt = (title: string, defaultValue: string = '', onConfirm: (val: string) => void) => {
    setPromptConfig({ isOpen: true, title, defaultValue, onConfirm });
  };

  const closePrompt = () => setPromptConfig(prev => ({ ...prev, isOpen: false }));

  return { promptConfig, customPrompt, closePrompt };
};

import React, { useState } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { KnowledgeBaseService } from "../services/KnowledgeBaseService";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";

export const CreateKnowledgeBase = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [icon, setIcon] = useState("📚");
  const [color, setColor] = useState("#0066FF");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const EMOJI_LIST = ["📚", "📁", "🗂️", "💼", "🧠", "💡", "📑", "📊", "📋", "💻"];
  const COLOR_LIST = ["#0066FF", "#EF4444", "#22C55E", "#A855F7", "#F97316", "#EC4899"];

  const handleSubmit = async () => {
    if (!name.trim()) return;
    
    setIsSubmitting(true);
    await KnowledgeBaseService.createKnowledgeBase({
      name,
      description,
      icon,
      color,
    });
    setIsSubmitting(false);
    navigate(-1);
  };

  return (
    <PageLayout 
      title={t('knowledge.create_kb', 'New Knowledge Base')}
      rightElement={
        <button
          onClick={handleSubmit}
          disabled={!name.trim() || isSubmitting}
          className="text-primary-blue font-semibold disabled:opacity-50"
        >
          {t('common.create', 'Create')}
        </button>
      }
    >
      <div className="flex flex-col h-full bg-[#f8f9fc] dark:bg-[#121214] p-4">
        <div className="bg-white dark:bg-[#1e1e20] p-6 rounded-2xl shadow-sm border border-border-color/50 flex flex-col gap-6">
          
          <div className="flex flex-col gap-3 items-center pt-2 pb-4">
            <div 
              className="w-20 h-20 rounded-2xl flex items-center justify-center text-4xl shadow-inner mb-3 transition-colors"
              style={{ backgroundColor: `${color}1A`, color: color }}
            >
              {icon}
            </div>
            
            {/* Emoji Picker */}
            <div className="flex flex-wrap justify-center gap-2 bg-[#f8f9fc] dark:bg-[#2c2d2e] p-2 rounded-xl">
              {EMOJI_LIST.map((emoji) => (
                <button
                  key={emoji}
                  onClick={() => setIcon(emoji)}
                  className={`w-8 h-8 flex items-center justify-center rounded-lg text-lg transition-colors ${icon === emoji ? 'bg-white dark:bg-[#1e1e20] shadow-sm scale-110' : 'hover:bg-black/5 dark:hover:bg-white/5 opacity-70 hover:opacity-100'}`}
                >
                  {emoji}
                </button>
              ))}
            </div>

            {/* Color Picker */}
            <div className="flex flex-wrap justify-center gap-3 mt-1">
              {COLOR_LIST.map((c) => (
                <button
                  key={c}
                  onClick={() => setColor(c)}
                  className={`w-6 h-6 rounded-full flex items-center justify-center transition-transform ${color === c ? 'scale-125 shadow-sm ring-2 ring-offset-2 ring-offset-white dark:ring-offset-[#1e1e20]' : 'hover:scale-110'}`}
                  style={{ backgroundColor: c, '--tw-ring-color': c } as React.CSSProperties}
                >
                  {color === c && (
                    <div className="w-2 h-2 rounded-full bg-white opacity-80" />
                  )}
                </button>
              ))}
            </div>
          </div>

          <div>
            <label className="text-[14px] font-medium text-text-main mb-2 block">{t('knowledge.kb_name', 'Knowledge Base Name')}</label>
            <input
              type="text"
              placeholder={t('knowledge.name_placeholder', 'e.g. Employee Handbook')}
              className="w-full text-[16px] text-text-main bg-[#f8f9fc] dark:bg-[#2c2d2e] border-none outline-none rounded-xl px-4 py-3 placeholder:text-text-sub/50 transition-colors focus:ring-2 focus:ring-primary-blue/20"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          
          <div>
            <label className="text-[14px] font-medium text-text-main mb-2 block">{t('knowledge.kb_desc', 'Description')}</label>
            <textarea
              placeholder={t('knowledge.desc_placeholder', 'Briefly describe what goes in here...')}
              className="w-full text-[15px] text-text-main bg-[#f8f9fc] dark:bg-[#2c2d2e] border-none outline-none rounded-xl px-4 py-3 min-h-[120px] resize-none placeholder:text-text-sub/50 transition-colors focus:ring-2 focus:ring-primary-blue/20 leading-relaxed"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </div>
        </div>
      </div>
    </PageLayout>
  );
};

import React, { useState } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { KnowledgeBaseService } from "../services/KnowledgeBaseService";
import { useNavigate, useParams } from "react-router";
import { useTranslation } from "react-i18next";

export const CreateDocument = () => {
  const { t } = useTranslation();
  
const { id } = useParams();
  const navigate = useNavigate();
  
  
  const [title, setTitle] = useState("");
  const [category, setCategory] = useState("General");
  const [content, setContent] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async () => {
    if (!id || !title.trim() || !content.trim()) return;
    
    setIsSubmitting(true);
    await KnowledgeBaseService.createDocument({
      kbId: id,
      title,
      category,
      content,
      author: "Current User",
    });
    setIsSubmitting(false);
    navigate(-1);
  };

  return (
    <PageLayout 
      title={t('knowledge.create_doc', 'New Document')}
      rightElement={
        <button
          onClick={handleSubmit}
          disabled={!title.trim() || !content.trim() || isSubmitting}
          className="text-primary-blue font-semibold disabled:opacity-50"
        >
          {t('common.publish', 'Publish')}
        </button>
      }
    >
      <div className="flex flex-col h-full bg-[#f8f9fc] dark:bg-[#121214]">
        <div className="bg-white dark:bg-[#1e1e20] p-4 flex flex-col gap-4 shadow-sm border-b border-border-color/50">
          <input
            type="text"
            placeholder={t('knowledge.doc_title', 'Document Title')}
            className="w-full text-[18px] font-semibold text-text-main bg-transparent border-none outline-none placeholder:text-text-sub/50"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
          />
          <div className="h-[1px] bg-border-color/50" />
          <div className="flex items-center gap-2 overflow-x-auto no-scrollbar pb-1">
            {['General', 'Draft', 'Policy', 'Manual', 'Technical'].map((cat) => (
              <button
                key={cat}
                onClick={() => setCategory(cat)}
                className={`whitespace-nowrap px-4 py-1.5 rounded-full text-[13px] font-medium transition-colors ${
                  category === cat
                    ? 'bg-primary-blue text-white'
                    : 'bg-[#f3f4f6] dark:bg-[#2c2d2e] text-text-sub hover:bg-gray-200 dark:hover:bg-gray-700'
                }`}
              >
                {cat}
              </button>
            ))}
          </div>
        </div>

        <div className="mt-2 flex-1 bg-white dark:bg-[#1e1e20] p-4 shadow-sm">
          <textarea
            placeholder={t('knowledge.content_placeholder', 'Start typing your document...')}
            className="w-full h-full text-[16px] text-text-main bg-transparent border-none outline-none resize-none placeholder:text-text-sub/40 leading-relaxed"
            value={content}
            onChange={(e) => setContent(e.target.value)}
          />
        </div>
      </div>
    </PageLayout>
  );
};

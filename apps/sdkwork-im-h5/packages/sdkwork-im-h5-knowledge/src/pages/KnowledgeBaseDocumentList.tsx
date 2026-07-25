import React, { useState, useEffect } from "react";
import { PageLayout } from "@sdkwork/im-h5-commons";
import { Plus, FileText, Search, ChevronRight, Settings } from "lucide-react";
import { KnowledgeBaseService, KnowledgeDocument, KnowledgeBase } from "../services/KnowledgeBaseService";
import { motion } from "motion/react";
import { useNavigate, useParams } from "react-router";
import { useTranslation } from "react-i18next";

export const KnowledgeBaseDocumentList = () => {
  const { t } = useTranslation();
  
const { id } = useParams();
  const navigate = useNavigate();
  
  
  const [kb, setKb] = useState<KnowledgeBase | null>(null);
  const [documents, setDocuments] = useState<KnowledgeDocument[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    setIsLoading(true);
    
    Promise.all([
      KnowledgeBaseService.getKnowledgeBase(id),
      KnowledgeBaseService.getDocumentsByKbId(id)
    ]).then(([kbData, docsData]) => {
      setKb(kbData);
      setDocuments(docsData);
      setIsLoading(false);
    });
  }, [id]);

  const handleDeleteKb = async () => {
    if (!id || !kb) return;
    if (window.confirm(t('knowledge.delete_kb_confirm', 'Are you sure you want to delete this entire knowledge base and all its documents?'))) {
      await KnowledgeBaseService.deleteKnowledgeBase(id);
      navigate(-1);
    }
  };

  if (!kb && !isLoading) {
    return (
      <PageLayout title={t('knowledge.not_found', 'Not Found')}>
        <div className="flex flex-col items-center justify-center h-full text-text-sub">
          {t('knowledge.kb_not_found', 'Knowledge base not found')}
        </div>
      </PageLayout>
    );
  }

  return (
    <PageLayout 
      title={kb?.name || t('knowledge.documents', 'Documents')}
      rightElement={
        kb && (
          <button onClick={handleDeleteKb} className="text-red-500 p-2">
            <Settings className="w-5 h-5" />
          </button>
        )
      }
    >
      <div className="flex flex-col h-full bg-[#f8f9fc] dark:bg-[#121214]">
        
        {/* Search Bar */}
        <div className="p-4 bg-white dark:bg-[#1e1e20] shadow-sm mb-4">
          <div className="flex items-center bg-[#f3f4f6] dark:bg-[#2c2d2e] rounded-xl px-4 py-2.5 transition-colors">
            <Search className="w-5 h-5 text-text-sub mr-2" />
            <input 
              type="text" 
              placeholder={t('knowledge.search_docs', 'Search documents...')}
              className="bg-transparent border-none outline-none flex-1 text-[15px] text-text-main"
            />
          </div>
        </div>

        <div className="flex-1 overflow-y-auto px-4 pb-24">
          <div className="flex flex-col gap-3">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                 <div className="w-8 h-8 rounded-full border-4 border-primary-blue/30 border-t-primary-blue animate-spin mb-3"></div>
                 <span className="text-[14px]">{t('knowledge.loading', 'Loading...')}</span>
              </div>
            ) : documents.length > 0 ? (
              documents.map((doc) => (
                <motion.div
                  whileHover={{ scale: 1.01 }}
                  whileTap={{ scale: 0.98 }}
                  key={doc.id}
                  onClick={() => navigate(`/workspace/knowledge/${id}/doc/${doc.id}`)}
                  className="bg-white dark:bg-[#1e1e20] p-4 rounded-xl shadow-sm border border-border-color/50 flex flex-col cursor-pointer hover:shadow-md transition-all"
                >
                  <div className="flex justify-between items-start mb-2">
                    <h3 className="text-[16px] font-semibold text-text-main leading-tight line-clamp-1 pr-4">{doc.title}</h3>
                    <ChevronRight className="w-5 h-5 text-text-sub shrink-0 opacity-50" />
                  </div>
                  <p className="text-[14px] text-text-sub line-clamp-2 mb-3">
                    {doc.content}
                  </p>
                  <div className="flex items-center justify-between mt-auto pt-3 border-t border-border-color/50">
                    <span className="text-[12px] px-2.5 py-1 bg-primary-blue/10 text-primary-blue rounded-md font-medium">{doc.category}</span>
                    <div className="flex items-center gap-2">
                      <span className="text-[12px] text-text-sub">{doc.author}</span>
                      <span className="text-[12px] text-text-sub/40">•</span>
                      <span className="text-[12px] text-text-sub">{new Date(doc.createdAt).toLocaleDateString()}</span>
                    </div>
                  </div>
                </motion.div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <FileText className="w-14 h-14 mb-4 stroke-current opacity-30" />
                <span className="text-[15px]">{t('knowledge.no_docs', 'No documents found')}</span>
                <p className="text-[13px] mt-2 opacity-70 text-center px-8">This knowledge base is empty. Create a document to get started.</p>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate(`/workspace/knowledge/${id}/doc/create`)}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/40 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};

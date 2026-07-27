import React, { useState, useEffect } from "react";
import { PageLayout, IconButton } from "@sdkwork/im-h5-commons";
import { Trash2 } from "lucide-react";
import { KnowledgeBaseService, KnowledgeDocument } from "../services/KnowledgeBaseService";
import { useNavigate, useParams } from "react-router";
import { useTranslation } from "react-i18next";

export const KnowledgeBaseDetail = () => {
  const { t } = useTranslation();
  
const { id } = useParams();
  const navigate = useNavigate();
  
  const [doc, setDoc] = useState<KnowledgeDocument | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    setIsLoading(true);
    KnowledgeBaseService.getDocument(id).then((data) => {
      setDoc(data);
      setIsLoading(false);
    });
  }, [id]);

  const handleDelete = async () => {
    if (!id) return;
    if (window.confirm(t('knowledge.deleteConfirm', 'Are you sure you want to delete this document?'))) {
      await KnowledgeBaseService.deleteDocument(id);
      navigate(-1);
    }
  };

  if (isLoading) {
    return (
      <PageLayout title={t('knowledge.detailTitle', 'Document Detail')}>
        <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70 h-full">
           <div className="w-8 h-8 rounded-full border-4 border-primary-blue/30 border-t-primary-blue animate-spin mb-3"></div>
        </div>
      </PageLayout>
    );
  }

  if (!doc) {
    return (
      <PageLayout title={t('knowledge.detailTitle', 'Document Detail')}>
        <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70 h-full">
          {t('knowledge.notFound', 'Document not found')}
        </div>
      </PageLayout>
    );
  }

  return (
    <PageLayout 
      title={""}
      rightElement={
        <IconButton
          icon={<Trash2 className="w-5 h-5 text-red-500" />}
          onClick={handleDelete}
        />
      }
    >
      <div className="flex flex-col h-full bg-white dark:bg-[#121214] p-6 overflow-y-auto">
        
        <div className="mb-6">
          <div className="inline-block px-2.5 py-1 bg-primary-blue/10 text-primary-blue text-[12px] font-semibold rounded-md mb-3">
            {doc.category}
          </div>
          <h1 className="text-[26px] font-bold text-text-main leading-snug tracking-tight">{doc.title}</h1>
        </div>
        
        <div className="flex items-center gap-3 mb-8 pb-6 border-b border-border-color/50">
          <div className="w-10 h-10 rounded-full bg-gradient-to-tr from-blue-500 to-primary-blue flex items-center justify-center text-white font-medium shadow-sm">
            {doc.author.charAt(0).toUpperCase()}
          </div>
          <div>
            <div className="text-[14px] font-medium text-text-main">{doc.author}</div>
            <div className="text-[12px] text-text-sub mt-0.5">{new Date(doc.createdAt).toLocaleString()}</div>
          </div>
        </div>

        <div className="text-[16px] text-text-main leading-loose whitespace-pre-wrap font-serif">
          {doc.content}
        </div>
      </div>
    </PageLayout>
  );
};

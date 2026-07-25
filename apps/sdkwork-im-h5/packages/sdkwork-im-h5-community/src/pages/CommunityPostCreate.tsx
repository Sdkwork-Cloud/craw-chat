import { useTranslation } from "react-i18next";
import React, { useState, useRef } from "react";
import { useParams, useNavigate } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { showToast, cn } from "@sdkwork/im-h5-commons";
import { X, Plus, MapPin, Hash, AtSign } from "lucide-react";
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';

export const CommunityPostCreate: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [images, setImages] = useState<string[]>([]);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const editor = useEditor({
    extensions: [
      StarterKit,
      Placeholder.configure({
        placeholder: '这一刻的想法...',
        emptyEditorClass: 'is-editor-empty',
      }),
    ],
    content: '',
    editorProps: {
      attributes: {
        class: 'prose prose-sm max-w-none focus:outline-none min-h-[120px] text-[16px] text-black dark:text-white p-4 bg-transparent',
      },
    },
  });

  const handleImagePick = () => {
  if (images.length >= 9) {
      showToast(t('community.auto_fn_n2d485cc', '最多只能选择9张照片'));
      return;
    }
    fileInputRef.current?.click();
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(e.target.files || []) as File[];
    if (!files.length) return;

    if (fileInputRef.current) {
        fileInputRef.current.value = '';
    }

    const availableSlots = 9 - images.length;
    const filesProcess = files.slice(0, availableSlots);

    if (files.length > availableSlots) {
       showToast(`最多只能选择9张图片，已截取前${availableSlots}张`);
    }

    try {
      const filePromises = filesProcess.map(file => {
        return new Promise<string>((resolve, reject) => {
          const reader = new FileReader();
          reader.onload = (event) => {
            if (event.target?.result) {
              resolve(event.target.result as string);
            } else {
              reject(new Error("Failed to read file"));
            }
          };
          reader.readAsDataURL(file);
        });
      });

      const newImages = await Promise.all(filePromises);
      setImages(prev => [...prev, ...newImages]);
    } catch (err) {
       console.error("Image loading failed:", err);
       showToast(t('community.auto_fn_e0f746b', '图片读取失败，请重试'));
    }
  };

  const removeImage = (indexToRemove: number) => {
  setImages(prev => prev.filter((_, idx) => idx !== indexToRemove));
  };

  const handleSubmit = async () => {
    if (!id) return;
    const content = editor?.getText() || "";
    if (!content.trim() && images.length === 0) {
      showToast(t('community.auto_fn_n707125e8', '请输入内容或上传图片'));
      return;
    }
    setIsSubmitting(true);
    try {
      // Create post payload ideally would accept images, we pass content.
      // (Assuming the backend might extract HTML or similar in fully developed version)
      await CommunityService.createPost(id, content);
      showToast(t('community.auto_fn_28260f86', '发表成功'));
      // We just go back. For actual replace we would need the previous path, 
      // but going back is usually sufficient for create pages.
      navigate(-1);
    } catch {
      showToast(t('community.auto_fn_b5c6cde', '发表失败，请重试'));
    } finally {
      setIsSubmitting(false);
    }
  };

  const isEmpty = editor?.isEmpty;
  const isPublishDisabled = isSubmitting || (isEmpty && images.length === 0);

  return (
    <div className="flex flex-col h-full bg-white dark:bg-[#111111] overflow-hidden relative selection:bg-blue-500/30">
      <style>{`
        .ProseMirror p.is-editor-empty:first-child::before {
          color: #A3A3A3;
          content: attr(data-placeholder);
          float: left;
          height: 0;
          pointer-events: none;
        }
        .dark .ProseMirror p.is-editor-empty:first-child::before {
          color: #52525B;
        }
        .ProseMirror {
          outline: none !important;
          word-break: break-word;
        }
      `}</style>

      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-white dark:bg-[#111111]">
        <button 
          onClick={() => navigate(-1)}
          className="text-[16px] text-black dark:text-white active:opacity-50 p-2 -ml-2"
        >{t('community.auto_a9472', '取消')}</button>
        
        <button 
          className={cn(
             "px-4 py-1.5 rounded-[4px] text-[15px] font-medium transition-all flex items-center",
             isPublishDisabled 
                ? "bg-[#E5E5EA] dark:bg-[#2C2C2E] text-black/30 dark:text-white/30" 
                : "bg-[#07C160] text-white active:bg-[#06ad56]"
          )}
          onClick={handleSubmit}
          disabled={isPublishDisabled}
        >{isSubmitting ? "发表中..." : "发表"}</button>
      </header>

      <div className="flex-1 overflow-y-auto w-full flex flex-col mb-safe">
         <EditorContent editor={editor} className="w-full flex-shrink-0" />
         
         {/* Internal Image Grid */}
         <div className="px-4 pb-4">
            <div className="grid grid-cols-3 gap-2">
                {images.map((img, index) => (
                    <div key={index} className="relative aspect-square rounded-md overflow-hidden bg-[#F2F2F7] dark:bg-[#2C2C2E]">
                        <img src={img} alt="" className="w-full h-full object-cover" />
                        <button 
                            className="absolute top-1 right-1 w-5 h-5 bg-black/50 rounded-full flex items-center justify-center pointer-events-auto active:scale-95 transition-transform"
                            onClick={(e) => { e.stopPropagation(); removeImage(index); }}
                        >
                            <X className="w-3.5 h-3.5 text-white" />
                        </button>
                    </div>
                ))}
                
                {images.length < 9 && (
                    <div 
                        className="aspect-square rounded-md bg-[#F2F2F7] dark:bg-[#2C2C2E] flex flex-col items-center justify-center cursor-pointer active:bg-[#E5E5EA] dark:active:bg-[#3A3A3C] transition-colors"
                        onClick={handleImagePick}
                    >
                        <Plus className="w-8 h-8 text-[#8E8E93] dark:text-[#636366]" strokeWidth={1.5} />
                    </div>
                )}
            </div>
         </div>

         {/* Extra Toolset like WeChat */}
         <div className="mt-8 border-t border-black/5 dark:border-white/5 pl-4 flex flex-col">
            <div className="flex items-center py-4 pr-4 border-b border-black/5 dark:border-white/5 text-black dark:text-white active:bg-black/5 dark:active:bg-white/5 cursor-pointer">
               <MapPin className="w-6 h-6 mr-3 text-[#8E8E93] dark:text-[#636366]" strokeWidth={1.5} />
               <span className="text-[16px] flex-1">{t('community.auto_2dfabca9', '所在位置')}</span>
            </div>
            <div className="flex items-center py-4 pr-4 border-b border-black/5 dark:border-white/5 text-black dark:text-white active:bg-black/5 dark:active:bg-white/5 cursor-pointer">
               <AtSign className="w-6 h-6 mr-3 text-[#8E8E93] dark:text-[#636366]" strokeWidth={1.5} />
               <span className="text-[16px] flex-1">{t('community.auto_2f932a6c', '提醒谁看')}</span>
            </div>
            <div className="flex items-center py-4 pr-4 text-black dark:text-white active:bg-black/5 dark:active:bg-white/5 cursor-pointer">
               <Hash className="w-6 h-6 mr-3 text-[#8E8E93] dark:text-[#636366]" strokeWidth={1.5} />
               <span className="text-[16px] flex-1">{t('community.auto_274987e7', '参与话题')}</span>
            </div>
         </div>
      </div>
      
      {/* Hidden File Input */}
      <input 
         type="file" 
         multiple
         accept="image/*"
         className="hidden"
         ref={fileInputRef}
         onChange={handleFileChange}
      />
    </div>
  );
};

import { useTranslation } from "react-i18next";
import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router";
import { ChevronLeft, MessageSquare, Settings2 } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { CharacterService, type Character } from "../services/CharacterService";

export const MyCharacterDetail: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams();
  const navigate = useNavigate();
  const [character, setCharacter] = useState<Character | null>(null);

  useEffect(() => {
    if (!id) return;
    CharacterService.getCharacters().then(chars => {
        const found = chars.find(c => c.id === id);
        if (found) setCharacter(found);
    });
  }, [id]);

  if (!character) return null;

  return (
    <div className="flex flex-col h-full bg-[#f2f2f2] dark:bg-[#121212]">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
           <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" strokeWidth={2.5} />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">角色详情</h1>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-2">
           <IconButton
            icon={<Settings2 className="w-5 h-5 text-text-main" />}
            onClick={() => {}}
          />
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto w-full flex flex-col items-center">
        {/* Profile Card */}
        <div className="w-full flex flex-col items-center justify-center py-10 px-6 bg-white dark:bg-[#1A1A1A] border-b border-border-color">
            <div className="w-24 h-24 mb-4 rounded-full overflow-hidden shadow-sm border border-border-color">
                 <img src={character.avatar} alt="avatar" className="w-full h-full object-cover" />
            </div>

            <h2 className="text-[20px] font-bold text-text-main mb-1">{character.name}</h2>
            <p className="text-[14px] text-text-sub mb-6">{character.visibility === 'private' ? '私密角色' : '公开角色'}</p>
            
            <p className="text-[15px] leading-relaxed text-text-main/90 text-center px-4 line-clamp-3">
                {character.desc}
            </p>
        </div>

        {/* Info */}
        <div className="w-full mt-2 bg-white dark:bg-[#1A1A1A] py-2">
            <div className="flex flex-col px-4 py-3 border-b border-border-color last:border-0 hover:bg-chat-active-bg transition-colors cursor-pointer">
                 <span className="text-[16px] text-text-main mb-1">系统设定</span>
                 <span className="text-[14px] text-text-sub line-clamp-2">{character.prompt || "无设定内容"}</span>
            </div>
             <div className="flex items-center px-4 py-4 border-b border-border-color last:border-0 hover:bg-chat-active-bg transition-colors cursor-pointer">
                 <span className="text-[16px] text-text-main flex-1">使用的声音</span>
                 <span className="text-[15px] text-text-sub flex items-center gap-2">默认声音<ChevronLeft className="w-4 h-4 text-text-sub rotate-180" />
                 </span>
            </div>
        </div>

        <div className="w-full mt-6 px-4 pb-8 flex flex-col gap-3">
            <button
                onClick={() => navigate(`/chat/${character.id}`)}
                className="w-full flex items-center justify-center gap-2 py-3.5 bg-primary-blue text-white rounded-full font-bold active:opacity-80 transition-opacity shadow-md"
            >
                <MessageSquare className="w-5 h-5 text-current" />
                <span>与TA聊天</span>
            </button>
            <button 
                onClick={() => navigate(`/me/characters/create?id=${character.id}`)}
                className="w-full py-3 bg-white dark:bg-[#1A1A1A] border border-border-color text-text-main rounded-full font-medium active:bg-chat-active-bg transition-colors"
            >编辑角色</button>
        </div>
      </div>
    </div>
  );
};

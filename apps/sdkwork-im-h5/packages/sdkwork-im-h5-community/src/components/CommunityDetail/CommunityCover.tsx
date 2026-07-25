import React from "react";
import { ChevronLeft, Share2, Settings2, Users } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { useNavigate } from "react-router";
import { Community } from "../../types";
import { useTranslation } from "react-i18next";

export const CommunityCover = ({ community }: { community: Community }) => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  

  return (
    <div className="relative h-[240px] shrink-0">
      <img src={community.coverImage} alt={community.name} className="w-full h-full object-cover" />
      <div className="absolute inset-0 bg-black/40 backdrop-blur-[2px]"></div>
      
      <header className="absolute top-0 left-0 right-0 h-[56px] px-4 flex items-center justify-between pt-safe z-10">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-white" />}
          className="bg-black/20 w-9 h-9 p-0 rounded-full backdrop-blur-md"
          onClick={() => navigate(-1)}
        />
        <div className="flex gap-2">
          <IconButton
            icon={<Share2 className="w-5 h-5 text-white" />}
            className="bg-black/20 w-9 h-9 p-0 rounded-full backdrop-blur-md"
            onClick={() => navigate(`/community/${community.id}/profile/qrcode`)}
          />
          <IconButton
            icon={<Settings2 className="w-5 h-5 text-white" />}
            className="bg-black/20 w-9 h-9 p-0 rounded-full backdrop-blur-md"
            onClick={() => navigate(`/community/${community.id}/profile`)}
          />
        </div>
      </header>
      
      <div className="absolute bottom-4 left-4 right-4 text-white">
          <div className="flex items-center gap-4">
            {community.avatar && (
              <div className="w-16 h-16 rounded-2xl border-2 border-white/20 overflow-hidden shrink-0 bg-black/20 backdrop-blur-md">
                <img src={community.avatar} alt="Avatar" className="w-full h-full object-cover" />
              </div>
            )}
            <div>
              <h2 className="text-[24px] font-bold shadow-sm">{community.name}</h2>
              <div className="flex items-center gap-4 mt-2">
                  <div className="flex items-center gap-1.5 opacity-90">
                    <Users className="w-4 h-4" />
                    <span className="text-[13px]">{t('community.auto_n4e746140', '{community.memberCount} 成员', { memberCount: community.memberCount })}</span>
                  </div>
                  <div className="flex gap-2">
                    {community.tags.slice(0,2).map(tag => (
                      <span key={tag} className="bg-white/20 backdrop-blur-md px-1.5 py-0.5 rounded text-[11px]">#{tag}</span>
                    ))}
                  </div>
              </div>
            </div>
          </div>
      </div>
    </div>
  );
};

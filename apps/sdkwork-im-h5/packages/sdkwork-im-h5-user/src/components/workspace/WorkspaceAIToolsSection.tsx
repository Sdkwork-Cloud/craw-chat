import React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { FileText, Wand2, Video, Music, Mic, Sparkles } from "lucide-react";

export const WorkspaceAIToolsSection: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <div className="pt-4 pb-6">
      <div className="flex items-center justify-between mb-4 px-1">
        <h3 className="text-[17px] font-bold flex items-center gap-2">
          <Sparkles className="w-5 h-5 text-primary-blue fill-primary-blue/20" />
          <span className="text-text-main">
            {t("workspace.ai_apps")}
          </span>
        </h3>
      </div>
      
      <div className="grid grid-cols-2 gap-3">
        {/* AI Writing */}
        <div 
          className="relative overflow-hidden rounded-[20px] p-3.5 bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20 cursor-pointer active:scale-[0.98] transition-all flex flex-col gap-3 group"
          onClick={() => navigate("/ai/writing")}
        >
          <div className="absolute right-0 bottom-0 w-24 h-24 bg-blue-500/10 blur-2xl rounded-full pointer-events-none group-hover:bg-blue-500/20 transition-colors" />
          <div className="w-10 h-10 rounded-[12px] bg-gradient-to-br from-blue-500 to-primary-blue text-white flex shrink-0 items-center justify-center shadow-md shadow-blue-500/20 relative overflow-hidden">
            <FileText className="w-[18px] h-[18px] relative z-10" />
          </div>
          <div className="relative z-10 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <h4 className="text-[14px] font-bold text-text-main truncate leading-none">{t("workspace.ai_writing")}</h4>
            </div>
            <p className="text-[11px] text-text-sub leading-tight line-clamp-2">{t("workspace.ai_writing_desc")}</p>
          </div>
        </div>
        
        {/* AI Image */}
        <div 
          className="relative overflow-hidden rounded-[20px] p-3.5 bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20 cursor-pointer active:scale-[0.98] transition-all flex flex-col gap-3 group"
          onClick={() => navigate("/ai/image")}
        >
            <div className="absolute right-0 bottom-0 w-24 h-24 bg-blue-500/10 blur-2xl rounded-full pointer-events-none group-hover:bg-blue-500/20 transition-colors" />
            <div className="w-10 h-10 rounded-[12px] bg-gradient-to-br from-blue-500 to-primary-blue text-white flex shrink-0 items-center justify-center shadow-md shadow-blue-500/20">
              <Wand2 className="w-[18px] h-[18px]" />
            </div>
            <div className="relative z-10">
              <h4 className="text-[14px] font-bold text-text-main mb-1 truncate leading-none">{t("workspace.ai_image")}</h4>
              <p className="text-[11px] text-text-sub leading-tight line-clamp-2">{t("workspace.ai_image_desc")}</p>
            </div>
        </div>

        {/* AI Video */}
        <div 
          className="relative overflow-hidden rounded-[20px] p-3.5 bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20 cursor-pointer active:scale-[0.98] transition-all flex flex-col gap-3 group"
          onClick={() => navigate("/ai/video")}
        >
            <div className="absolute right-0 bottom-0 w-24 h-24 bg-blue-500/10 blur-2xl rounded-full pointer-events-none group-hover:bg-blue-500/20 transition-colors" />
            <div className="w-10 h-10 rounded-[12px] bg-gradient-to-br from-blue-500 to-primary-blue text-white flex shrink-0 items-center justify-center shadow-md shadow-blue-500/20">
              <Video className="w-[18px] h-[18px]" />
            </div>
            <div className="relative z-10">
              <h4 className="text-[14px] font-bold text-text-main mb-1 truncate leading-none">{t("workspace.ai_video")}</h4>
              <p className="text-[11px] text-text-sub leading-tight line-clamp-2">{t("workspace.ai_video_desc")}</p>
            </div>
        </div>
        
        {/* AI Music */}
        <div 
          className="relative overflow-hidden rounded-[20px] p-3.5 bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20 cursor-pointer active:scale-[0.98] transition-all flex flex-col gap-3 group"
          onClick={() => navigate("/ai/music")}
        >
          <div className="absolute right-0 bottom-0 w-24 h-24 bg-blue-500/10 blur-2xl rounded-full pointer-events-none group-hover:bg-blue-500/20 transition-colors" />
          <div className="w-10 h-10 rounded-[12px] bg-gradient-to-br from-blue-500 to-primary-blue text-white flex shrink-0 items-center justify-center shadow-md shadow-blue-500/20 relative">
            <Music className="w-[18px] h-[18px] relative z-10" />
          </div>
          <div className="relative z-10">
            <h4 className="text-[14px] font-bold text-text-main mb-1 truncate leading-none">{t("workspace.ai_music")}</h4>
            <p className="text-[11px] text-text-sub leading-tight line-clamp-2">{t("workspace.ai_music_desc")}</p>
          </div>
        </div>

        {/* AI Voice Synth */}
        <div 
          className="relative overflow-hidden rounded-[20px] p-3.5 bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20 cursor-pointer active:scale-[0.98] transition-all flex flex-col gap-3 group"
          onClick={() => navigate("/ai/voice-synth")}
        >
          <div className="absolute right-0 bottom-0 w-24 h-24 bg-blue-500/10 blur-2xl rounded-full pointer-events-none group-hover:bg-blue-500/20 transition-colors" />
          <div className="w-10 h-10 rounded-[12px] bg-gradient-to-br from-blue-500 to-primary-blue text-white flex shrink-0 items-center justify-center shadow-md shadow-blue-500/20 relative">
            <Mic className="w-[18px] h-[18px] relative z-10" />
          </div>
          <div className="relative z-10">
            <h4 className="text-[14px] font-bold text-text-main mb-1 truncate leading-none">{t("workspace.ai_voice_synth")}</h4>
            <p className="text-[11px] text-text-sub leading-tight line-clamp-2">{t("workspace.ai_voice_synth_desc")}</p>
          </div>
        </div>

        {/* AI Voice Summary */}
        <div 
          className="relative overflow-hidden rounded-[20px] p-3.5 bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20 cursor-pointer active:scale-[0.98] transition-all flex flex-col gap-3 group"
          onClick={() => navigate("/workspace/voice-summary")}
        >
          <div className="absolute right-0 bottom-0 w-24 h-24 bg-blue-500/10 blur-2xl rounded-full pointer-events-none group-hover:bg-blue-500/20 transition-colors" />
          <div className="w-10 h-10 rounded-[12px] bg-gradient-to-br from-blue-500 to-primary-blue text-white flex shrink-0 items-center justify-center shadow-md shadow-blue-500/20 relative">
            <Mic className="w-[18px] h-[18px] relative z-10" />
          </div>
          <div className="relative z-10">
            <h4 className="text-[14px] font-bold text-text-main mb-1 truncate leading-none">{t("workspace.voice_summary", "Voice Summary")}</h4>
            <p className="text-[11px] text-text-sub leading-tight line-clamp-2">{t("workspace.voice_summary_desc", "Generate a recording summary in one tap")}</p>
          </div>
        </div>
      </div>
    </div>
  );
};


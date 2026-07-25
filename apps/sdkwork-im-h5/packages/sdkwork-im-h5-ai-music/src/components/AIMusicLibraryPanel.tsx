import { useTranslation } from "react-i18next";
import React from "react";
import { Music } from "lucide-react";
import { MusicTask } from "../pages/AIMusicPage";
import { AIMusicTaskCard } from "./AIMusicTaskCard";

interface AIMusicLibraryPanelProps {
  t: any;
  history: MusicTask[];
  setMode: (mode: "create" | "library") => void;
  playingId: string | null;
  handlePlay: (id: string) => void;
  vocalType: string;
  tempo: string;
}

export const AIMusicLibraryPanel: React.FC<AIMusicLibraryPanelProps> = ({
  t,
  history,
  setMode,
  playingId,
  handlePlay,
  vocalType,
  tempo,
}) => {
  

return (
    <div className="p-4 flex flex-col gap-4">
      {history.length > 0 ? (
        history.map((task) => (
          <AIMusicTaskCard
            key={task.id}
            task={task}
            t={t}
            playingId={playingId}
            handlePlay={handlePlay}
            vocalType={vocalType}
            tempo={tempo}
          />
        ))
      ) : (
        <div className="flex flex-col items-center justify-center py-20 text-white/40">
            <Music className="w-12 h-12 mb-3 stroke-current opacity-30" />
            <span className="text-[14px]">{t('library.empty_text')}</span>
            <button 
              onClick={() => setMode("create")}
              className="mt-4 px-6 py-2 bg-[#222] border border-white/10 rounded-full text-[13px] text-white/80 active:scale-95 transition-transform"
            >
              {t('library.go_create')}
            </button>
        </div>
      )}
    </div>
  );
};

import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Plus } from "lucide-react";
import { IconButton, ActionSheet, showToast } from "@sdkwork/im-h5-commons";
import { VoiceService, type VoiceInfo } from "@sdkwork/im-h5-commons";
import { VoiceCard } from "../components/VoiceCard";

export const MyVoices: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [playingId, setPlayingId] = useState<string | null>(null);
  const [voices, setVoices] = useState<VoiceInfo[]>([]);
  const [actionSheetItem, setActionSheetItem] = useState<VoiceInfo | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);

  useEffect(() => {
    VoiceService.getVoiceCategories().then((cats) => {
      const myCat = cats.find((c) => c.id === "my");
      if (myCat) setVoices(myCat.voices);
    });
  }, []);

  const handlePlay = (id: string) => {
  if (playingId === id) {
      setPlayingId(null);
    } else {
      setPlayingId(id);
      setTimeout(() => {
        setPlayingId(null);
      }, 3000); // simulate 3s audio
    }
  };

  const startLongPress = (voice: VoiceInfo) => {
  const handlePressStart = () => {
  setIsLongPressed(false);
      (window as any).longPressTimeout = setTimeout(() => {
        setIsLongPressed(true);
        setActionSheetItem(voice);
      }, 500);
    };

    const handlePressEnd = () => {
  clearTimeout((window as any).longPressTimeout);
    };

    return {
      onPointerDown: handlePressStart,
      onPointerUp: handlePressEnd,
      onPointerLeave: () => {
        handlePressEnd();
        setIsLongPressed(false);
      },
      onContextMenu: (e: React.MouseEvent) => {
        e.preventDefault();
        handlePressStart();
        setIsLongPressed(true);
        setActionSheetItem(voice);
        handlePressEnd();
      }
    };
  };

  const handleActionSheetSelect = (action: string) => {
  if (!actionSheetItem) return;
    if (action === 'edit') {
       navigate(`/me/voices/create?id=${actionSheetItem.id}`);
    } else if (action === 'delete') {
       setVoices(prev => prev.filter(c => c.id !== actionSheetItem.id));
       showToast(t('user.auto_fn_16b31b6', '已删除'));
    }
    setActionSheetItem(null);
  };

  return (
    <div className="flex flex-col h-full bg-[#f2f2f2] dark:bg-[#121212] relative">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">{t('user.auto_2e5c5ad6', '我的声音')}</h1>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-2">
          <IconButton
            icon={<Plus className="w-5 h-5 text-text-main" />}
            onClick={() => navigate("/me/voices/create")}
          />
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto pb-8 mt-2 w-full">
        <div className="flex flex-col border-y border-border-color">
          {voices.map((voice) => (
            <VoiceCard
              key={voice.id}
              id={voice.id}
              name={voice.label}
              type={voice.desc}
              isPlaying={playingId === voice.id}
              onClick={() => {
                if (isLongPressed) {
                  setIsLongPressed(false);
                  return;
                }
                navigate(`/me/voices/${voice.id}`);
              }}
              onPlayClick={(e) => {
                e.stopPropagation();
                handlePlay(voice.id);
              }}
              onLongPressProps={startLongPress(voice)}
            />
          ))}
        </div>
      </div>

      {actionSheetItem && (
        <ActionSheet
          isOpen={true}
          title={`${actionSheetItem.label} - 操作`}
          options={[
            { label: '编辑声音', onClick: () => handleActionSheetSelect('edit') },
            { label: '删除声音', danger: true, onClick: () => handleActionSheetSelect('delete') }
          ]}
          onClose={() => setActionSheetItem(null)}
        />
      )}
    </div>
  );
};

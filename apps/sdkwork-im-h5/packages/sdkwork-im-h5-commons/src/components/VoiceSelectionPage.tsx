import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import {
  ChevronLeft,
  Search,
  X,
  PlusCircle,
  Mic,
  Play,
  Square,
} from "lucide-react";
import { IconButton } from "./IconButton";
import { cn } from "../utils/cn";
import { VoiceService, type VoiceCategory } from "../services/VoiceService";

export const VoiceSelectionPage = ({
  currentVoiceId,
  onSelect,
  onClose,
  renderCreateVoice,
}: {
  currentVoiceId: string;
  onSelect: (voice: { id: string; label: string }) => void;
  onClose: () => void;
  renderCreateVoice?: (onCloseAndRefresh: () => void) => React.ReactNode;
}) => {
  const { t } = useTranslation();
  const [searchQuery, setSearchQuery] = useState("");
  const [activeCategoryId, setActiveCategoryId] = useState<string>("my");
  const [playingId, setPlayingId] = useState<string | null>(null);
  const [categories, setCategories] = useState<VoiceCategory[]>([]);
  const [unavailable, setUnavailable] = useState(false);
  const [showCreateVoice, setShowCreateVoice] = useState(false);

  React.useEffect(() => {
    VoiceService.getVoiceCategories()
      .then(setCategories)
      .catch(() => setUnavailable(true));
    return () => {
      window.speechSynthesis.cancel();
    };
  }, []);

  const handlePlay = (e: React.MouseEvent, vId: string, label: string) => {
  e.stopPropagation();
    if (playingId === vId) {
      window.speechSynthesis.cancel();
      setPlayingId(null);
    } else {
      window.speechSynthesis.cancel();
      setPlayingId(vId);
      const utterance = new SpeechSynthesisUtterance(
        `你好，我是${label}，很高兴为您服务。`,
      );

      const voices = window.speechSynthesis.getVoices();
      const chineseVoices = voices.filter((v) => v.lang.includes("zh"));
      if (chineseVoices.length > 0) {
        if (vId.includes("female")) {
          utterance.voice =
            chineseVoices.find(
              (v) =>
                v.name.includes("Xiaoxiao") ||
                v.name.includes("Tingting") ||
                v.name.toLowerCase().includes("female"),
            ) || chineseVoices[0];
          utterance.pitch = 1.2;
        } else if (vId.includes("male")) {
          utterance.voice =
            chineseVoices.find(
              (v) =>
                v.name.includes("Kangkang") ||
                v.name.includes("Jianjian") ||
                v.name.toLowerCase().includes("male"),
            ) || chineseVoices[0];
          utterance.pitch = 0.8;
        } else {
          utterance.voice = chineseVoices[0];
        }
      }

      utterance.onend = () => {
        setPlayingId(null);
      };
      utterance.onerror = () => {
        setPlayingId(null);
      };

      window.speechSynthesis.speak(utterance);
    }
  };

  const filteredVoices = searchQuery.trim()
    ? categories
        .flatMap((c) => c.voices)
        .filter(
          (v) => v.label.includes(searchQuery) || v.desc.includes(searchQuery),
        )
    : categories.find((c) => c.id === activeCategoryId)?.voices || [];

  const isShowingMy = !searchQuery.trim() && activeCategoryId === "my";

  return (
    <div className="fixed inset-0 z-50 bg-bg-color flex flex-col animate-in slide-in-from-right">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header shrink-0 pt-safe border-b border-border-color">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={onClose}
          />
        </div>
        <div className="absolute inset-x-0 flex flex-col items-center pointer-events-none">
          <h2 className="text-[17px] font-bold text-text-main">{t('commons.auto_42ffbcdf', 'Select voice')}</h2>
        </div>
        <div className="flex-1" />
      </header>

      {/* Search Bar */}
      <div className="px-4 py-3 shrink-0 border-b border-border-color bg-bg-color">
        <div className="flex items-center bg-chat-other-bg rounded-full px-4 py-2">
          <Search className="w-4 h-4 text-text-sub shrink-0" />
          <input
            type="text"
            placeholder={t('commons.auto_prop_n27578f97', 'Search voices...')}
            className="bg-transparent flex-1 outline-none ml-2 text-[14px] text-text-main placeholder:text-text-sub"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          {searchQuery && (
            <IconButton
              icon={<X className="w-4 h-4 text-text-sub" />}
              onClick={() => setSearchQuery("")}
              className="w-6 h-6 p-1 bg-black/10 dark:bg-white/10 rounded-full"
            />
          )}
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Fail-closed: no voice catalog API is composed (typed unavailable). */}
        {unavailable ? (
          <div className="flex-1 flex flex-col items-center justify-center gap-3 px-8 text-center">
            <Mic className="w-10 h-10 text-text-sub opacity-40" />
            <p className="text-[15px] text-text-main">
              {t('commons.feature_unavailable', 'This feature is not available yet while the real service is being integrated.')}
            </p>
          </div>
        ) : (
        <>
        {/* Left Sidebar (Categories) */}
        {!searchQuery.trim() && (
          <div className="w-[100px] bg-chat-other-bg overflow-y-auto shrink-0 flex flex-col divide-y divide-transparent border-r border-border-color">
            {categories.map((cat) => (
              <div
                key={cat.id}
                onClick={() => setActiveCategoryId(cat.id)}
                className={cn(
                  "py-4 px-2 text-[14px] text-center font-bold cursor-pointer transition-colors relative select-none",
                  activeCategoryId === cat.id
                    ? "bg-bg-color text-primary-blue"
                    : "text-text-main",
                )}
              >
                {activeCategoryId === cat.id && (
                  <div className="absolute left-0 inset-y-0 my-auto w-1 h-4 bg-primary-blue rounded-r-full" />
                )}
                {cat.name}
              </div>
            ))}
          </div>
        )}

        {/* Right Content (Voices) */}
        <div className="flex-1 overflow-y-auto bg-bg-color px-4 py-4 scroll-smooth">
          {searchQuery.trim() && filteredVoices.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-text-sub gap-3 mt-10">
              <Mic className="w-10 h-10 opacity-20" />
              <span className="text-[14px]">{t('commons.auto_n5f11116c', 'No matching voices found')}</span>
            </div>
          ) : (
            <div className="flex flex-col gap-4">
              {isShowingMy && renderCreateVoice && (
                <div
                  onClick={() => {
                    setShowCreateVoice(true);
                  }}
                  className="p-4 rounded-xl border border-dashed border-primary-blue/50 bg-primary-blue/5 flex flex-col items-center justify-center gap-2 cursor-pointer active:bg-primary-blue/10 transition-colors mb-2"
                >
                  <div className="w-10 h-10 bg-primary-blue text-white rounded-full flex items-center justify-center shadow-sm">
                    <PlusCircle className="w-5 h-5" />
                  </div>
                  <span className="text-[14px] font-bold text-primary-blue">{t('commons.auto_300803f1', 'Clone my voice')}</span>
                </div>
              )}

              {filteredVoices.map((v) => (
                <div
                  key={v.id}
                  onClick={() => onSelect(v)}
                  className={cn(
                    "p-3 rounded-xl border flex items-center justify-between cursor-pointer active:bg-black/5 dark:active:bg-white/5 transition-all overflow-hidden relative",
                    currentVoiceId === v.id
                      ? "border-primary-blue bg-primary-blue/5 shadow-sm"
                      : "border-border-color bg-bg-color",
                  )}
                >
                  <div className="flex items-center gap-3 min-w-0">
                    <div
                      onClick={(e) => handlePlay(e, v.id, v.label)}
                      className={cn(
                        "w-10 h-10 rounded-full flex items-center justify-center shrink-0 shadow-sm cursor-pointer transition-colors active:scale-95",
                        playingId === v.id
                          ? "bg-primary-blue"
                          : "bg-chat-other-bg border border-border-color",
                      )}
                    >
                      {playingId === v.id ? (
                        <Square className="w-4 h-4 text-white fill-white" />
                      ) : (
                        <Play className="w-4 h-4 text-text-main ml-0.5" />
                      )}
                    </div>
                    <div className="flex flex-col min-w-0">
                      <span
                        className={cn(
                          "text-[15px] font-bold truncate",
                          currentVoiceId === v.id
                            ? "text-primary-blue"
                            : "text-text-main",
                        )}
                      >
                        {v.label}
                      </span>
                      <span className="text-[12px] text-text-sub truncate">
                        {v.desc}
                      </span>
                    </div>
                  </div>
                  {currentVoiceId === v.id && (
                    <div className="w-5 h-5 bg-primary-blue rounded-full border border-primary-blue flex items-center justify-center shrink-0">
                      <div className="w-2 h-2 bg-white rounded-full" />
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
        </>
        )}
      </div>

      {/* Create Voice Flow Overlay */}
      {showCreateVoice && renderCreateVoice && (
        <div className="fixed inset-0 z-[60] bg-bg-color animate-in slide-in-from-right">
          {renderCreateVoice(() => {
            setShowCreateVoice(false);
            VoiceService.getVoiceCategories()
              .then(setCategories)
              .catch(() => setUnavailable(true));
          })}
        </div>
      )}
    </div>
  );
};

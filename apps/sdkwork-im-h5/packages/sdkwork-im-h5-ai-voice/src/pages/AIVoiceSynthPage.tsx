import React, { useState } from "react";
import { PageLayout, showToast, VoiceSelectionPage, VoiceService } from "@sdkwork/im-h5-commons";
import { Mic, Loader2 } from "lucide-react";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { VoiceInputCard } from "../components/VoiceInputCard";
import { VoiceSpeakerCard } from "../components/VoiceSpeakerCard";
import { VoiceAudioPlayerCard } from "../components/VoiceAudioPlayerCard";

export const AIVoiceSynthPage: React.FC = () => {
  const { t } = useTranslation();
  const [text, setText] = useState("");
  const [isSynthesizing, setIsSynthesizing] = useState(false);
  const [isPlaying, setIsPlaying] = useState(false);
  const [audioUrl, setAudioUrl] = useState<string | null>(null);
  
  const [isVoiceSelectorOpen, setIsVoiceSelectorOpen] = useState(false);
  const [selectedVoiceId, setSelectedVoiceId] = useState("female1");
  const [voices, setVoices] = useState<any[]>([]);

  React.useEffect(() => {
    VoiceService.getVoiceCategories().then((cats) => {
      const flattened = cats.flatMap((c) =>
        c.voices.map((v) => ({ id: v.id, name: v.label, type: c.name })),
      );
      setVoices(flattened);
    });
  }, []);

  const [speed, setSpeed] = useState(1.0);
  const [pitch, setPitch] = useState(0);
  const [volume, setVolume] = useState(50);
  const [showAdvanced, setShowAdvanced] = useState(false);

  const selectedVoice = voices.find(v => v.id === selectedVoiceId) || voices[0] || { name: '加载中...', type: '' };

  const handleSynthesize = () => {
    if (!text.trim()) {
      showToast(t("voice_synth.input_empty"));
      return;
    }
    
    setIsSynthesizing(true);
    // Mock synthesize delay
    setTimeout(() => {
      setIsSynthesizing(false);
      setAudioUrl("mock-audio-url");
      setIsPlaying(true);
      showToast(t("voice_synth.success"));
      
      setTimeout(() => {
         setIsPlaying(false);
      }, 3000);
    }, 2000);
  };

  const togglePlay = () => {
    if (!audioUrl) return;
    setIsPlaying(!isPlaying);
  };

  return (
    <PageLayout title={t("voice_synth.title")}>
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c] p-4 overflow-y-auto pb-safe">
        <VoiceInputCard
          text={text}
          setText={setText}
          placeholder={t("voice_synth.placeholder")}
        />

        <VoiceSpeakerCard
          t={t}
          selectedVoice={selectedVoice}
          onOpenVoiceSelector={() => setIsVoiceSelectorOpen(true)}
          showAdvanced={showAdvanced}
          setShowAdvanced={setShowAdvanced}
          speed={speed}
          setSpeed={setSpeed}
          pitch={pitch}
          setPitch={setPitch}
          volume={volume}
          setVolume={setVolume}
        />

        {audioUrl && (
          <VoiceAudioPlayerCard
            t={t}
            audioUrl={audioUrl}
            isPlaying={isPlaying}
            togglePlay={togglePlay}
          />
        )}

        <div className="mt-auto pt-4 pb-4">
          <motion.button
            whileTap={{ scale: 0.98 }}
            onClick={handleSynthesize}
            disabled={isSynthesizing || !text.trim()}
            className="w-full py-4 bg-gradient-to-tr from-blue-600 to-primary-blue text-white font-medium rounded-2xl flex justify-center items-center gap-2 shadow-lg shadow-blue-500/30 disabled:opacity-50 disabled:shadow-none"
          >
            {isSynthesizing ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                {t("voice_synth.synthesizing")}
              </>
            ) : (
              <>
                <Mic className="w-5 h-5" />
                {t("voice_synth.synthesize_now")}
              </>
            )}
          </motion.button>
        </div>
      </div>

      {isVoiceSelectorOpen && (
        <VoiceSelectionPage
          currentVoiceId={selectedVoiceId}
          onSelect={(v) => {
            setSelectedVoiceId(v.id);
            setIsVoiceSelectorOpen(false);
          }}
          onClose={() => setIsVoiceSelectorOpen(false)}
        />
      )}
    </PageLayout>
  );
};

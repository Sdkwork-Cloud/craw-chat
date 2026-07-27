import { useTranslation } from "react-i18next";
import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Mic,
  UploadCloud,
  Square,
  Play,
  RotateCcw,
  CheckCircle2,
  Sparkles,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { CreateVoiceProcessingStep } from "../components/voice/CreateVoiceProcessingStep";
import { CreateVoiceDetailsStep } from "../components/voice/CreateVoiceDetailsStep";
import { CreateVoicePreviewStep } from "../components/voice/CreateVoicePreviewStep";
import { CreateVoiceRecordStep } from "../components/voice/CreateVoiceRecordStep";
import { CreateVoiceUploadStep } from "../components/voice/CreateVoiceUploadStep";

export const CreateVoice: React.FC<{ onClose?: () => void }> = ({ onClose }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<"record" | "upload">("record");
  const [recordingState, setRecordingState] = useState<
    "idle" | "recording" | "recorded" | "processing" | "done"
  >("idle");
  const [timer, setTimer] = useState(0);
  const timerRef = useRef<NodeJS.Timeout | null>(null);

  const [previewLang, setPreviewLang] = useState("中文");

  useEffect(() => {
    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, []);

  const formatTime = (seconds: number) => {
  const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  };

  const startRecording = () => {
  setRecordingState("recording");
    setTimer(0);
    timerRef.current = setInterval(() => {
      setTimer((prev) => prev + 1);
    }, 1000);
  };

  const stopRecording = () => {
  setRecordingState("recorded");
    if (timerRef.current) clearInterval(timerRef.current);
  };

  const reRecord = () => {
  setRecordingState("idle");
    setTimer(0);
  };

  const [isPreviewPlaying, setIsPreviewPlaying] = useState(false);

  const togglePreview = () => {
  if (isPreviewPlaying) {
      setIsPreviewPlaying(false);
    } else {
      setIsPreviewPlaying(true);
      setTimeout(() => {
        setIsPreviewPlaying(false);
      }, 3000); // end after 3s
    }
  };

  const [voiceName, setVoiceName] = useState("");
  const [voiceDesc, setVoiceDesc] = useState("");

  const cloneVoice = () => {
  setIsPreviewPlaying(false);
    setRecordingState("processing");
    setTimeout(() => {
      setRecordingState("done");
    }, 2500); // simulate 2.5s clone process
  };

  const handleUpload = () => {
  const input = document.createElement("input");
    input.type = "file";
    input.accept = "audio/*";
    input.onchange = () => {
      setRecordingState("processing");
      setTimeout(() => {
        setRecordingState("recorded");
      }, 2500);
    };
    input.click();
  };

  const saveVoice = async () => {
    if (!voiceName.trim()) return;
    const { VoiceService } = await import("@sdkwork/im-h5-commons");
    await VoiceService.addCustomVoice(voiceName, voiceDesc || "新克隆的音色");
    if (onClose) {
      onClose();
    } else {
      navigate(-1);
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => {
              if (onClose) onClose();
              else navigate(-1);
            }}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">{t('user.auto_2ab296de', '声音克隆')}</h1>
        </div>
        <div className="flex-1" />
      </header>

      {/* Tabs */}
      {recordingState !== "processing" && recordingState !== "done" && recordingState !== "recorded" && (
        <div className="px-4 py-3 shrink-0 flex items-center justify-center gap-6">
          <button
            className={cn(
              "text-[16px] font-medium transition-colors",
              activeTab === "record"
                ? "text-primary-blue text-[17px]"
                : "text-text-sub",
            )}
            onClick={() => {
              setActiveTab("record");
              setRecordingState("idle");
            }}
          >{t('user.auto_2d9aba59', '录音克隆')}</button>
          <button
            className={cn(
              "text-[16px] font-medium transition-colors",
              activeTab === "upload"
                ? "text-primary-blue text-[17px]"
                : "text-text-sub",
            )}
            onClick={() => {
              setActiveTab("upload");
              setRecordingState("idle");
            }}
          >{t('user.auto_24b583d4', '上传音频')}</button>
        </div>
      )}

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col p-6 pb-safe overflow-hidden relative">
        <AnimatePresence mode="wait">
          {recordingState === "processing" && <CreateVoiceProcessingStep />}
          {recordingState === "done" && (
            <CreateVoiceDetailsStep
              voiceName={voiceName}
              setVoiceName={setVoiceName}
              voiceDesc={voiceDesc}
              setVoiceDesc={setVoiceDesc}
              onSave={saveVoice}
            />
          )}
          {recordingState === "recorded" && (
            <CreateVoicePreviewStep
              previewLang={previewLang}
              setPreviewLang={setPreviewLang}
              isPreviewPlaying={isPreviewPlaying}
              togglePreview={togglePreview}
              onRetake={() => {
                setRecordingState("idle");
                setTimer(0);
                setIsPreviewPlaying(false);
              }}
              onConfirm={() => setRecordingState("processing")}
            />
          )}
          {(recordingState === "idle" || recordingState === "recording") && activeTab === "record" && (
            <CreateVoiceRecordStep
              recordingState={recordingState}
              timer={timer}
              formatTime={formatTime}
              startRecording={startRecording}
              stopRecording={stopRecording}
            />
          )}
          {recordingState !== "processing" &&
            recordingState !== "done" &&
            recordingState !== "recorded" &&
            activeTab === "upload" && (
              <CreateVoiceUploadStep handleUpload={handleUpload} />
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};

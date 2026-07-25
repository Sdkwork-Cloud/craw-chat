#!/bin/bash
DIR="packages/sdkwork-im-h5-user/src/components/voice"

cat << 'COMP' > $DIR/CreateVoiceProcessingStep.tsx
import React from "react";
import { motion } from "motion/react";
import { Sparkles } from "lucide-react";

export const CreateVoiceProcessingStep: React.FC = () => (
  <motion.div
    key="processing"
    initial={{ opacity: 0, scale: 0.9 }}
    animate={{ opacity: 1, scale: 1 }}
    exit={{ opacity: 0 }}
    className="flex flex-col items-center justify-center h-full gap-6 w-full"
  >
    <div className="relative w-32 h-32 flex items-center justify-center">
      <div className="absolute inset-0 border-[4px] border-primary-blue/20 rounded-full" />
      <motion.div
        className="absolute inset-0 border-[4px] border-primary-blue rounded-full border-t-transparent"
        animate={{ rotate: 360 }}
        transition={{ repeat: Infinity, duration: 1, ease: "linear" }}
      />
      <Sparkles className="w-10 h-10 text-primary-blue" />
    </div>
    <div className="text-center">
      <h2 className="text-[20px] font-bold text-text-main mb-2">
        AI 正在克隆您的声音
      </h2>
      <p className="text-[14px] text-text-sub">
        分析音色特征并建立数字模型，请稍候...
      </p>
    </div>
  </motion.div>
);
COMP

cat << 'COMP' > $DIR/CreateVoiceDetailsStep.tsx
import React from "react";
import { motion } from "motion/react";
import { UploadCloud } from "lucide-react";

interface Props {
  voiceName: string;
  setVoiceName: (v: string) => void;
  voiceDesc: string;
  setVoiceDesc: (v: string) => void;
  onSave: () => void;
}

export const CreateVoiceDetailsStep: React.FC<Props> = ({ voiceName, setVoiceName, voiceDesc, setVoiceDesc, onSave }) => (
  <motion.div
    key="done"
    initial={{ opacity: 0, scale: 0.9 }}
    animate={{ opacity: 1, scale: 1 }}
    exit={{ opacity: 0 }}
    className="flex flex-col gap-6 w-full h-full"
  >
    <div className="flex flex-col items-center text-center mt-2">
      <h2 className="text-[20px] font-bold text-text-main mb-1">声音克隆完成</h2>
      <p className="text-[14px] text-text-sub">请完善您的专属声音信息</p>
    </div>

    <div className="flex flex-col gap-4 flex-1">
      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main ml-1">语音头像</label>
        <div className="w-20 h-20 bg-chat-other-bg border border-border-color rounded-2xl flex items-center justify-center overflow-hidden active:opacity-70 transition-opacity cursor-pointer mx-auto mb-2">
          <UploadCloud className="w-8 h-8 text-text-sub opacity-50" />
        </div>
      </div>

      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main ml-1">声音名称</label>
        <input
          type="text"
          value={voiceName}
          onChange={(e) => setVoiceName(e.target.value)}
          placeholder="例如：治愈系睡前故事音"
          className="w-full bg-chat-other-bg border border-border-color rounded-xl px-4 py-3.5 text-[15px] text-text-main outline-none focus:border-primary-blue transition-colors"
        />
      </div>

      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main ml-1">声音简介</label>
        <textarea
          value={voiceDesc}
          onChange={(e) => setVoiceDesc(e.target.value)}
          placeholder="描述一下这个声音的特点或用途..."
          rows={3}
          className="w-full bg-chat-other-bg border border-border-color rounded-xl px-4 py-3.5 text-[15px] text-text-main outline-none focus:border-primary-blue transition-colors resize-none mb-4"
        />
      </div>
    </div>

    <div className="mt-auto shrink-0 mb-4">
      <button
        onClick={onSave}
        disabled={!voiceName.trim()}
        className="w-full py-3.5 bg-primary-blue text-white rounded-full font-bold text-[16px] shadow-lg shadow-primary-blue/20 active:opacity-80 transition-opacity disabled:opacity-50"
      >
        保存我的专属声音
      </button>
    </div>
  </motion.div>
);
COMP

cat << 'COMP' > $DIR/CreateVoicePreviewStep.tsx
import React from "react";
import { motion } from "motion/react";
import { Sparkles, Square, Play, RotateCcw, CheckCircle2 } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

interface Props {
  previewLang: string;
  setPreviewLang: (v: string) => void;
  isPreviewPlaying: boolean;
  togglePreview: () => void;
  onRetake: () => void;
  onConfirm: () => void;
}

export const CreateVoicePreviewStep: React.FC<Props> = ({ previewLang, setPreviewLang, isPreviewPlaying, togglePreview, onRetake, onConfirm }) => (
  <motion.div
    key="recorded-mode"
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    className="w-full flex-1 flex flex-col min-h-0 pt-4"
  >
    <div className="flex-1 bg-chat-other-bg rounded-3xl p-8 shadow-sm border border-border-color flex flex-col items-center justify-center w-full relative">
      <div className="absolute top-4 right-4 flex items-center gap-1 bg-black/5 dark:bg-white/5 rounded-full p-1">
        {['中文', 'English'].map(lang => (
          <button
            key={lang}
            onClick={() => setPreviewLang(lang)}
            className={cn("px-4 py-1.5 rounded-full text-[13px] font-medium transition-colors", previewLang === lang ? "bg-white dark:bg-[#333] shadow-sm text-text-main" : "text-text-sub")}
          >
            {lang}
          </button>
        ))}
      </div>
      
      <h3 className="text-[16px] font-medium text-text-sub mb-6 flex items-center gap-2">
        <Sparkles className="w-5 h-5 text-primary-blue" />
        克隆成功，快来试听一下吧
      </h3>
      <p className="text-[22px] leading-relaxed text-text-main/90 font-serif tracking-wide text-center mt-2 px-4">
        {previewLang === '中文' ? '“您好，我是您的专属AI智能语音伴侣。”' : '"Hello, I am your personal AI voice companion."'}
      </p>
    </div>

    <div className="shrink-0 flex flex-col items-center w-full mt-8 mb-6 gap-6">
      <div className="flex flex-col items-center w-full max-w-[300px] mx-auto">
        <button
          onClick={togglePreview}
          className="flex flex-col items-center gap-3 active:scale-95 transition-transform mb-6"
        >
          <div className="w-16 h-16 bg-primary-blue rounded-full flex items-center justify-center shadow-md relative">
            {isPreviewPlaying && (
              <motion.div
                className="absolute inset-0 border-[2px] border-primary-blue rounded-full"
                animate={{ scale: [1, 1.25, 1], opacity: [0.6, 0, 0.6] }}
                transition={{ repeat: Infinity, duration: 1.5 }}
              />
            )}
            {isPreviewPlaying ? (
              <Square className="w-6 h-6 text-white fill-current relative z-10" />
            ) : (
              <Play className="w-7 h-7 text-white fill-current ml-1 relative z-10" />
            )}
          </div>
          <span className="text-[14px] font-medium text-text-main">
            {isPreviewPlaying ? "停止试听" : "播放试听"}
          </span>
        </button>

        <div className="flex items-center justify-between w-full mt-2 px-4">
          <button
            onClick={onRetake}
            className="flex flex-col items-center gap-2 active:opacity-70 transition-opacity"
          >
            <div className="w-12 h-12 bg-chat-other-bg rounded-full flex items-center justify-center border border-border-color">
              <RotateCcw className="w-5 h-5 text-text-sub" />
            </div>
            <span className="text-[13px] text-text-sub">重新录制</span>
          </button>
          
          <button
            onClick={onConfirm}
            className="flex flex-col items-center gap-2 active:opacity-70 transition-opacity"
          >
            <div className="w-12 h-12 bg-primary-blue/10 rounded-full flex items-center justify-center border border-primary-blue/20">
              <CheckCircle2 className="w-6 h-6 text-primary-blue" />
            </div>
            <span className="text-[13px] font-medium text-primary-blue">确认使用</span>
          </button>
        </div>
      </div>
    </div>
  </motion.div>
);
COMP

cat << 'COMP' > $DIR/CreateVoiceRecordStep.tsx
import React from "react";
import { motion } from "motion/react";
import { Mic, Square } from "lucide-react";

interface Props {
  recordingState: string;
  timer: number;
  formatTime: (s: number) => string;
  startRecording: () => void;
  stopRecording: () => void;
}

export const CreateVoiceRecordStep: React.FC<Props> = ({ recordingState, timer, formatTime, startRecording, stopRecording }) => (
  <motion.div
    key="record-mode"
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    className="w-full flex-1 flex flex-col h-full min-h-0"
  >
    <div className="flex-1 bg-chat-other-bg rounded-3xl p-6 shadow-sm border border-border-color flex flex-col justify-center min-h-[200px]">
      <h3 className="text-[16px] font-bold text-text-main mb-4 text-center">
        请使用普通话朗读以下文本
      </h3>
      <div className="relative">
        <p className="text-[22px] leading-relaxed text-text-main/90 font-serif tracking-wide text-center">
          "清晨的阳光透过树叶的缝隙，洒在林间小路上。微风拂过，带来阵阵花香，这是美好的一天开始。"
        </p>
      </div>
      <div className="mt-8 flex justify-center">
        <span className="text-[32px] font-mono font-bold tracking-wider text-text-main tabular-nums">
          {formatTime(timer)}
        </span>
      </div>
    </div>

    <div className="shrink-0 flex justify-center w-full mt-10 mb-6 min-h-[100px]">
      {recordingState === "idle" && (
        <button
          onClick={startRecording}
          className="flex flex-col items-center gap-3 active:opacity-70 transition-opacity pb-2"
        >
          <div className="w-20 h-20 bg-red-500 rounded-full flex items-center justify-center shadow-lg shadow-red-500/20">
            <Mic className="w-8 h-8 text-white" />
          </div>
          <span className="text-[14px] font-medium text-text-sub">点击开始录音</span>
        </button>
      )}

      {recordingState === "recording" && (
        <div className="flex flex-col items-center gap-3 pb-2">
          <button
            onClick={stopRecording}
            className="relative w-20 h-20 bg-red-500 rounded-full flex items-center justify-center active:scale-95 transition-transform shadow-lg shadow-red-500/20"
          >
            <motion.div
              className="absolute inset-0 bg-red-500 rounded-full"
              animate={{ scale: [1, 1.4, 1], opacity: [0.5, 0, 0.5] }}
              transition={{ repeat: Infinity, duration: 1.5 }}
            />
            <Square className="w-8 h-8 text-white fill-current relative z-10" />
          </button>
          <span className="text-[14px] font-bold text-red-500 tracking-wide">
            录音中...，点击结束
          </span>
        </div>
      )}
    </div>
  </motion.div>
);
COMP

cat << 'COMP' > $DIR/CreateVoiceUploadStep.tsx
import React from "react";
import { motion } from "motion/react";
import { UploadCloud } from "lucide-react";

interface Props {
  handleUpload: () => void;
}

export const CreateVoiceUploadStep: React.FC<Props> = ({ handleUpload }) => (
  <motion.div
    key="upload-mode"
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    className="w-full flex-1 flex flex-col items-center justify-center gap-8 h-full min-h-0"
  >
    <div className="w-32 h-32 bg-primary-blue/5 rounded-full flex items-center justify-center border-2 border-dashed border-primary-blue/30">
      <UploadCloud className="w-12 h-12 text-primary-blue" />
    </div>
    <div className="text-center px-4">
      <h3 className="text-[18px] font-bold text-text-main mb-2">上传本地音频</h3>
      <p className="text-[14px] text-text-sub leading-relaxed">
        请上传包含清晰人声的音频文件<br />
        建议时长 1 分钟到 3 分钟<br />
        支持 MP3, WAV, M4A 格式
      </p>
    </div>
    <button
      onClick={handleUpload}
      className="px-10 py-3.5 bg-primary-blue text-white rounded-full font-bold text-[16px] shadow-lg shadow-primary-blue/20 active:opacity-80 transition-opacity whitespace-nowrap"
    >
      选择文件并开始生成
    </button>
  </motion.div>
);
COMP


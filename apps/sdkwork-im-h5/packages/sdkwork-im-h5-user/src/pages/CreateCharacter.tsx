import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { ChevronLeft, ChevronRight, UploadCloud, Mic, X, Video, Image as ImageIcon } from "lucide-react";
import { IconButton, cn, VoiceSelectionPage, VoiceService, showToast } from "@sdkwork/im-h5-commons";
import { CharacterService } from "../services/CharacterService";
import { CreateVoice } from "./CreateVoice";
import { CharacterAssetSelectorModal } from "../components/CharacterAssetSelectorModal";
import { motion, AnimatePresence } from "motion/react";

export const CreateCharacter: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const editId = searchParams.get("id");

  const [name, setName] = useState("");
  const [desc, setDesc] = useState("");
  const [gender, setGender] = useState<"female" | "male" | "other">("female");
  const [voice, setVoice] = useState("female1");
  const [showVoiceSelector, setShowVoiceSelector] = useState(false);
  const [voices, setVoices] = useState<any[]>([]);
  const [showAssetSelector, setShowAssetSelector] = useState(false);
  const [assets, setAssets] = useState<{
    referenceImage: string | null;
    introVideo: string | null;
  }>({ referenceImage: null, introVideo: null });

  useEffect(() => {
    if (editId) {
      CharacterService.getCharacters().then(chars => {
          const char = chars.find(c => c.id === editId);
          if (char) {
              setName(char.name);
              setDesc(char.desc);
              if (char.gender) setGender(char.gender);
              if (char.voice) setVoice(char.voice);
              if (char.assets) setAssets(char.assets);
          }
      });
    }
  }, [editId]);

  useEffect(() => {
    VoiceService.getVoiceCategories().then((cats) => {
      const flattened = cats.flatMap((c) =>
        c.voices.map((v) => ({ id: v.id, name: v.label, type: c.name })),
      );
      setVoices(flattened);
    });
  }, []);

  const handleCreate = async () => {
    if (!name.trim()) {
        showToast(t('user.auto_fn_n9244eed', '请输入名字'));
        return;
    }
    const charData = {
        name,
        desc,
        gender,
        voice,
        assets,
        avatar: editId ? undefined : "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/newchar/200x200.png"
    };

    if (editId) {
        await CharacterService.editCharacter(editId, charData);
        showToast(t('user.auto_fn_n2d231832', '角色已更新'));
    } else {
        await CharacterService.addCharacter(charData as any);
        showToast(t('user.auto_fn_n7cbfce12', '角色创建成功'));
    }
    navigate("/me/characters", { replace: true });
  };

  const selectedVoice = voices.find((v) => v.id === voice);

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
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
          <h1 className="text-[17px] font-medium text-text-main">{editId ? "编辑角色" : "新建角色"}</h1>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-6">
        <div className="flex flex-col items-center gap-3">
          <div className="w-24 h-24 bg-chat-other-bg border border-border-color rounded-3xl flex items-center justify-center overflow-hidden active:opacity-70 transition-opacity cursor-pointer shadow-sm relative group">
            <UploadCloud className="w-8 h-8 text-text-sub opacity-50" />
            <div className="absolute inset-0 bg-black/5 opacity-0 group-hover:opacity-100 transition-opacity" />
          </div>
          <span className="text-[14px] text-text-sub font-medium ml-1">{t('user.auto_39391c8', '点击修改头像')}</span>
        </div>

        <div className="bg-chat-other-bg rounded-2xl border border-border-color/50 px-4 py-2 flex flex-col">
          <div className="flex items-center gap-4 py-3 border-b border-border-color/60">
            <span className="w-16 whitespace-nowrap text-[16px] text-text-main">{t('user.auto_a88ea', '名字')}</span>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={t('user.auto_prop_na194a50', '例如：旅行规划师')}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none focus:placeholder-opacity-50"
            />
          </div>

          <div className="flex items-center gap-4 py-3 border-b border-border-color/60">
            <span className="w-16 whitespace-nowrap text-[16px] text-text-main">{t('user.auto_bf6e4', '性别')}</span>
            <div className="flex-1 flex gap-2">
              <button
                className={cn(
                  "px-4 py-1.5 rounded-full text-[14px] transition-colors border",
                  gender === "female"
                    ? "bg-primary-blue/10 text-primary-blue border-primary-blue/30"
                    : "bg-transparent text-text-main border-border-color",
                )}
                onClick={() => setGender("female")}
              >{t('user.auto_b3514', '女性')}</button>
              <button
                className={cn(
                  "px-4 py-1.5 rounded-full text-[14px] transition-colors border",
                  gender === "male"
                    ? "bg-primary-blue/10 text-primary-blue border-primary-blue/30"
                    : "bg-transparent text-text-main border-border-color",
                )}
                onClick={() => setGender("male")}
              >{t('user.auto_e91d0', '男性')}</button>
              <button
                className={cn(
                  "px-4 py-1.5 rounded-full text-[14px] transition-colors border",
                  gender === "other"
                    ? "bg-primary-blue/10 text-primary-blue border-primary-blue/30"
                    : "bg-transparent text-text-main border-border-color",
                )}
                onClick={() => setGender("other")}
              >{t('user.auto_a2c20', '其他')}</button>
            </div>
          </div>

          <div
            className="flex items-center gap-4 py-3 cursor-pointer active:opacity-70 transition-opacity"
            onClick={() => setShowVoiceSelector(true)}
          >
            <span className="w-16 whitespace-nowrap text-[16px] text-text-main">{t('user.auto_26f36e04', '关联声音')}</span>
            <div className="flex-1 flex justify-between items-center text-[16px]">
              <span
                className={
                  voice ? "text-primary-blue font-medium" : "text-text-sub"
                }
              >{selectedVoice?.name || "请选择声音"}</span>
              <div className="flex items-center gap-1 text-text-sub">
                <Mic className="w-4 h-4" />
                <ChevronRight className="w-4 h-4 ml-1 opacity-50" />
              </div>
            </div>
          </div>

          <div
            className="flex items-center gap-4 py-3 border-t border-border-color/60 cursor-pointer active:opacity-70 transition-opacity"
            onClick={() => setShowAssetSelector(true)}
          >
            <span className="w-16 whitespace-nowrap text-[16px] text-text-main">{t('user.auto_40a164c3', '角色资产')}</span>
            <div className="flex-1 flex justify-between items-center text-[16px]">
              <span
                className={
                  (assets.referenceImage || assets.introVideo) ? "text-primary-blue font-medium" : "text-text-sub"
                }
              >{(assets.referenceImage || assets.introVideo) ? "已上传" : "未上传"}</span>
              <div className="flex items-center gap-1 text-text-sub">
                <ChevronRight className="w-4 h-4 ml-1 opacity-50" />
              </div>
            </div>
          </div>
        </div>

        <div className="flex flex-col gap-2">
          <label className="text-[14px] font-medium text-text-main ml-2">{t('user.auto_75258de4', '角色设定 / 描述信息')}</label>
          <div className="bg-chat-other-bg border border-border-color/50 rounded-2xl p-4">
            <textarea
              value={desc}
              onChange={(e) => setDesc(e.target.value)}
              placeholder={t('user.auto_prop_n6bede7ef', '详细描述这个角色的性格、功能或设定背景...')}
              rows={4}
              className="w-full bg-transparent text-[15px] text-text-main outline-none resize-none"
            />
          </div>
        </div>
      </div>

      <div className="p-4 pt-2 pb-safe shrink-0">
        <button
          onClick={handleCreate}
          className="w-full py-3.5 bg-primary-blue text-white rounded-full font-bold text-[16px] shadow-lg shadow-primary-blue/20 active:opacity-80 transition-opacity"
        >{t('user.auto_25b5df3b', '保存角色')}</button>
      </div>

      {showVoiceSelector && (
        <VoiceSelectionPage
          currentVoiceId={voice}
          onSelect={(v) => {
            setVoice(v.id);
            setShowVoiceSelector(false);
          }}
          onClose={() => setShowVoiceSelector(false)}
          renderCreateVoice={(onCloseAndRefresh) => (
            <CreateVoice onClose={onCloseAndRefresh} />
          )}
        />
      )}

      <CharacterAssetSelectorModal
        isOpen={showAssetSelector}
        assets={assets}
        onClose={() => setShowAssetSelector(false)}
        onUpdateAssets={(newAssets) => setAssets(newAssets)}
      />
    </div>
  );
};

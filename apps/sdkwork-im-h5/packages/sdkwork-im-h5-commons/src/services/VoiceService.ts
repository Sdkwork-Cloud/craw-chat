import { useTranslation } from "react-i18next";
export interface VoiceCategory {
  id: string;
  name: string;
  voices: VoiceInfo[];
}

export interface VoiceInfo {
  id: string;
  label: string;
  desc: string;
}

const INITIAL_VOICE_CATEGORIES: VoiceCategory[] = [
  {
    id: "my",
    name: "我的音色",
    voices: [
      { id: "custom_1", label: "我自己(克隆)", desc: "2026-04-01 完成克隆" },
      { id: "custom_2", label: "老婆的声音", desc: "2026-04-18 完成克隆" },
    ],
  },
  {
    id: "female",
    name: "优质女声",
    voices: [
      { id: "female1", label: "温柔女声", desc: "甜美知性、适合客服陪伴" },
      { id: "female2", label: "开朗少女", desc: "朝气蓬勃、适合元气沟通" },
      { id: "female3", label: "成熟御姐", desc: "冷静专业、适合商业播报" },
      { id: "female4", label: "儿童小女孩", desc: "稚嫩童声、适合儿童教育" },
    ],
  },
  {
    id: "male",
    name: "沉稳男声",
    voices: [
      { id: "male1", label: "沉稳大叔", desc: "磁性低沉、适合新闻朗读" },
      { id: "male2", label: "阳光青年", desc: "清晰爽朗、适合教育教学" },
      { id: "male3", label: "严肃播音", desc: "字正腔圆、适合纪录片解说" },
    ],
  },
  {
    id: "dialect",
    name: "方言特色",
    voices: [
      { id: "dialect_sz", label: "四川话特征", desc: "带四川口音的普通话" },
      { id: "dialect_tw", label: "港台腔调", desc: "柔和港台偶像剧风格" },
      { id: "game_npc", label: "史诗旁白", desc: "威武庄严、适合短视频" },
    ],
  },
];

const STORAGE_KEY = "sdkwork_im_h5_voice_categories";

let MOCK_VOICE_CATEGORIES: VoiceCategory[] = [];

const loadVoices = () => {
  if (MOCK_VOICE_CATEGORIES.length > 0) return MOCK_VOICE_CATEGORIES;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_VOICE_CATEGORIES = JSON.parse(data);
    } else {
      MOCK_VOICE_CATEGORIES = [...INITIAL_VOICE_CATEGORIES];
      saveVoices();
    }
  } catch (e) {
    MOCK_VOICE_CATEGORIES = [...INITIAL_VOICE_CATEGORIES];
  }
  return MOCK_VOICE_CATEGORIES;
};

const saveVoices = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_VOICE_CATEGORIES));
  } catch (e) {
    console.error("Failed to save voices", e);
  }
};

export const VoiceService = {
  getVoiceCategories: async (): Promise<VoiceCategory[]> => {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadVoices()]), 200),
    );
  },
  addCustomVoice: async (label: string, desc: string): Promise<void> => {
    loadVoices();
    const myCat = MOCK_VOICE_CATEGORIES.find((c) => c.id === "my");
    if (myCat) {
      myCat.voices.push({
        id: `custom_${Date.now()}`,
        label,
        desc,
      });
      saveVoices();
    }
  },
};

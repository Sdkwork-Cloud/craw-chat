import { useTranslation } from "react-i18next";
export interface Character {
  id: string;
  name: string;
  desc: string;
  avatar: string;
  visibility?: 'public' | 'private';
  prompt?: string;
  gender?: 'female' | 'male' | 'other';
  voice?: string;
  assets?: {
    referenceImage: string | null;
    introVideo: string | null;
  };
}

const INITIAL_CHARACTERS: Character[] = [
  {
    id: "char1",
    name: "旅行规划师",
    desc: "帮你规划行程安排、景点推荐和游玩攻略",
    avatar: "https://picsum.photos/seed/role1/200/200",
  },
  {
    id: "char2",
    name: "贴身语言教练",
    desc: "陪练多种语言，包含情景模拟与发音纠正",
    avatar: "https://picsum.photos/seed/role2/200/200",
  },
  {
    id: "char3",
    name: "健身私人教练",
    desc: "为你量身定制健身计划与饮食建议",
    avatar: "https://picsum.photos/seed/role3/200/200",
  },
];

const STORAGE_KEY = "clawchat_characters";

let MOCK_CHARACTERS: Character[] = [];

const loadCharacters = () => {
  if (MOCK_CHARACTERS.length > 0) return MOCK_CHARACTERS;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_CHARACTERS = JSON.parse(data);
    } else {
      MOCK_CHARACTERS = [...INITIAL_CHARACTERS];
      saveCharacters();
    }
  } catch (e) {
    MOCK_CHARACTERS = [...INITIAL_CHARACTERS];
  }
  return MOCK_CHARACTERS;
};

const saveCharacters = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_CHARACTERS));
  } catch (e) {
    console.error("Failed to save characters", e);
  }
};

export const CharacterService = {
  async getCharacters(): Promise<Character[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadCharacters()]), 200),
    );
  },

  async addCharacter(character: Omit<Character, "id">): Promise<Character> {
    loadCharacters();
    const newChar = { ...character, id: `char_${Date.now()}` };
    MOCK_CHARACTERS = [newChar, ...MOCK_CHARACTERS];
    saveCharacters();
    return newChar;
  },
  async editCharacter(id: string, character: Partial<Character>): Promise<Character> {
    loadCharacters();
    const idx = MOCK_CHARACTERS.findIndex(c => c.id === id);
    if (idx !== -1) {
      MOCK_CHARACTERS[idx] = { ...MOCK_CHARACTERS[idx], ...character };
      saveCharacters();
      return MOCK_CHARACTERS[idx];
    }
    throw new Error("Character not found");
  },
};

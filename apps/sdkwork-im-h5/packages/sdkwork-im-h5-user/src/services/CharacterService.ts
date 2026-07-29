import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";

export interface Character {
  id: string;
  name: string;
  desc: string;
  avatar: string;
  visibility?: "public" | "private";
  prompt?: string;
  gender?: "female" | "male" | "other";
  voice?: string;
  assets?: {
    referenceImage: string | null;
    introVideo: string | null;
  };
}

export const CharacterService = {
  async getCharacters(): Promise<Character[]> {
    throw new UserCapabilityUnavailableError("Character management");
  },

  async addCharacter(_character: Omit<Character, "id">): Promise<Character> {
    throw new UserCapabilityUnavailableError("Character management");
  },

  async editCharacter(_id: string, _character: Partial<Character>): Promise<Character> {
    throw new UserCapabilityUnavailableError("Character management");
  },
};

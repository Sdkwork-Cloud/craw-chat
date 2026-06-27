export const PC_MUSICGEN_CONTRACT_UNAVAILABLE = 'pc musicgen contract is not available';

export interface MusicGenService {
  generate(prompt: string): Promise<void>;
}

class SdkworkMusicGenService implements MusicGenService {
  async generate(_prompt: string): Promise<void> {
    throw new Error(PC_MUSICGEN_CONTRACT_UNAVAILABLE);
  }
}

export const musicGenService = new SdkworkMusicGenService();

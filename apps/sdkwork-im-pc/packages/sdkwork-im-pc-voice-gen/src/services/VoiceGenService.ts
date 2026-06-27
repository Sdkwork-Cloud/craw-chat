export const PC_VOICEGEN_CONTRACT_UNAVAILABLE = 'pc voicegen contract is not available';

export interface VoiceGenService {
  generate(prompt: string): Promise<void>;
}

class SdkworkVoiceGenService implements VoiceGenService {
  async generate(_prompt: string): Promise<void> {
    throw new Error(PC_VOICEGEN_CONTRACT_UNAVAILABLE);
  }
}

export const voiceGenService = new SdkworkVoiceGenService();

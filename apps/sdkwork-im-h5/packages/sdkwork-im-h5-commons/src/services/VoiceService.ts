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

export class VoiceCapabilityUnavailableError extends Error {
  constructor() {
    super("Voice catalog is unavailable because the Voice owner SDK is not composed.");
    this.name = "VoiceCapabilityUnavailableError";
  }
}

export const VoiceService = {
  async getVoiceCategories(): Promise<VoiceCategory[]> {
    throw new VoiceCapabilityUnavailableError();
  },

  async addCustomVoice(_label: string, _desc: string): Promise<void> {
    throw new VoiceCapabilityUnavailableError();
  },
};

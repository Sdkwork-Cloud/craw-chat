/**
 * Voice (音色) catalog — fail-closed (PRD).
 *
 * Audited as a pure localStorage mock with no owner backend SDK. The fake
 * voice categories, custom-voice cloning and `clawchat_*` storage are
 * removed: every method throws a typed `VoiceCapabilityUnavailableError` so
 * consumers (e.g. the voice-selection sheet) surface a typed unavailable
 * state instead of fabricated voices.
 */
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
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "VoiceCapabilityUnavailableError";
  }
}

export const VoiceService = {
  getVoiceCategories: async (): Promise<VoiceCategory[]> => {
    throw new VoiceCapabilityUnavailableError("Voice catalog");
  },
  addCustomVoice: async (_label: string, _desc: string): Promise<void> => {
    throw new VoiceCapabilityUnavailableError("Custom voice creation");
  },
};

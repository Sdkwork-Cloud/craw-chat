export interface EffectiveProtocolSnapshotResponse {
  allowedBindings: string[];
  allowedCodecs: string[];
  enabledCapabilities: string[];
  killSwitchActive: boolean;
  precedence: string[];
  protocolVersion: string;
  quotaProfileId: string;
  releaseChannel: string;
}

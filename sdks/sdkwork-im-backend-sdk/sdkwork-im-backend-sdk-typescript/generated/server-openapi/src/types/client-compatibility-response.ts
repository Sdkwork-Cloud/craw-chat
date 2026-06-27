export interface ClientCompatibilityResponse {
  blockedExperimentalCapabilities: string[];
  clientType: string;
  minimumProtocolVersion: string;
  supportedBindings: string[];
  supportedCapabilities: string[];
  supportedCodecs: string[];
}

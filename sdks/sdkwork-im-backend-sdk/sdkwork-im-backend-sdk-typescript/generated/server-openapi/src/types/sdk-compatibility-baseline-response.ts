export interface SdkCompatibilityBaselineResponse {
  appSdkFamily: string;
  backendSdkFamily: string;
  imSdkFamily: string;
  rtcSdkFamily: string;
  matrixClientTypes: string[];
  protocolGovernancePath: string;
  protocolRegistryPath: string;
}

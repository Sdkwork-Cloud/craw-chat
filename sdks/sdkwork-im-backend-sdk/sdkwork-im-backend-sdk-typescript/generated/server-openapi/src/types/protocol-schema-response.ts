export interface ProtocolSchemaResponse {
  bindingProtocols: string[];
  kind: string;
  requiredCapabilities: string[];
  schema: string;
  stage: string;
  supportedConsumers: string[];
}

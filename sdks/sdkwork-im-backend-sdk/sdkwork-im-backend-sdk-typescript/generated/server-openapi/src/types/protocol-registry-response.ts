import type { ClientCompatibilityResponse } from './client-compatibility-response';
import type { ProtocolSchemaResponse } from './protocol-schema-response';

export interface ProtocolRegistryResponse {
  bindings: string[];
  codecs: string[];
  compatibilityMatrix: ClientCompatibilityResponse[];
  protocolVersion: string;
  schemas: ProtocolSchemaResponse[];
}

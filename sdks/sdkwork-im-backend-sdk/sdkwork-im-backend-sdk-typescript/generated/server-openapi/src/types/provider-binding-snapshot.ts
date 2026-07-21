import type { ProviderBindingItem } from './provider-binding-item';

export interface ProviderBindingSnapshot {
  interfaceVersion: string;
  tenantId: string | null;
  effectiveBindings: ProviderBindingItem[];
  precedence: string[];
}

import type { BusinessPolicyVocabularyResponse } from './business-policy-vocabulary-response';
import type { CapabilityProfileResponse } from './capability-profile-response';
import type { EffectiveProtocolSnapshotResponse } from './effective-protocol-snapshot-response';
import type { KillSwitchResponse } from './kill-switch-response';
import type { QuotaProfileResponse } from './quota-profile-response';
import type { RolloutPolicyResponse } from './rollout-policy-response';
import type { SdkCompatibilityBaselineResponse } from './sdk-compatibility-baseline-response';

export interface ProtocolGovernanceResponse {
  businessPolicyVocabulary: BusinessPolicyVocabularyResponse;
  capabilityProfile: CapabilityProfileResponse;
  effectiveSnapshot: EffectiveProtocolSnapshotResponse;
  killSwitch: KillSwitchResponse;
  quotaProfile: QuotaProfileResponse;
  rolloutPolicy: RolloutPolicyResponse;
  sdkCompatibilityBaseline: SdkCompatibilityBaselineResponse;
}

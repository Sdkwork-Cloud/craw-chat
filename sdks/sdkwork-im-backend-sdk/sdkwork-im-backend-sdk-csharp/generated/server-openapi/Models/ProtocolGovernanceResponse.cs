using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ProtocolGovernanceResponse
    {
        public BusinessPolicyVocabularyResponse? BusinessPolicyVocabulary { get; set; }
        public CapabilityProfileResponse? CapabilityProfile { get; set; }
        public EffectiveProtocolSnapshotResponse? EffectiveSnapshot { get; set; }
        public KillSwitchResponse? KillSwitch { get; set; }
        public QuotaProfileResponse? QuotaProfile { get; set; }
        public RolloutPolicyResponse? RolloutPolicy { get; set; }
        public SdkCompatibilityBaselineResponse? SdkCompatibilityBaseline { get; set; }
    }
}

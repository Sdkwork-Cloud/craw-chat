using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class BusinessPolicyVocabularyResponse
    {
        public string? CapabilityFlagsField { get; set; }
        public string? HistoryVisibilityField { get; set; }
        public List<string>? HistoryVisibilityModes { get; set; }
        public string? PolicyVersionField { get; set; }
        public string? RetentionPolicyRefField { get; set; }
        public List<string>? RetentionPolicyScopes { get; set; }
    }
}

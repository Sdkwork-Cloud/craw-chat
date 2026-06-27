using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class RolloutPolicyResponse
    {
        public string? CellSelector { get; set; }
        public bool? OperatorOverride { get; set; }
        public string? PolicyId { get; set; }
        public string? RegionSelector { get; set; }
        public string? ReleaseChannel { get; set; }
        public List<string>? TenantAllowlist { get; set; }
        public int? TrafficPercent { get; set; }
    }
}

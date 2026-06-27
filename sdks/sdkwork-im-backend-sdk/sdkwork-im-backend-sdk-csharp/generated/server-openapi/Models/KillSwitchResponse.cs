using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class KillSwitchResponse
    {
        public bool? Active { get; set; }
        public List<string>? DisabledBindings { get; set; }
        public List<string>? DisabledCapabilities { get; set; }
        public List<string>? DisabledCodecs { get; set; }
        public string? Reason { get; set; }
        public string? RuleId { get; set; }
    }
}

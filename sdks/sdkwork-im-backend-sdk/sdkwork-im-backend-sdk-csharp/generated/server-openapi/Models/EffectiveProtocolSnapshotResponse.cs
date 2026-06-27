using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class EffectiveProtocolSnapshotResponse
    {
        public List<string>? AllowedBindings { get; set; }
        public List<string>? AllowedCodecs { get; set; }
        public List<string>? EnabledCapabilities { get; set; }
        public bool? KillSwitchActive { get; set; }
        public List<string>? Precedence { get; set; }
        public string? ProtocolVersion { get; set; }
        public string? QuotaProfileId { get; set; }
        public string? ReleaseChannel { get; set; }
    }
}

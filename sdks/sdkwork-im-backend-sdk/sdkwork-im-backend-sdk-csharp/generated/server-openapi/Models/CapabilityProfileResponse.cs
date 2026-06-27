using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class CapabilityProfileResponse
    {
        public List<string>? EnabledCapabilities { get; set; }
        public List<string>? ExperimentalCapabilities { get; set; }
        public string? ProfileId { get; set; }
        public string? ReleaseChannel { get; set; }
    }
}

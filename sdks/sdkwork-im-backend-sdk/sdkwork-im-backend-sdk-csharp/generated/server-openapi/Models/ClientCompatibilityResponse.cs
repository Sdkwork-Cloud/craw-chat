using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ClientCompatibilityResponse
    {
        public List<string>? BlockedExperimentalCapabilities { get; set; }
        public string? ClientType { get; set; }
        public string? MinimumProtocolVersion { get; set; }
        public List<string>? SupportedBindings { get; set; }
        public List<string>? SupportedCapabilities { get; set; }
        public List<string>? SupportedCodecs { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ProtocolRegistryResponse
    {
        public List<string>? Bindings { get; set; }
        public List<string>? Codecs { get; set; }
        public List<ClientCompatibilityResponse>? CompatibilityMatrix { get; set; }
        public string? ProtocolVersion { get; set; }
        public List<ProtocolSchemaResponse>? Schemas { get; set; }
    }
}

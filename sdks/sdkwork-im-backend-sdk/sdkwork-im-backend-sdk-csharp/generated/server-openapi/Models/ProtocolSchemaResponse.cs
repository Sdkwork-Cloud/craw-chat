using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ProtocolSchemaResponse
    {
        public List<string>? BindingProtocols { get; set; }
        public string? Kind { get; set; }
        public List<string>? RequiredCapabilities { get; set; }
        public string? Schema { get; set; }
        public string? Stage { get; set; }
        public List<string>? SupportedConsumers { get; set; }
    }
}

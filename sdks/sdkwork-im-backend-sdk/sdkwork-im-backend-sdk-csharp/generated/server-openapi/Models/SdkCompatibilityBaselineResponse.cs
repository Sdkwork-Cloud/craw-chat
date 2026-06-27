using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class SdkCompatibilityBaselineResponse
    {
        public string? AppSdkFamily { get; set; }
        public string? BackendSdkFamily { get; set; }
        public string? ImSdkFamily { get; set; }
        public string? RtcSdkFamily { get; set; }
        public List<string>? MatrixClientTypes { get; set; }
        public string? ProtocolGovernancePath { get; set; }
        public string? ProtocolRegistryPath { get; set; }
    }
}

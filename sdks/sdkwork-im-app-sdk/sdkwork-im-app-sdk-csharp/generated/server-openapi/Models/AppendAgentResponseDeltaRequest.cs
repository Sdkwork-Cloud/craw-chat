using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class AppendAgentResponseDeltaRequest
    {
        public int? FrameSeq { get; set; }
        public string? FrameType { get; set; }
        public string? SchemaRef { get; set; }
        public string? Encoding { get; set; }
        public string? Payload { get; set; }
        public Dictionary<string, string>? Attributes { get; set; }
    }
}

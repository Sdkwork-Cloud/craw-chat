using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class PostRtcSignalRequest
    {
        public string SignalType { get; set; }
        public string? SchemaRef { get; set; }
        public string Payload { get; set; }
        public string? SignalingStreamId { get; set; }
    }
}

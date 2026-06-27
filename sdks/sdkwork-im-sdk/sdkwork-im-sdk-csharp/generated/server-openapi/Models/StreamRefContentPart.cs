using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class StreamRefContentPart : ContentPart
    {
        public string Kind { get; set; }
        public string StreamId { get; set; }
        public string StreamType { get; set; }
        public string State { get; set; }
    }
}

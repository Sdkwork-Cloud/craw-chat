using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class StreamFrameView
    {
        public string StreamId { get; set; }
        public int FrameSeq { get; set; }
        public string Payload { get; set; }
        public string CreatedAt { get; set; }
    }
}

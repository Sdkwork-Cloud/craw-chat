using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class TimelineResponse
    {
        public List<TimelineViewEntry> Items { get; set; }
        public int? NextAfterSeq { get; set; }
        public bool HasMore { get; set; }
    }
}

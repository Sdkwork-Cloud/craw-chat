using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MentionContentPart : ContentPart
    {
        public string Kind { get; set; }
        public string TargetKind { get; set; }
        public string TargetId { get; set; }
        public string DisplayText { get; set; }
        public int AssignmentGeneration { get; set; }
    }
}

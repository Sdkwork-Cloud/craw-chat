using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MediaContentPart : ContentPart
    {
        public string Kind { get; set; }
        public DriveReference Drive { get; set; }
        public MediaResource Resource { get; set; }
        public string? MediaRole { get; set; }
    }
}

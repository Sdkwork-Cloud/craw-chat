using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class DataContentPart : ContentPart
    {
        public string Kind { get; set; }
        public string SchemaRef { get; set; }
        public string Encoding { get; set; }
        public string Payload { get; set; }
    }
}

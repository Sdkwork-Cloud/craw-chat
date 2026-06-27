using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class SignalContentPart : ContentPart
    {
        public string Kind { get; set; }
        public string SignalType { get; set; }
        public string? SchemaRef { get; set; }
        public string Payload { get; set; }
    }
}

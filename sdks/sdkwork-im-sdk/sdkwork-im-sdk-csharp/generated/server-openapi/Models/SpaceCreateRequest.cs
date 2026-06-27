using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class SpaceCreateRequest
    {
        public string SpaceName { get; set; }
        public string SpaceType { get; set; }
        public string? Description { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class OpenApiBlockUserRequest
    {
        public string BlockedUserId { get; set; }
        public string? Scope { get; set; }
    }
}

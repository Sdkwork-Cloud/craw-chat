using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class UpdateContactPreferencesRequest
    {
        public bool? IsStarred { get; set; }
        public string? Remark { get; set; }
        public bool? IsBlocked { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class AddConversationMemberRequest
    {
        public string PrincipalId { get; set; }
        public string PrincipalKind { get; set; }
        public string Role { get; set; }
        public Dictionary<string, object>? Attributes { get; set; }
    }
}

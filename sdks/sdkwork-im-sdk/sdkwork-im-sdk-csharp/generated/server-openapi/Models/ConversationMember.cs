using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationMember
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string MemberId { get; set; }
        public string PrincipalId { get; set; }
        public string PrincipalKind { get; set; }
        public string Role { get; set; }
        public string State { get; set; }
        public string JoinedAt { get; set; }
    }
}

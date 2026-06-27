using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessageVisibilityMutationResult
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string MessageId { get; set; }
        public int MessageSeq { get; set; }
        public string PrincipalKind { get; set; }
        public string PrincipalId { get; set; }
        public bool IsDeleted { get; set; }
        public string UpdatedAt { get; set; }
    }
}

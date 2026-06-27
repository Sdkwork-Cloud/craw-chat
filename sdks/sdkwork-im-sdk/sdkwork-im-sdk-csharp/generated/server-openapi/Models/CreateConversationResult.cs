using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateConversationResult
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string Kind { get; set; }
        public string CreatedAt { get; set; }
    }
}

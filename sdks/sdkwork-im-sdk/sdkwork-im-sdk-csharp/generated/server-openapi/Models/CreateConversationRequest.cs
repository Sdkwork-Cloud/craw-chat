using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateConversationRequest
    {
        public string? ConversationId { get; set; }
        public string? ConversationType { get; set; }
        public string? Kind { get; set; }
        public string? Title { get; set; }
        public List<string>? MemberIds { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class BindDirectChatRequest
    {
        public string? BoundAt { get; set; }
        public string? ConversationId { get; set; }
        public string? DirectChatId { get; set; }
        public string? EventId { get; set; }
        public string? LeftActorId { get; set; }
        public string? RightActorId { get; set; }
    }
}

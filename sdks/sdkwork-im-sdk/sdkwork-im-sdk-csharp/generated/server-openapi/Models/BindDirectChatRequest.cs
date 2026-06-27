using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class BindDirectChatRequest
    {
        public string? ConversationId { get; set; }
        public string? DirectChatId { get; set; }
        public string? LeftActorId { get; set; }
        public string? LeftActorKind { get; set; }
        public string? RightActorId { get; set; }
        public string? RightActorKind { get; set; }
        public string? TargetUserId { get; set; }
    }
}

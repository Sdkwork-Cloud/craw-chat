using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessageForwardReference
    {
        public string OriginalMessageId { get; set; }
        public string OriginalConversationId { get; set; }
        public string OriginalSenderId { get; set; }
        public string OriginalSenderKind { get; set; }
        public string OriginalSenderDisplayName { get; set; }
        public string OriginalOccurredAt { get; set; }
        public string ForwardedAt { get; set; }
        public string ContentPreview { get; set; }
        public int? ForwardCount { get; set; }
    }
}

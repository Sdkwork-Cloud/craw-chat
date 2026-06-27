using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class TimelineViewEntry
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string MessageId { get; set; }
        public int MessageSeq { get; set; }
        public string? Summary { get; set; }
        public Sender Sender { get; set; }
        public MessageBody Body { get; set; }
        public string MessageType { get; set; }
        public string DeliveryMode { get; set; }
        public string? ClientMsgId { get; set; }
        public string? StreamSessionId { get; set; }
        public string? RtcSessionId { get; set; }
        public string OccurredAt { get; set; }
        public string? CommittedAt { get; set; }
    }
}

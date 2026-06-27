using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ApplySharedChannelPolicyRequest
    {
        public string? AppliedAt { get; set; }
        public string? ChannelId { get; set; }
        public string? ConnectionId { get; set; }
        public string? ConversationId { get; set; }
        public string? EventId { get; set; }
        public string? HistoryVisibility { get; set; }
        public string? PolicyId { get; set; }
        public int? PolicyVersion { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationSummaryView
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public int MessageCount { get; set; }
        public int LastMessageSeq { get; set; }
        public string? LastSummary { get; set; }
        public string? LastMessageAt { get; set; }
    }
}

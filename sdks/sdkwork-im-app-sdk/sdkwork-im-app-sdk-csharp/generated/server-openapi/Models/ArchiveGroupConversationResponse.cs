using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.AppApi.Generated.Models
{
    public class ArchiveGroupConversationResponse
    {
        public bool Accepted { get; set; }
        public string ResourceId { get; set; }
        public string Status { get; set; }
        public string ArchiveEventId { get; set; }
        public string ArchivedAt { get; set; }
        public bool KnowledgebaseArchiveScheduled { get; set; }
    }
}

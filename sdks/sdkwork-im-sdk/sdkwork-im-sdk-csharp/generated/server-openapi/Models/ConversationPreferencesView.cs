using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationPreferencesView
    {
        public string TenantId { get; set; }
        public string ConversationId { get; set; }
        public string PrincipalKind { get; set; }
        public string PrincipalId { get; set; }
        public bool IsPinned { get; set; }
        public bool IsMuted { get; set; }
        public bool IsMarkedUnread { get; set; }
        public bool IsHidden { get; set; }
        public string UpdatedAt { get; set; }
    }
}

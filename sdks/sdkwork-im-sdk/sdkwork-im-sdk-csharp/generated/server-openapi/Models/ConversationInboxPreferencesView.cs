using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationInboxPreferencesView
    {
        public bool IsPinned { get; set; }
        public bool IsMuted { get; set; }
        public bool IsMarkedUnread { get; set; }
        public bool IsHidden { get; set; }
    }
}

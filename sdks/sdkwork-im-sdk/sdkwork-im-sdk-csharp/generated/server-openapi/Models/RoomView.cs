using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class RoomView
    {
        public string RoomId { get; set; }
        public string RoomKind { get; set; }
        public string ConversationId { get; set; }
        public int ActiveMemberCount { get; set; }
        public int MaxMembers { get; set; }
    }
}

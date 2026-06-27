using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ConversationInboxPeerView
    {
        public string PrincipalKind { get; set; }
        public string PrincipalId { get; set; }
        public string? UserId { get; set; }
        public string? ChatId { get; set; }
        public string? DisplayName { get; set; }
        public string? AvatarUrl { get; set; }
        public string? RelationshipState { get; set; }
    }
}

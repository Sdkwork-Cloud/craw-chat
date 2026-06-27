using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ContactView
    {
        public string TenantId { get; set; }
        public string OwnerUserId { get; set; }
        public string TargetUserId { get; set; }
        public string ContactType { get; set; }
        public string RelationshipState { get; set; }
        public string FriendshipId { get; set; }
        public string? DirectChatId { get; set; }
        public string? ConversationId { get; set; }
        public string EstablishedAt { get; set; }
        public string LastInteractionAt { get; set; }
    }
}

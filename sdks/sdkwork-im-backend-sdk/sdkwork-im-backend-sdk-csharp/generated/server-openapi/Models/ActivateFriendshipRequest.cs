using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.BackendApi.Generated.Models
{
    public class ActivateFriendshipRequest
    {
        public string? DirectChatId { get; set; }
        public string? EstablishedAt { get; set; }
        public string? EventId { get; set; }
        public string? FriendshipId { get; set; }
        public string? InitiatorUserId { get; set; }
        public string? PeerUserId { get; set; }
    }
}

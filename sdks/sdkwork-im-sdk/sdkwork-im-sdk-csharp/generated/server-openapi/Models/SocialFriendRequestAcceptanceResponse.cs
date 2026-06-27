using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class SocialFriendRequestAcceptanceResponse
    {
        public FriendRequest FriendRequest { get; set; }
        public Friendship Friendship { get; set; }
        public DirectChat DirectChat { get; set; }
        public CreateConversationResult Conversation { get; set; }
    }
}

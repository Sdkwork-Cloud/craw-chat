using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class ListMembersResponse
    {
        public List<ConversationMember> Items { get; set; }
        public string? NextCursor { get; set; }
        public bool HasMore { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class SpaceGroupMemberView
    {
        public string UserId { get; set; }
        public string Role { get; set; }
        public string? Nickname { get; set; }
        public string? MuteUntil { get; set; }
        public string JoinedAt { get; set; }
    }
}

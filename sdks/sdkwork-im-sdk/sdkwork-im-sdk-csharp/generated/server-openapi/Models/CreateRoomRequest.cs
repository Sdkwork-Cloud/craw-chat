using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class CreateRoomRequest
    {
        public string ConversationId { get; set; }
        public string RoomId { get; set; }
        public string RoomKind { get; set; }
    }
}

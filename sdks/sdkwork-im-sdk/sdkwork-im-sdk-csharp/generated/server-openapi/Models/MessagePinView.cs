using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessagePinView
    {
        public InteractionActorView PinnedBy { get; set; }
        public string PinnedAt { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class FavoriteMessageRequest
    {
        public string ConversationId { get; set; }
        public string FavoriteType { get; set; }
        public string Title { get; set; }
        public string ContentPreview { get; set; }
        public string SourceDisplayName { get; set; }
    }
}

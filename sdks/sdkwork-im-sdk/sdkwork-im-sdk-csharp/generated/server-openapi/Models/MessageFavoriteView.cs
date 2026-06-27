using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MessageFavoriteView
    {
        public string TenantId { get; set; }
        public string PrincipalKind { get; set; }
        public string PrincipalId { get; set; }
        public string FavoriteId { get; set; }
        public string FavoriteType { get; set; }
        public string ConversationId { get; set; }
        public string MessageId { get; set; }
        public int MessageSeq { get; set; }
        public string Title { get; set; }
        public string ContentPreview { get; set; }
        public string SourceDisplayName { get; set; }
        public string FavoritedAt { get; set; }
    }
}

using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class MediaResource
    {
        public string? Id { get; set; }
        public string? Kind { get; set; }
        public string? MediaKind { get; set; }
        public string Source { get; set; }
        public string Uri { get; set; }
        public string? PublicUrl { get; set; }
        public string? Url { get; set; }
        public string? Name { get; set; }
        public string? Title { get; set; }
        public string? FileName { get; set; }
        public string? MimeType { get; set; }
        public int? Size { get; set; }
        public string? SizeBytes { get; set; }
        public string? FileSize { get; set; }
        public int? DurationSeconds { get; set; }
        public MediaResource? Poster { get; set; }
        public List<MediaResource>? Thumbnails { get; set; }
    }
}

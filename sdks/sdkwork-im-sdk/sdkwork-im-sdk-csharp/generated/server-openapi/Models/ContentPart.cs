using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    [JsonPolymorphic(TypeDiscriminatorPropertyName = "kind")]
    [JsonDerivedType(typeof(TextContentPart), "text")]
    [JsonDerivedType(typeof(DataContentPart), "data")]
    [JsonDerivedType(typeof(MediaContentPart), "media")]
    [JsonDerivedType(typeof(SignalContentPart), "signal")]
    [JsonDerivedType(typeof(StreamRefContentPart), "stream_ref")]
    public abstract class ContentPart
    {
    }
}

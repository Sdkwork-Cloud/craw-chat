using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.Im.Sdk.Generated.Models
{
    public class OpenApiUserBlockResponse
    {
        public UserBlock UserBlock { get; set; }
        public CommitEnvelopeResponse LatestCommit { get; set; }
        public SocialWritePersistence Persistence { get; set; }
    }
}

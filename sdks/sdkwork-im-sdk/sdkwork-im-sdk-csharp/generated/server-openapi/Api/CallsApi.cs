using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.Sdk.Generated.Models;
using SdkHttpClient = Sdkwork.Im.Sdk.Generated.Http.HttpClient;

namespace Sdkwork.Im.Sdk.Generated.Api
{
    public class CallsApi
    {
        private readonly SdkHttpClient _client;

        public CallsApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Create an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse?> SessionsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse>(ApiPaths.ImPath("/calls/sessions"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve IM call signaling session state
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcSession?> SessionsRetrieveAsync(string rtcSessionId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.RtcSession>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}"));
        }

        /// <summary>
        /// Invite participants into an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse?> SessionsInviteAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.InviteRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/invite"), body, null, null, "application/json");
        }

        /// <summary>
        /// Accept an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse?> SessionsAcceptAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/accept"), body, null, null, "application/json");
        }

        /// <summary>
        /// Reject an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse?> SessionsRejectAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/reject"), body, null, null, "application/json");
        }

        /// <summary>
        /// End an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse?> SessionsEndAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RtcSessionMutationResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/end"), body, null, null, "application/json");
        }

        /// <summary>
        /// Post an IM call signaling event
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcSignalEvent?> SessionsSignalsCreateAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.PostRtcSignalRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RtcSignalEvent>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/signals"), body, null, null, "application/json");
        }

        /// <summary>
        /// Issue an RTC media participant credential for an IM call
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RtcParticipantCredential?> SessionsCredentialsCreateAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.IssueRtcParticipantCredentialRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RtcParticipantCredential>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/credentials"), body, null, null, "application/json");
        }

        private sealed record PathParameterSpec(string Name, string Style, bool Explode);

        private static string SerializePathParameter(object? value, PathParameterSpec spec)
        {
            if (value is null)
            {
                return string.Empty;
            }
            var style = string.IsNullOrWhiteSpace(spec.Style) ? "simple" : spec.Style;
            if (value is System.Collections.IDictionary dictionary)
            {
                return SerializePathObject(spec.Name, dictionary, style, spec.Explode);
            }
            if (value is System.Collections.IEnumerable enumerable && value is not string)
            {
                return SerializePathArray(spec.Name, enumerable, style, spec.Explode);
            }
            return PathPrimitivePrefix(spec.Name, style) + Uri.EscapeDataString(value.ToString() ?? string.Empty);
        }

        private static string SerializePathArray(string name, System.Collections.IEnumerable values, string style, bool explode)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(Uri.EscapeDataString(item.ToString() ?? string.Empty));
                }
            }
            if (serialized.Count == 0)
            {
                return PathPrefix(name, style);
            }
            if (style == "matrix")
            {
                if (explode)
                {
                    var parts = new List<string>();
                    foreach (var item in serialized)
                    {
                        parts.Add(";" + name + "=" + item);
                    }
                    return string.Join(string.Empty, parts);
                }
                return ";" + name + "=" + string.Join(",", serialized);
            }
            var separator = explode ? "." : ",";
            return PathPrefix(name, style) + string.Join(separator, serialized);
        }

        private static string SerializePathObject(string name, System.Collections.IDictionary values, string style, bool explode)
        {
            var entries = new List<string>();
            var exploded = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                var escapedKey = Uri.EscapeDataString(item.Key.ToString() ?? string.Empty);
                var escapedValue = Uri.EscapeDataString(item.Value.ToString() ?? string.Empty);
                if (explode)
                {
                    exploded.Add(style == "matrix" ? ";" + escapedKey + "=" + escapedValue : escapedKey + "=" + escapedValue);
                }
                else
                {
                    entries.Add(escapedKey);
                    entries.Add(escapedValue);
                }
            }
            if (style == "matrix")
            {
                return explode ? string.Join(string.Empty, exploded) : ";" + name + "=" + string.Join(",", entries);
            }
            if (explode)
            {
                var separator = style == "label" ? "." : ",";
                return PathPrefix(name, style) + string.Join(separator, exploded);
            }
            return PathPrefix(name, style) + string.Join(",", entries);
        }

        private static string PathPrefix(string name, string style)
        {
            return style switch
            {
                "label" => ".",
                "matrix" => ";" + name,
                _ => string.Empty,
            };
        }

        private static string PathPrimitivePrefix(string name, string style)
        {
            return style == "matrix" ? ";" + name + "=" : PathPrefix(name, style);
        }


    }
}

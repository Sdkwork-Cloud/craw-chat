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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsCreateResponse201?> SessionsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsCreateResponse201>(ApiPaths.ImPath("/calls/sessions"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve IM call signaling session state
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsRetrieveResponse?> SessionsRetrieveAsync(string rtcSessionId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsRetrieveResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}"));
        }

        /// <summary>
        /// Invite participants into an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsInviteResponse?> SessionsInviteAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.InviteRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsInviteResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/invite"), body, null, null, "application/json");
        }

        /// <summary>
        /// Accept an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsAcceptResponse?> SessionsAcceptAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsAcceptResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/accept"), body, null, null, "application/json");
        }

        /// <summary>
        /// Reject an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsRejectResponse?> SessionsRejectAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsRejectResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/reject"), body, null, null, "application/json");
        }

        /// <summary>
        /// End an IM call signaling session
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsEndResponse?> SessionsEndAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsEndResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/end"), body, null, null, "application/json");
        }

        /// <summary>
        /// List IM call signaling events
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsSignalsListResponse?> SessionsSignalsListAsync(string rtcSessionId, int? afterSignalSeq = null, string? cursor = null, int? pageSize = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("afterSignalSeq", afterSignalSeq, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsSignalsListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/signals"), queryString));
        }

        /// <summary>
        /// Post an IM call signaling event
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsSignalsCreateResponse201?> SessionsSignalsCreateAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.PostRtcSignalRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsSignalsCreateResponse201>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/signals"), body, null, null, "application/json");
        }

        /// <summary>
        /// Issue an RTC media participant credential for an IM call
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsCredentialsCreateResponse201?> SessionsCredentialsCreateAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.IssueRtcParticipantCredentialRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsCredentialsCreateResponse201>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/credentials"), body, null, null, "application/json");
        }

        /// <summary>
        /// Refresh an expiring RTC media participant credential
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsCredentialsRefreshResponse?> SessionsCredentialsRefreshAsync(string rtcSessionId, Sdkwork.Im.Sdk.Generated.Models.IssueRtcParticipantCredentialRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CallsSessionsCredentialsRefreshResponse>(ApiPaths.ImPath($"/calls/sessions/{SerializePathParameter(rtcSessionId, new PathParameterSpec("rtcSessionId", "simple", false))}/credentials/refresh"), body, null, null, "application/json");
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

        private sealed record QueryParameterSpec(
            string Name,
            object? Value,
            string Style,
            bool Explode,
            bool AllowReserved,
            string? ContentType);

        private static string BuildQueryString(IEnumerable<QueryParameterSpec> parameters)
        {
            var pairs = new List<string>();
            foreach (var parameter in parameters)
            {
                AppendSerializedParameter(pairs, parameter);
            }
            return string.Join("&", pairs);
        }

        private static void AppendSerializedParameter(List<string> pairs, QueryParameterSpec parameter)
        {
            if (parameter.Value is null)
            {
                return;
            }

            if (!string.IsNullOrWhiteSpace(parameter.ContentType))
            {
                var json = System.Text.Json.JsonSerializer.Serialize(parameter.Value);
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(json, parameter.AllowReserved));
                return;
            }

            var style = string.IsNullOrWhiteSpace(parameter.Style) ? "form" : parameter.Style;
            if (style == "deepObject" && parameter.Value is System.Collections.IDictionary deepObject)
            {
                AppendDeepObjectParameter(pairs, parameter.Name, deepObject, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IEnumerable enumerable && parameter.Value is not string && parameter.Value is not System.Collections.IDictionary)
            {
                AppendArrayParameter(pairs, parameter.Name, enumerable, style, parameter.Explode, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IDictionary dictionary)
            {
                AppendObjectParameter(pairs, parameter.Name, dictionary, style, parameter.Explode, parameter.AllowReserved);
            }
            else
            {
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(parameter.Value.ToString() ?? string.Empty, parameter.AllowReserved));
            }
        }

        private static void AppendArrayParameter(List<string> pairs, string name, System.Collections.IEnumerable values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(item.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count == 0)
            {
                return;
            }
            if (style == "form" && explode)
            {
                foreach (var item in serialized)
                {
                    pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(item, allowReserved));
                }
                return;
            }
            pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
        }

        private static void AppendObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                if (style == "form" && explode)
                {
                    pairs.Add(Uri.EscapeDataString(item.Key.ToString() ?? string.Empty) + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
                else
                {
                    serialized.Add(item.Key.ToString() ?? string.Empty);
                    serialized.Add(item.Value.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count > 0)
            {
                pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
            }
        }

        private static void AppendDeepObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, bool allowReserved)
        {
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is not null)
                {
                    pairs.Add(Uri.EscapeDataString(name + "[" + item.Key + "]") + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
            }
        }

        private static string EncodeQueryValue(string value, bool allowReserved)
        {
            var encoded = Uri.EscapeDataString(value);
            if (!allowReserved)
            {
                return encoded;
            }
            return encoded
                .Replace("%3A", ":").Replace("%2F", "/").Replace("%3F", "?").Replace("%23", "#")
                .Replace("%5B", "[").Replace("%5D", "]").Replace("%40", "@").Replace("%21", "!")
                .Replace("%24", "$").Replace("%26", "&").Replace("%27", "'").Replace("%28", "(")
                .Replace("%29", ")").Replace("%2A", "*").Replace("%2B", "+").Replace("%2C", ",")
                .Replace("%3B", ";").Replace("%3D", "=");
        }

    }
}

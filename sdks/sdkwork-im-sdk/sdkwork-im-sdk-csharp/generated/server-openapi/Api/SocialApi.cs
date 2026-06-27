using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.Sdk.Generated.Models;
using SdkHttpClient = Sdkwork.Im.Sdk.Generated.Http.HttpClient;

namespace Sdkwork.Im.Sdk.Generated.Api
{
    public class SocialApi
    {
        private readonly SdkHttpClient _client;

        public SocialApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Search social users
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SocialUserSearchResponse?> UsersListAsync(string? q = null, int? limit = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("q", q, "form", true, false, null),
                new QueryParameterSpec("limit", limit, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SocialUserSearchResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/social/users"), queryString));
        }

        /// <summary>
        /// List friend requests
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestListResponse?> FriendRequestsListAsync(string? direction = null, string? status = null, int? limit = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("direction", direction, "form", true, false, null),
                new QueryParameterSpec("status", status, "form", true, false, null),
                new QueryParameterSpec("limit", limit, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/social/friend_requests"), queryString));
        }

        /// <summary>
        /// Create a friend request
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestMutationResponse?> FriendRequestsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.SubmitFriendRequestRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestMutationResponse>(ApiPaths.ImPath("/social/friend_requests"), body, null, null, "application/json");
        }

        /// <summary>
        /// Accept a friend request
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestAcceptanceResponse?> FriendRequestsAcceptAsync(string requestId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestAcceptanceResponse>(ApiPaths.ImPath($"/social/friend_requests/{SerializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false))}/accept"), null);
        }

        /// <summary>
        /// Decline a friend request
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestMutationResponse?> FriendRequestsDeclineAsync(string requestId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestMutationResponse>(ApiPaths.ImPath($"/social/friend_requests/{SerializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false))}/decline"), null);
        }

        /// <summary>
        /// Cancel a friend request
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestMutationResponse?> FriendRequestsCancelAsync(string requestId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SocialFriendRequestMutationResponse>(ApiPaths.ImPath($"/social/friend_requests/{SerializePathParameter(requestId, new PathParameterSpec("requestId", "simple", false))}/cancel"), null);
        }

        /// <summary>
        /// Remove a friendship
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.SocialFriendshipMutationResponse?> FriendshipsRemoveAsync(string friendshipId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.SocialFriendshipMutationResponse>(ApiPaths.ImPath($"/social/friendships/{SerializePathParameter(friendshipId, new PathParameterSpec("friendshipId", "simple", false))}/remove"), null);
        }

        /// <summary>
        /// List contact tags
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactTagsResponse?> ContactsTagsListAsync(int? limit = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("limit", limit, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ContactTagsResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/social/contacts/tags"), queryString));
        }

        /// <summary>
        /// Create a contact tag
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactTagView?> ContactsTagsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateContactTagRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ContactTagView>(ApiPaths.ImPath("/social/contacts/tags"), body, null, null, "application/json");
        }

        /// <summary>
        /// Update a contact tag
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactTagView?> ContactsTagsUpdateAsync(string tagId, Sdkwork.Im.Sdk.Generated.Models.UpdateContactTagRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.ContactTagView>(ApiPaths.ImPath($"/social/contacts/tags/{SerializePathParameter(tagId, new PathParameterSpec("tagId", "simple", false))}"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete a contact tag
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.DeleteContactTagResponse?> ContactsTagsDeleteAsync(string tagId)
        {
            return await _client.DeleteAsync<Sdkwork.Im.Sdk.Generated.Models.DeleteContactTagResponse>(ApiPaths.ImPath($"/social/contacts/tags/{SerializePathParameter(tagId, new PathParameterSpec("tagId", "simple", false))}"));
        }

        /// <summary>
        /// Create a contact recommendation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactRecommendationView?> ContactsRecommendationsCreateAsync(string targetUserId, Sdkwork.Im.Sdk.Generated.Models.CreateContactRecommendationRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ContactRecommendationView>(ApiPaths.ImPath($"/social/contacts/{SerializePathParameter(targetUserId, new PathParameterSpec("targetUserId", "simple", false))}/recommendations"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve contact preferences
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactPreferencesView?> ContactsPreferencesRetrieveAsync(string targetUserId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ContactPreferencesView>(ApiPaths.ImPath($"/social/contacts/{SerializePathParameter(targetUserId, new PathParameterSpec("targetUserId", "simple", false))}/preferences"));
        }

        /// <summary>
        /// Update contact preferences
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactPreferencesView?> ContactsPreferencesUpdateAsync(string targetUserId, Sdkwork.Im.Sdk.Generated.Models.UpdateContactPreferencesRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.ContactPreferencesView>(ApiPaths.ImPath($"/social/contacts/{SerializePathParameter(targetUserId, new PathParameterSpec("targetUserId", "simple", false))}/preferences"), body, null, null, "application/json");
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

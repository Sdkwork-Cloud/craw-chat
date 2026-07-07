import Foundation

public class SpacesApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Create a space
    public func create(body: SpaceCreateRequest) async throws -> SpacesCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesCreateResponse201.self)
    }

    /// List spaces
    public func list(pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces"), query), responseType: SpacesListResponse.self)
    }

    /// Retrieve a space
    public func retrieve(spaceId: String) async throws -> SpacesRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))"), responseType: SpacesRetrieveResponse.self)
    }

    /// Update a space
    public func update(spaceId: String, body: SpaceUpdateRequest) async throws -> SpacesUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesUpdateResponse.self)
    }

    /// Delete a space
    public func delete(spaceId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))"))
    }

    /// List spaces members
    public func membersList(spaceId: String, pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesMembersListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members"), query), responseType: SpacesMembersListResponse.self)
    }

    /// Create spaces members
    public func membersCreate(spaceId: String, body: SpaceMemberCreateRequest) async throws -> SpacesMembersCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesMembersCreateResponse201.self)
    }

    /// retrieve spaces members
    public func membersRetrieve(spaceId: String, userId: String) async throws -> SpacesMembersRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), responseType: SpacesMembersRetrieveResponse.self)
    }

    /// Update spaces members
    public func membersUpdate(spaceId: String, userId: String, body: SpaceMemberUpdateRequest) async throws -> SpacesMembersUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesMembersUpdateResponse.self)
    }

    /// Delete spaces members
    public func membersDelete(spaceId: String, userId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"))
    }

    /// List spaces groups
    public func groupsList(spaceId: String, pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesGroupsListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups"), query), responseType: SpacesGroupsListResponse.self)
    }

    /// Create spaces groups
    public func groupsCreate(spaceId: String, body: SpaceGroupCreateRequest) async throws -> SpacesGroupsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesGroupsCreateResponse201.self)
    }

    /// retrieve spaces groups
    public func groupsRetrieve(spaceId: String, groupId: String) async throws -> SpacesGroupsRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"), responseType: SpacesGroupsRetrieveResponse.self)
    }

    /// Update spaces groups
    public func groupsUpdate(spaceId: String, groupId: String, body: SpaceGroupUpdateRequest) async throws -> SpacesGroupsUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesGroupsUpdateResponse.self)
    }

    /// Delete spaces groups
    public func groupsDelete(spaceId: String, groupId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))"))
    }

    /// List spaces groups members
    public func groupsMembersList(spaceId: String, groupId: String, pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesGroupsMembersListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members"), query), responseType: SpacesGroupsMembersListResponse.self)
    }

    /// Create spaces groups members
    public func groupsMembersCreate(spaceId: String, groupId: String, body: SpaceGroupMemberCreateRequest) async throws -> SpacesGroupsMembersCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesGroupsMembersCreateResponse201.self)
    }

    /// retrieve spaces groups members
    public func groupsMembersRetrieve(spaceId: String, groupId: String, userId: String) async throws -> SpacesGroupsMembersRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), responseType: SpacesGroupsMembersRetrieveResponse.self)
    }

    /// Update spaces groups members
    public func groupsMembersUpdate(spaceId: String, groupId: String, userId: String, body: SpaceGroupMemberUpdateRequest) async throws -> SpacesGroupsMembersUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesGroupsMembersUpdateResponse.self)
    }

    /// Delete spaces groups members
    public func groupsMembersDelete(spaceId: String, groupId: String, userId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/groups/\(serializePathParameter(groupId, PathParameterSpec(name: "groupId", style: "simple", explode: false)))/members/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"))
    }

    /// List spaces channels
    public func channelsList(spaceId: String, pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesChannelsListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels"), query), responseType: SpacesChannelsListResponse.self)
    }

    /// Create spaces channels
    public func channelsCreate(spaceId: String, body: SpaceChannelCreateRequest) async throws -> SpacesChannelsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesChannelsCreateResponse201.self)
    }

    /// retrieve spaces channels
    public func channelsRetrieve(spaceId: String, channelId: String) async throws -> SpacesChannelsRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"), responseType: SpacesChannelsRetrieveResponse.self)
    }

    /// Update spaces channels
    public func channelsUpdate(spaceId: String, channelId: String, body: SpaceChannelUpdateRequest) async throws -> SpacesChannelsUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesChannelsUpdateResponse.self)
    }

    /// Delete spaces channels
    public func channelsDelete(spaceId: String, channelId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))"))
    }

    /// List spaces channels access Rules
    public func channelsAccessRulesList(spaceId: String, channelId: String, pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesChannelsAccessRulesListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/access_rules"), query), responseType: SpacesChannelsAccessRulesListResponse.self)
    }

    /// Create spaces channels access Rules
    public func channelsAccessRulesCreate(spaceId: String, channelId: String, body: SpaceChannelAccessRuleCreateRequest) async throws -> SpacesChannelsAccessRulesCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/access_rules"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesChannelsAccessRulesCreateResponse201.self)
    }

    /// Delete spaces channels access Rules
    public func channelsAccessRulesDelete(spaceId: String, channelId: String, ruleId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/channels/\(serializePathParameter(channelId, PathParameterSpec(name: "channelId", style: "simple", explode: false)))/access_rules/\(serializePathParameter(ruleId, PathParameterSpec(name: "ruleId", style: "simple", explode: false)))"))
    }

    /// List spaces invites
    public func invitesList(spaceId: String, status: String? = nil, pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesInvitesListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "status", value: status, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites"), query), responseType: SpacesInvitesListResponse.self)
    }

    /// Create spaces invites
    public func invitesCreate(spaceId: String, body: SpaceInviteCreateRequest) async throws -> SpacesInvitesCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesInvitesCreateResponse201.self)
    }

    /// retrieve spaces invites
    public func invitesRetrieve(spaceId: String, inviteCode: String) async throws -> SpacesInvitesRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites/\(serializePathParameter(inviteCode, PathParameterSpec(name: "inviteCode", style: "simple", explode: false)))"), responseType: SpacesInvitesRetrieveResponse.self)
    }

    /// Delete spaces invites
    public func invitesDelete(spaceId: String, inviteCode: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites/\(serializePathParameter(inviteCode, PathParameterSpec(name: "inviteCode", style: "simple", explode: false)))"))
    }

    /// Accept spaces invites
    public func invitesAccept(spaceId: String, inviteCode: String) async throws -> SdkWorkCommandResponse? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/invites/\(serializePathParameter(inviteCode, PathParameterSpec(name: "inviteCode", style: "simple", explode: false)))/accept"), body: nil, responseType: SdkWorkCommandResponse.self)
    }

    /// List spaces bans
    public func bansList(spaceId: String, pageSize: Int? = nil, cursor: String? = nil) async throws -> SpacesBansListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans"), query), responseType: SpacesBansListResponse.self)
    }

    /// Create spaces bans
    public func bansCreate(spaceId: String, body: SpaceBanCreateRequest) async throws -> SpacesBansCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: SpacesBansCreateResponse201.self)
    }

    /// retrieve spaces bans
    public func bansRetrieve(spaceId: String, userId: String) async throws -> SpacesBansRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"), responseType: SpacesBansRetrieveResponse.self)
    }

    /// Delete spaces bans
    public func bansDelete(spaceId: String, userId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/spaces/\(serializePathParameter(spaceId, PathParameterSpec(name: "spaceId", style: "simple", explode: false)))/bans/\(serializePathParameter(userId, PathParameterSpec(name: "userId", style: "simple", explode: false)))"))
    }

    private struct PathParameterSpec {
        let name: String
        let style: String
        let explode: Bool
    }

    private func serializePathParameter(_ value: Any?, _ spec: PathParameterSpec) -> String {
        guard let value else { return "" }
        let style = spec.style.isEmpty ? "simple" : spec.style
        if let array = value as? [Any] {
            return serializePathArray(spec.name, array, style, spec.explode)
        }
        if let object = value as? [String: Any] {
            return serializePathObject(spec.name, object, style, spec.explode)
        }
        return pathPrimitivePrefix(spec.name, style) + pathEncode(String(describing: value))
    }

    private func serializePathArray(_ name: String, _ values: [Any], _ style: String, _ explode: Bool) -> String {
        let serialized = values.map { pathEncode(String(describing: $0)) }
        if serialized.isEmpty { return pathPrefix(name, style) }
        if style == "matrix" {
            if explode {
                return serialized.map { ";\(name)=\($0)" }.joined()
            }
            return ";\(name)=" + serialized.joined(separator: ",")
        }
        let separator = explode ? "." : ","
        return pathPrefix(name, style) + serialized.joined(separator: separator)
    }

    private func serializePathObject(_ name: String, _ values: [String: Any], _ style: String, _ explode: Bool) -> String {
        var entries: [String] = []
        var exploded: [String] = []
        for (key, value) in values {
            let escapedKey = pathEncode(key)
            let escapedValue = pathEncode(String(describing: value))
            if explode {
                if style == "matrix" {
                    exploded.append(";\(escapedKey)=\(escapedValue)")
                } else {
                    exploded.append("\(escapedKey)=\(escapedValue)")
                }
            } else {
                entries.append(escapedKey)
                entries.append(escapedValue)
            }
        }
        if style == "matrix" {
            if explode {
                return exploded.joined()
            }
            return ";\(name)=" + entries.joined(separator: ",")
        }
        if explode {
            let separator = style == "label" ? "." : ","
            return pathPrefix(name, style) + exploded.joined(separator: separator)
        }
        return pathPrefix(name, style) + entries.joined(separator: ",")
    }

    private func pathPrefix(_ name: String, _ style: String) -> String {
        if style == "label" { return "." }
        if style == "matrix" { return ";\(name)" }
        return ""
    }

    private func pathPrimitivePrefix(_ name: String, _ style: String) -> String {
        style == "matrix" ? ";\(name)=" : pathPrefix(name, style)
    }

    private func pathEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? value
    }

    private struct QueryParameterSpec {
        let name: String
        let value: Any?
        let style: String
        let explode: Bool
        let allowReserved: Bool
        let contentType: String?
    }

    private func buildQueryString(_ parameters: [QueryParameterSpec]) -> String {
        var pairs: [String] = []
        for parameter in parameters {
            appendSerializedParameter(&pairs, parameter)
        }
        return pairs.joined(separator: "&")
    }

    private func appendSerializedParameter(_ pairs: inout [String], _ parameter: QueryParameterSpec) {
        guard let value = parameter.value else { return }
        if let contentType = parameter.contentType, !contentType.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            let data = (try? JSONSerialization.data(withJSONObject: value, options: [])) ?? Data(String(describing: value).utf8)
            let json = String(data: data, encoding: .utf8) ?? String(describing: value)
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(json, allowReserved: parameter.allowReserved))")
            return
        }

        let style = parameter.style.isEmpty ? "form" : parameter.style
        if style == "deepObject", let object = value as? [String: Any] {
            appendDeepObjectParameter(&pairs, name: parameter.name, values: object, allowReserved: parameter.allowReserved)
        } else if let array = value as? [Any] {
            appendArrayParameter(&pairs, name: parameter.name, values: array, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else if let object = value as? [String: Any] {
            appendObjectParameter(&pairs, name: parameter.name, values: object, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else {
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(String(describing: value), allowReserved: parameter.allowReserved))")
        }
    }

    private func appendArrayParameter(
        _ pairs: inout [String],
        name: String,
        values: [Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        let serialized = values.map { String(describing: $0) }
        guard !serialized.isEmpty else { return }
        if style == "form" && explode {
            for item in serialized {
                pairs.append("\(urlEncode(name))=\(encodeQueryValue(item, allowReserved: allowReserved))")
            }
            return
        }
        pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
    }

    private func appendObjectParameter(
        _ pairs: inout [String],
        name: String,
        values: [String: Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        var serialized: [String] = []
        for (key, value) in values {
            if style == "form" && explode {
                pairs.append("\(urlEncode(key))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
            } else {
                serialized.append(key)
                serialized.append(String(describing: value))
            }
        }
        if !serialized.isEmpty {
            pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
        }
    }

    private func appendDeepObjectParameter(_ pairs: inout [String], name: String, values: [String: Any], allowReserved: Bool) {
        for (key, value) in values {
            pairs.append("\(urlEncode("\(name)[\(key)]"))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
        }
    }

    private func encodeQueryValue(_ value: String, allowReserved: Bool) -> String {
        var encoded = urlEncode(value)
        if !allowReserved { return encoded }
        [
            "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
            "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
            "%24": "$", "%26": "&", "%27": "'", "%28": "(",
            "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
            "%3B": ";", "%3D": "=",
        ].forEach { encoded = encoded.replacingOccurrences(of: $0.key, with: $0.value) }
        return encoded
    }

    private func urlEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? value
    }

}

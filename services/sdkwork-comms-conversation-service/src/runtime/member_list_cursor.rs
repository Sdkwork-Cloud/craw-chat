use std::fmt;

use im_platform_contracts::ConversationMemberPageCursor;
use serde::{Deserialize, Serialize};

use super::cursor_signing::{SignedCursorError, decode_signed_cursor, encode_signed_cursor};
use super::message_history_cursor::{
    MessageHistoryCursorError, resolve_message_history_cursor_secret,
};

const MEMBER_LIST_CURSOR_TYPE: &str = "sdkwork-im-member-list-cursor";
const MEMBER_LIST_CURSOR_VERSION: u8 = 1;
const MEMBER_LIST_CURSOR_SORT: &str = "principal_kind_asc_principal_id_asc";

#[derive(Clone, Copy, Debug)]
pub(super) struct MemberListCursorScope<'a> {
    pub tenant_id: &'a str,
    pub organization_id: &'a str,
    pub conversation_id: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum MemberListCursorError {
    Invalid,
    Configuration(String),
}

impl fmt::Display for MemberListCursorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => formatter.write_str("conversation member list cursor is invalid"),
            Self::Configuration(message) => formatter.write_str(message),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct MemberListCursorPayload {
    #[serde(rename = "c")]
    conversation_id: String,
    #[serde(rename = "k")]
    principal_kind: String,
    #[serde(rename = "o")]
    organization_id: String,
    #[serde(rename = "p")]
    principal_id: String,
    #[serde(rename = "s")]
    sort: String,
    #[serde(rename = "t")]
    tenant_id: String,
    #[serde(rename = "v")]
    version: u8,
}

pub(super) fn encode_member_list_cursor(
    scope: MemberListCursorScope<'_>,
    cursor: &ConversationMemberPageCursor,
) -> Result<String, MemberListCursorError> {
    let payload = MemberListCursorPayload {
        conversation_id: scope.conversation_id.to_owned(),
        principal_kind: cursor.principal_kind.clone(),
        organization_id: scope.organization_id.to_owned(),
        principal_id: cursor.principal_id.clone(),
        sort: MEMBER_LIST_CURSOR_SORT.to_owned(),
        tenant_id: scope.tenant_id.to_owned(),
        version: MEMBER_LIST_CURSOR_VERSION,
    };
    encode_signed_cursor(
        MEMBER_LIST_CURSOR_TYPE,
        MEMBER_LIST_CURSOR_VERSION,
        &payload,
        resolve_message_history_cursor_secret().map_err(map_secret_error)?,
    )
    .map_err(|error| match error {
        SignedCursorError::Invalid => MemberListCursorError::Invalid,
        SignedCursorError::Serialization(message) => MemberListCursorError::Configuration(format!(
            "conversation member list cursor serialization failed: {message}"
        )),
    })
}

pub(super) fn decode_member_list_cursor(
    cursor: &str,
    scope: MemberListCursorScope<'_>,
) -> Result<ConversationMemberPageCursor, MemberListCursorError> {
    let payload: MemberListCursorPayload = decode_signed_cursor(
        cursor,
        MEMBER_LIST_CURSOR_TYPE,
        MEMBER_LIST_CURSOR_VERSION,
        resolve_message_history_cursor_secret().map_err(map_secret_error)?,
    )
    .map_err(|error| match error {
        SignedCursorError::Invalid => MemberListCursorError::Invalid,
        SignedCursorError::Serialization(message) => MemberListCursorError::Configuration(message),
    })?;
    if payload.version != MEMBER_LIST_CURSOR_VERSION
        || payload.sort != MEMBER_LIST_CURSOR_SORT
        || payload.tenant_id != scope.tenant_id
        || payload.organization_id != scope.organization_id
        || payload.conversation_id != scope.conversation_id
        || payload.principal_kind.is_empty()
        || payload.principal_id.is_empty()
    {
        return Err(MemberListCursorError::Invalid);
    }
    Ok(ConversationMemberPageCursor {
        principal_kind: payload.principal_kind,
        principal_id: payload.principal_id,
    })
}

fn map_secret_error(error: MessageHistoryCursorError) -> MemberListCursorError {
    match error {
        MessageHistoryCursorError::Invalid => MemberListCursorError::Invalid,
        MessageHistoryCursorError::Configuration(message) => {
            MemberListCursorError::Configuration(message)
        }
    }
}

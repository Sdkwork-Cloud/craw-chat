//! Canonical direct-chat business and conversation identifiers shared across services.

use crate::social::normalize_actor_pair;
use sdkwork_utils_rust::sha256_hash;

const CANONICAL_CONVERSATION_ID_DIGEST_LEN: usize = 24;

fn encode_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

pub fn normalize_commit_organization_id(organization_id: &str) -> String {
    let trimmed = organization_id.trim();
    if trimmed.is_empty() || trimmed == "0" || trimmed == "default" {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

pub fn deterministic_conversation_resource_id(prefix: &str, seed: &str) -> String {
    let digest = sha256_hash(seed.as_bytes());
    format!(
        "{prefix}{}",
        &digest[..CANONICAL_CONVERSATION_ID_DIGEST_LEN]
    )
}

pub fn canonical_direct_chat_business_id(
    left_actor_kind: &str,
    left_actor_id: &str,
    right_actor_kind: &str,
    right_actor_id: &str,
) -> Result<String, String> {
    let pair =
        normalize_actor_pair(left_actor_id, right_actor_id).map_err(|error| error.to_string())?;
    let (left_kind, right_kind) = if pair.left_actor_id == left_actor_id {
        (left_actor_kind, right_actor_kind)
    } else {
        (right_actor_kind, left_actor_kind)
    };
    Ok(encode_key_segments([
        left_kind,
        pair.left_actor_id.as_str(),
        right_kind,
        pair.right_actor_id.as_str(),
    ]))
}

pub fn canonical_direct_chat_conversation_id(
    tenant_id: &str,
    organization_id: &str,
    direct_chat_business_id: &str,
) -> String {
    let seed = encode_key_segments([
        tenant_id,
        normalize_commit_organization_id(organization_id).as_str(),
        "direct_chat",
        direct_chat_business_id,
    ]);
    deterministic_conversation_resource_id("c_direct_", seed.as_str())
}

pub fn resolve_direct_chat_binding_ids(
    tenant_id: &str,
    organization_id: &str,
    left_actor_kind: &str,
    left_actor_id: &str,
    right_actor_kind: &str,
    right_actor_id: &str,
    requested_conversation_id: &str,
    requested_direct_chat_id: &str,
) -> Result<(String, String), String> {
    let direct_chat_id = canonical_direct_chat_business_id(
        left_actor_kind,
        left_actor_id,
        right_actor_kind,
        right_actor_id,
    )?;
    let conversation_id =
        canonical_direct_chat_conversation_id(tenant_id, organization_id, direct_chat_id.as_str());
    let requested_conversation_id = requested_conversation_id.trim();
    let requested_direct_chat_id = requested_direct_chat_id.trim();
    if !requested_direct_chat_id.is_empty() && requested_direct_chat_id != direct_chat_id {
        return Err(format!(
            "directChatId must be omitted or match the canonical direct chat id; expected {direct_chat_id}"
        ));
    }
    if !requested_conversation_id.is_empty() && requested_conversation_id != conversation_id {
        return Err(format!(
            "conversationId must be omitted or match the canonical direct chat conversation id; expected {conversation_id}"
        ));
    }
    Ok((conversation_id, direct_chat_id))
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_direct_chat_business_id, canonical_direct_chat_conversation_id,
        resolve_direct_chat_binding_ids,
    };

    #[test]
    fn canonical_direct_chat_ids_are_stable_for_actor_pair() {
        let business_a = canonical_direct_chat_business_id("user", "u_alice", "user", "u_bob")
            .expect("business id");
        let business_b = canonical_direct_chat_business_id("user", "u_bob", "user", "u_alice")
            .expect("business id");
        assert_eq!(business_a, business_b);

        let conversation = canonical_direct_chat_conversation_id("t1", "0", business_a.as_str());
        assert!(conversation.starts_with("c_direct_"));
    }

    #[test]
    fn resolve_direct_chat_binding_ids_accepts_empty_requested_ids() {
        let (conversation_id, direct_chat_id) =
            resolve_direct_chat_binding_ids("t1", "0", "user", "u_alice", "user", "u_bob", "", "")
                .expect("resolved ids");
        assert!(!conversation_id.is_empty());
        assert!(!direct_chat_id.is_empty());
    }
}

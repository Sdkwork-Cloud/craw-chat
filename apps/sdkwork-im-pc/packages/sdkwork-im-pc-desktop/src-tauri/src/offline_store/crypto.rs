use aes_gcm::aead::{
    rand_core::{OsRng, RngCore},
    Aead, KeyInit, Payload,
};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use keyring::Entry;
use serde_json::Value;
use sha2::{Digest, Sha256};

const CIPHERTEXT_PREFIX: &str = "enc-v1:";
const KEYRING_SERVICE: &str = "com.sdkwork.im-pc.client-local";
const KEY_BYTES: usize = 32;
const NONCE_BYTES: usize = 12;

#[derive(Clone)]
pub(super) struct PayloadCipher {
    key: [u8; KEY_BYTES],
    scope_fingerprint: String,
}

impl PayloadCipher {
    pub(super) fn from_keyring(
        scope_fingerprint: &str,
        allow_key_creation: bool,
    ) -> Result<Self, String> {
        let entry = Entry::new(KEYRING_SERVICE, scope_fingerprint)
            .map_err(|error| format!("open client-local keyring entry failed: {error}"))?;
        let key = match entry.get_password() {
            Ok(encoded) => decode_key(encoded.as_str())?,
            Err(keyring::Error::NoEntry) if allow_key_creation => {
                let mut key = [0_u8; KEY_BYTES];
                OsRng.fill_bytes(&mut key);
                entry
                    .set_password(URL_SAFE_NO_PAD.encode(key).as_str())
                    .map_err(|error| {
                        format!("persist client-local encryption key failed: {error}")
                })?;
                key
            }
            Err(keyring::Error::NoEntry) => {
                return Err(
                    "client-local encryption key is missing for persisted ciphertext; refusing to replace it"
                        .to_owned(),
                )
            }
            Err(error) => {
                return Err(format!(
                    "read client-local encryption key failed closed: {error}"
                ))
            }
        };
        Ok(Self {
            key,
            scope_fingerprint: scope_fingerprint.to_owned(),
        })
    }

    #[cfg(test)]
    pub(super) fn for_test(scope_fingerprint: &str) -> Self {
        Self::for_test_key(scope_fingerprint, [0x5a; KEY_BYTES])
    }

    #[cfg(test)]
    fn for_test_key(scope_fingerprint: &str, key: [u8; KEY_BYTES]) -> Self {
        Self {
            key,
            scope_fingerprint: scope_fingerprint.to_owned(),
        }
    }

    pub(super) fn encrypt_json(
        &self,
        purpose: &str,
        record_key: &str,
        plaintext: &str,
    ) -> Result<String, String> {
        validate_json_without_credentials(plaintext)?;
        let mut nonce_bytes = [0_u8; NONCE_BYTES];
        OsRng.fill_bytes(&mut nonce_bytes);
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|_| "initialize client-local payload cipher failed".to_owned())?;
        let aad = self.associated_data(purpose, record_key);
        let ciphertext = cipher
            .encrypt(
                Nonce::from_slice(&nonce_bytes),
                Payload {
                    msg: plaintext.as_bytes(),
                    aad: aad.as_bytes(),
                },
            )
            .map_err(|_| "encrypt client-local payload failed".to_owned())?;
        let mut envelope = Vec::with_capacity(NONCE_BYTES + ciphertext.len());
        envelope.extend_from_slice(&nonce_bytes);
        envelope.extend_from_slice(&ciphertext);
        Ok(format!(
            "{CIPHERTEXT_PREFIX}{}",
            URL_SAFE_NO_PAD.encode(envelope)
        ))
    }

    pub(super) fn decrypt_json(
        &self,
        purpose: &str,
        record_key: &str,
        encoded: &str,
    ) -> Result<String, String> {
        let envelope = encoded
            .strip_prefix(CIPHERTEXT_PREFIX)
            .ok_or_else(|| "client-local payload has an unsupported cipher version".to_owned())?;
        let decoded = URL_SAFE_NO_PAD
            .decode(envelope)
            .map_err(|_| "decode client-local payload failed".to_owned())?;
        if decoded.len() <= NONCE_BYTES {
            return Err("client-local payload envelope is truncated".to_owned());
        }
        let (nonce, ciphertext) = decoded.split_at(NONCE_BYTES);
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|_| "initialize client-local payload cipher failed".to_owned())?;
        let aad = self.associated_data(purpose, record_key);
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(nonce),
                Payload {
                    msg: ciphertext,
                    aad: aad.as_bytes(),
                },
            )
            .map_err(|_| "decrypt client-local payload failed closed".to_owned())?;
        let plaintext = String::from_utf8(plaintext)
            .map_err(|_| "client-local payload is not valid UTF-8".to_owned())?;
        validate_json_without_credentials(plaintext.as_str())?;
        Ok(plaintext)
    }

    fn associated_data(&self, purpose: &str, record_key: &str) -> String {
        format!(
            "sdkwork-im-pc-client-local:v1:{}:{purpose}:{record_key}",
            self.scope_fingerprint
        )
    }
}

pub(super) fn payload_hash(plaintext: &str) -> String {
    format!("{:x}", Sha256::digest(plaintext.as_bytes()))
}

fn decode_key(encoded: &str) -> Result<[u8; KEY_BYTES], String> {
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| "client-local keyring entry is malformed".to_owned())?;
    decoded
        .try_into()
        .map_err(|_| "client-local keyring entry has an invalid key length".to_owned())
}

fn validate_json_without_credentials(plaintext: &str) -> Result<(), String> {
    let value = serde_json::from_str::<Value>(plaintext)
        .map_err(|_| "client-local payload must be valid JSON".to_owned())?;
    reject_credential_fields(&value)
}

fn reject_credential_fields(value: &Value) -> Result<(), String> {
    match value {
        Value::Object(object) => {
            for (key, nested) in object {
                let normalized = key
                    .chars()
                    .filter(char::is_ascii_alphanumeric)
                    .flat_map(char::to_lowercase)
                    .collect::<String>();
                if matches!(
                    normalized.as_str(),
                    "accesstoken"
                        | "authtoken"
                        | "refreshtoken"
                        | "authorization"
                        | "password"
                        | "apikey"
                        | "privatekey"
                        | "credential"
                        | "credentials"
                        | "secret"
                ) {
                    return Err(format!(
                        "client-local payload field {key} is credential-bearing and cannot be persisted"
                    ));
                }
                reject_credential_fields(nested)?;
            }
        }
        Value::Array(values) => {
            for nested in values {
                reject_credential_fields(nested)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_cipher_round_trips_and_binds_record_identity() {
        let cipher = PayloadCipher::for_test(&"a".repeat(64));
        let encrypted = cipher
            .encrypt_json("message", "conversation:1", r#"{"content":"hello"}"#)
            .expect("encrypt");
        assert!(encrypted.starts_with(CIPHERTEXT_PREFIX));
        assert!(!encrypted.contains("hello"));
        assert_eq!(
            cipher
                .decrypt_json("message", "conversation:1", encrypted.as_str())
                .expect("decrypt"),
            r#"{"content":"hello"}"#
        );
        assert!(cipher
            .decrypt_json("message", "conversation:2", encrypted.as_str())
            .is_err());
    }

    #[test]
    fn payload_cipher_rejects_credential_fields_at_any_depth() {
        let cipher = PayloadCipher::for_test(&"b".repeat(64));
        assert!(cipher
            .encrypt_json(
                "message",
                "one",
                r#"{"media":{"authorization":"Bearer forbidden"}}"#,
            )
            .is_err());
        assert!(cipher
            .encrypt_json(
                "message",
                "one",
                r#"{"parts":[{"refresh_token":"forbidden"}]}"#,
            )
            .is_err());
    }

    #[test]
    fn payload_cipher_fails_closed_for_a_different_scope_key() {
        let encrypted = PayloadCipher::for_test_key(&"c".repeat(64), [0x11; KEY_BYTES])
            .encrypt_json("message-cache", "conversation:1", r#"{"content":"hello"}"#)
            .expect("encrypt");
        assert!(
            PayloadCipher::for_test_key(&"c".repeat(64), [0x22; KEY_BYTES])
                .decrypt_json("message-cache", "conversation:1", encrypted.as_str())
                .is_err()
        );
        assert!(
            PayloadCipher::for_test_key(&"d".repeat(64), [0x11; KEY_BYTES])
                .decrypt_json("message-cache", "conversation:1", encrypted.as_str())
                .is_err()
        );
    }
}

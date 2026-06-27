use im_app_context::AppContext;

pub const MAX_WEBSOCKET_DEVICE_ID_BYTES: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebsocketDeviceBindingError {
    pub code: &'static str,
    pub message: String,
}

pub fn resolve_websocket_device_binding(
    auth: &AppContext,
    requested_device_id: Option<String>,
) -> Result<String, WebsocketDeviceBindingError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            validate_device_id(requested.as_str())?;
            validate_device_id(bound.as_str())?;
            if requested != bound {
                return Err(WebsocketDeviceBindingError {
                    code: "device_id_mismatch",
                    message: format!("device id does not match auth context: {requested}"),
                });
            }
            Ok(requested)
        }
        (Some(requested), None) => {
            validate_device_id(requested.as_str())?;
            Ok(requested)
        }
        (None, Some(bound)) => {
            validate_device_id(bound.as_str())?;
            Ok(bound)
        }
        (None, None) => Err(WebsocketDeviceBindingError {
            code: "device_id_missing",
            message: "device id must be provided by auth context or request body".to_owned(),
        }),
    }
}

fn validate_device_id(device_id: &str) -> Result<(), WebsocketDeviceBindingError> {
    if device_id.len() > MAX_WEBSOCKET_DEVICE_ID_BYTES {
        return Err(WebsocketDeviceBindingError {
            code: "device_id_too_large",
            message: format!(
                "device id exceeds maximum size of {} bytes",
                MAX_WEBSOCKET_DEVICE_ID_BYTES
            ),
        });
    }
    if device_id.trim().is_empty() {
        return Err(WebsocketDeviceBindingError {
            code: "device_id_missing",
            message: "device id must not be empty".to_owned(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use im_app_context::local_service_app_context;

    use super::*;

    #[test]
    fn resolve_device_binding_prefers_matching_requested_device_id() {
        let auth = local_service_app_context("t1", "u1", "user", Some("device_real"), ["*"]);
        assert_eq!(
            resolve_websocket_device_binding(&auth, Some("device_real".to_owned())),
            Ok("device_real".to_owned())
        );
    }

    #[test]
    fn resolve_device_binding_rejects_mismatch() {
        let auth = local_service_app_context("t1", "u1", "user", Some("device_real"), ["*"]);
        assert_eq!(
            resolve_websocket_device_binding(&auth, Some("device-frame".to_owned())).map_err(|e| e.code),
            Err("device_id_mismatch")
        );
    }
}

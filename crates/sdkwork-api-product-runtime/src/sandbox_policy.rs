use anyhow::Result;

pub(crate) fn ensure_admin_sandbox_allowed(enabled: bool, production_like: bool) -> Result<()> {
    if enabled && production_like {
        anyhow::bail!(
            "SDKWORK_ADMIN_SANDBOX is development-only and must be disabled in production-like environments"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::ensure_admin_sandbox_allowed;

    #[test]
    fn production_like_environment_rejects_admin_sandbox() {
        let error = ensure_admin_sandbox_allowed(true, true)
            .expect_err("production-like runtimes must reject seeded admin data");

        assert!(error.to_string().contains("SDKWORK_ADMIN_SANDBOX"));
    }

    #[test]
    fn development_can_explicitly_enable_admin_sandbox() {
        ensure_admin_sandbox_allowed(true, false)
            .expect("development runtimes may explicitly enable the sandbox");
    }

    #[test]
    fn disabled_admin_sandbox_is_valid_in_every_environment() {
        ensure_admin_sandbox_allowed(false, true)
            .expect("production-like runtimes are valid when the sandbox is disabled");
        ensure_admin_sandbox_allowed(false, false)
            .expect("development runtimes are valid when the sandbox is disabled");
    }
}

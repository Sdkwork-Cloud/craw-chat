# SDKWork IM PC Source Configuration

This deployable renderer consumes the enclosing IM deployment profile from
`../../../etc/sdkwork.deployment.config.json`. `browser.runtime.json` owns only PC renderer
bindings and the local Vite target; public domains and SDK Base URLs remain root deployment values.

Validate with `node ../../../sdkwork-specs/tools/check-source-config-standard.mjs --root .` from this app root.

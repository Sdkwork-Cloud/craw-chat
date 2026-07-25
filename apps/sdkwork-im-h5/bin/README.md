# bin

Cross-platform operational scripts for the Sdkwork IM H5 application root.

This directory hosts operational helper commands for build, install, run,
diagnostics, and mobile host integration. Native platform subdirectories
(`ios/`, `android/`) hold host-adapter helper scripts reserved for the
Capacitor host target (not yet materialized for H5-only runtime).

Scripts here are operational entry points; they must not duplicate build
source logic owned by `scripts/` or repository-root tooling.

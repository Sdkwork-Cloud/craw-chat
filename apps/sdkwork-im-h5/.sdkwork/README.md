# SDKWork Workspace Metadata

This directory is the source-controlled `.sdkwork/` workspace for the `sdkwork-im-h5` application root per `SDKWORK_WORKSPACE_SPEC.md`.

It is not generated SDK output and is not user runtime state. It owns local skills, plugins, manifests, and AI workspace metadata for this application root.

## Structure

- `skills/`: local SDKWork skills scoped to this application root.
- `plugins/`: local SDKWork plugins scoped to this application root.

## Rules

- Do not copy global `sdkwork-specs` content into this directory.
- Do not store runtime state, secrets, build output, or generated SDK artifacts here.
- Local skills and plugins must reference global standards through relative paths.

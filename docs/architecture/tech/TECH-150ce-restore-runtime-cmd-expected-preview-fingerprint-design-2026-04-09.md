> Migrated from `docs/架构/150CE-restore-runtime-cmd-expected-preview-fingerprint-design-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Restore Runtime CMD Expected Preview Fingerprint Design

## Decision

- Treat Windows `.cmd` restore forwarding as part of the visible operator contract, not as an incidental wrapper detail.

## Contract

- `restore-runtime-local.cmd` must accept `--expected-preview-fingerprint <fingerprint>` and pass `-ExpectedPreviewFingerprint` into PowerShell.
- Existing `--backup-dir` forwarding remains unchanged.
- The preview-confirmation guard continues to be enforced by the restore entrypoint; the wrapper must not strip it.

## Rationale

- Repo docs already advertise the GNU-style restore confirmation flag.
- Losing the flag on Windows weakens the restore safety contract only on one platform, which is exactly the class of drift Step 10 is meant to prevent.
- Fixing the shared forwarder is the smallest change and covers future wrappers that may reuse the same parameter.

## Boundary

- This design covers Windows wrapper normalization only.
- It does not alter preview fingerprint generation, validation rules, or restore side effects.


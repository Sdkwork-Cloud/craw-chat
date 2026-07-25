# Client-Local Fixtures

Fixtures in this directory are test-only and must never be read by production
bootstrap. Rust tests construct encrypted records with deterministic test keys
so no credential-vault entry or real user content is required.


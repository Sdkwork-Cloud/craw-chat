// sdkwork-im docker test deployment: credential-entry bootstrap Access-Token.
// Injected into the served index.html by nginx (sub_filter in
// testapidocker-im.conf) and loaded as an EXTERNAL same-origin script so it
// passes the gateway CSP (script-src 'self'; inline scripts are blocked unless
// nonce-tagged, so the token cannot be inlined).
// This is the well-known unsigned development fallback JWT documented in
// deployments/docker/README.md; anything real must set a signed token via
// SDKWORK_ACCESS_TOKEN at renderer build time.
globalThis.__SDKWORK_CREDENTIAL_ENTRY_BOOTSTRAP_ACCESS_TOKEN__ =
  "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0b2tlbl90eXBlIjoiYWNjZXNzIiwidGVuYW50X2lkIjoiMTAwMDAxIiwidXNlcl9pZCI6InN5c3RlbSIsImFwcF9pZCI6ImFwcF8xMDAwMDEiLCJsb2dpbl9zY29wZSI6IlRFTkFOVCIsInRva2VuX3ZlcnNpb24iOjF9.test-signature";

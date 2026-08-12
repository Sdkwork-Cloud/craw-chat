# sdkwork-im-h5-moments Specs

Local spec index for the H5 Moments feature package.

## Contracts

- Machine authority: [`component.spec.json`](./component.spec.json)
- Moments consume the injected Community App SDK port through
  `getMomentsRuntimePort`; the package owns no transport, no browser business
  state, and no fabricated media.
- Global standards authority: `sdkwork-specs/` (resolved from the workspace
  root), never copied into this directory.

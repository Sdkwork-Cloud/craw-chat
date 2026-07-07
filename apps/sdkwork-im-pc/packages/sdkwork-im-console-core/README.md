# @sdkwork/im-console-core

Domain: communication  
Capability: im-console-core

IM console core owns the console layout contract and core composition ports. The PC application
bootstrap injects console capability route elements into `ConsoleLayout`; this package must not
depend on console capability packages directly.

## Ownership

| Surface | Location | Notes |
| --- | --- | --- |
| `ConsoleLayout` | This package | Accepts capability route elements through the `routes` prop |
| Console capability pages | PC application bootstrap | Imported by the composition root and injected into core |

Do not add new domain console features here when a sibling product repo owns the domain. Do not add
feature-package dependencies to this core package. See
`docs/architecture/tech/INTEGRATION-ADAPTER-REGISTER.md`.

Machine-readable contract: `specs/component.spec.json`.

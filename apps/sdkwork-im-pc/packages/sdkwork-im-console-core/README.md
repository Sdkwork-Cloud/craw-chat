# @sdkwork/im-console-core

Domain: communication  
Capability: im-console-core

IM console shell layout. Domain admin surfaces import canonical sibling packages (for example `@sdkwork/course-pc-console`).

## Ownership

| Surface | Location | Notes |
| --- | --- | --- |
| `ConsoleLayout` | This package | IM console shell |
| Course admin | `@sdkwork/course-pc-console` | Wired via `bootstrapImCourseConsolePcIntegration` |

Do not add new domain console features here when a sibling product repo owns the domain. See `docs/architecture/tech/INTEGRATION-ADAPTER-REGISTER.md`.

Machine-readable contract: `specs/component.spec.json`.

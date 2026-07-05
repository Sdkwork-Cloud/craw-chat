# @sdkwork/im-console-core

Domain: communication  
Capability: im-console-core

IM console shell layout plus **temporary** course admin surfaces pending migration to `sdkwork-course`.

## Ownership

| Surface | Current location | Canonical owner |
| --- | --- | --- |
| `ConsoleLayout` | This package | IM console shell |
| `ConsoleCourse`, `CourseConsoleService` | This package | **Migrate to** `sdkwork-course` console package |

Do not add new domain console features here when a sibling product repo owns the domain. See `docs/architecture/tech/INTEGRATION-ADAPTER-REGISTER.md`.

Machine-readable contract: `specs/component.spec.json`.

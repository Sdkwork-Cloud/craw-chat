# Sdkwork IM Server Installation And Deployment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish the first production-grade `sdkwork-im-server` install, config, PostgreSQL initialization, and service-management contract across Linux, macOS, and Windows.

**Architecture:** Keep the existing `local-minimal` deployment surface intact and add a parallel `server` deployment surface. Reuse the repo's existing release bundle and deployment-doc conventions, introduce shared server path/profile helpers, define file-based PostgreSQL configuration, and add cross-platform lifecycle scripts plus a first `systemd` service template. Start with archive-friendly payload semantics and leave native installer packaging as the next layer.

**Tech Stack:** Rust deployment tests, PowerShell, Bash, CMD wrappers, YAML/env templates, Markdown operator docs, existing release bundle conventions

---

## File Structure

### Existing files to modify

- `services/local-minimal-node/tests/deployment_profile_test.rs`
- `bin/_runtime-profile-common.ps1`
- `bin/_runtime-profile-common.sh`
- `deployments/templates/local-default.env.example`
- `docs/部署/README.md`
- `artifacts/releases/README.md`

### New files to create

- `docs/superpowers/specs/2026-04-17-sdkwork-im-server-installation-and-deployment-design.md`
- `docs/superpowers/plans/2026-04-17-sdkwork-im-server-installation-and-deployment.md`
- `deployments/templates/server.yaml.example`
- `deployments/templates/server.env.example`
- `deployments/templates/postgresql.yaml.example`
- `deployments/templates/quickstart-server-compose.env.example`
- `deployments/systemd/sdkwork-im-server.service`
- `bin/install-server.ps1`
- `bin/install-server.sh`
- `bin/install-server.cmd`
- `bin/init-config-server.ps1`
- `bin/init-config-server.sh`
- `bin/init-config-server.cmd`
- `bin/init-storage-server.ps1`
- `bin/init-storage-server.sh`
- `bin/init-storage-server.cmd`
- `bin/verify-server.ps1`
- `bin/verify-server.sh`
- `bin/verify-server.cmd`
- `bin/install-service-server.ps1`
- `bin/install-service-server.sh`
- `bin/install-service-server.cmd`
- `bin/status-server.ps1`
- `bin/status-server.sh`
- `bin/status-server.cmd`
- `docs/部署/server版本安装与初始化.md`
- `docs/部署/server版本配置与PostgreSQL接入.md`
- `docs/部署/server版本service托管标准.md`

---

### Task 1: Freeze The Server Deployment Contract In Tests

**Files:**
- Modify: `services/local-minimal-node/tests/deployment_profile_test.rs`

- [ ] **Step 1: Write the failing deployment-contract tests**

Add tests that require:

- the new `server` command family to be documented in help or deployment docs
- `server` template files to exist under `deployments/templates`
- a dedicated `systemd` service template to exist
- docs to mention externally managed PostgreSQL via config file

- [ ] **Step 2: Run the focused deployment tests to verify RED**

Run:

```bash
cargo test -p local-minimal-node --offline --test deployment_profile_test server_ -- --nocapture
```

Expected: FAIL because the `server` scripts/templates/docs do not exist yet.

- [ ] **Step 3: Keep the failures precise**

Adjust assertions until the failures clearly identify the missing contract boundary instead of generic filesystem errors.

### Task 2: Add Shared Server Path And Profile Helpers

**Files:**
- Modify: `bin/_runtime-profile-common.ps1`
- Modify: `bin/_runtime-profile-common.sh`

- [ ] **Step 1: Add a failing helper-focused test**

Extend `deployment_profile_test.rs` to require:

- server install/config/data/log/run roots can be derived for the default instance
- helper output distinguishes server roots from `local-minimal` runtime roots

- [ ] **Step 2: Run the targeted test to verify RED**

Run:

```bash
cargo test -p local-minimal-node --offline --test deployment_profile_test test_server_runtime_helper_contract -- --exact --nocapture
```

Expected: FAIL because server helper functions do not exist yet.

- [ ] **Step 3: Implement minimal helper additions**

Add focused helper functions only for:

- resolving server default instance roots
- resolving server config/data/log/run paths
- keeping local helper behavior unchanged

- [ ] **Step 4: Re-run the helper test**

Run the same command and confirm PASS.

### Task 3: Add Server Templates

**Files:**
- Create: `deployments/templates/server.yaml.example`
- Create: `deployments/templates/server.env.example`
- Create: `deployments/templates/postgresql.yaml.example`
- Create: `deployments/templates/quickstart-server-compose.env.example`
- Modify: `deployments/templates/local-default.env.example`

- [ ] **Step 1: Add a failing template-contract test**

Require:

- `server.yaml.example` includes instance name, bind address, base URL, API URL, WebSocket URL, data/log/run roots
- `postgresql.yaml.example` includes external-managed configuration, password file, migration mode, and provisioning mode
- quickstart env example references the same server config model

- [ ] **Step 2: Run the targeted template test to verify RED**

Run:

```bash
cargo test -p local-minimal-node --offline --test deployment_profile_test test_server_templates_freeze_cross_platform_contract -- --exact --nocapture
```

Expected: FAIL because the template files do not exist yet.

- [ ] **Step 3: Add minimal templates**

Keep the templates explicit and operator-readable. Do not add speculative fields beyond the approved design.

- [ ] **Step 4: Re-run the template test**

Confirm PASS.

### Task 4: Add Cross-Platform Server Install And Config Scripts

**Files:**
- Create: `bin/install-server.ps1`
- Create: `bin/install-server.sh`
- Create: `bin/install-server.cmd`
- Create: `bin/init-config-server.ps1`
- Create: `bin/init-config-server.sh`
- Create: `bin/init-config-server.cmd`

- [ ] **Step 1: Add failing script-surface tests**

Require:

- consistent help text across PowerShell, Bash, and CMD wrappers
- support for `--instance`
- support for explicit config/data/log/run roots
- support for non-interactive mode

- [ ] **Step 2: Run the focused script-surface tests**

Run:

```bash
cargo test -p local-minimal-node --offline --test deployment_profile_test test_server_install_scripts_expose_consistent_help_surface -- --exact --nocapture
```

Expected: FAIL because the scripts do not exist yet.

- [ ] **Step 3: Implement the minimal install/config scripts**

`install-server.*` should:

- create standard directory skeletons
- copy templates if absent
- avoid overwriting user config unless explicitly forced

`init-config-server.*` should:

- render template placeholders into the target config root
- support existing external PostgreSQL config files
- avoid hidden defaults outside the documented config files

- [ ] **Step 4: Re-run the focused script tests**

Confirm PASS.

### Task 5: Add Cross-Platform PostgreSQL Initialization And Verification Scripts

**Files:**
- Create: `bin/init-storage-server.ps1`
- Create: `bin/init-storage-server.sh`
- Create: `bin/init-storage-server.cmd`
- Create: `bin/verify-server.ps1`
- Create: `bin/verify-server.sh`
- Create: `bin/verify-server.cmd`

- [ ] **Step 1: Add failing storage/verify tests**

Require:

- `init-storage-server` help surfaces `verify-only`, `bootstrap-schema`, and `create-db-and-schema`
- `verify-server` help surfaces config validation, storage validation, and JSON/text output modes
- docs reference the externally managed PostgreSQL path

- [ ] **Step 2: Run the targeted storage/verify tests**

Run:

```bash
cargo test -p local-minimal-node --offline --test deployment_profile_test test_server_storage_and_verify_scripts_freeze_postgresql_contract -- --exact --nocapture
```

Expected: FAIL because the new scripts do not exist yet.

- [ ] **Step 3: Implement minimal non-destructive versions**

For the first landing:

- parse args
- load files
- validate required paths and config keys
- emit truthful placeholder status/report output

Do not fake real DB mutation. The first landing only needs the contract skeleton plus safe verification behavior.

- [ ] **Step 4: Re-run the targeted tests**

Confirm PASS.

### Task 6: Add Initial Service Wrapper Surface

**Files:**
- Create: `deployments/systemd/sdkwork-im-server.service`
- Create: `bin/install-service-server.ps1`
- Create: `bin/install-service-server.sh`
- Create: `bin/install-service-server.cmd`
- Create: `bin/status-server.ps1`
- Create: `bin/status-server.sh`
- Create: `bin/status-server.cmd`

- [ ] **Step 1: Add failing service-surface tests**

Require:

- systemd service template exists and points at the server install/config contract
- install-service/status scripts expose stable help text
- docs state the future `launchd` and Windows Service targets even if the first landing only ships the systemd template

- [ ] **Step 2: Run the focused service tests**

Run:

```bash
cargo test -p local-minimal-node --offline --test deployment_profile_test test_server_service_surface_is_frozen -- --exact --nocapture
```

Expected: FAIL because the service template and scripts do not exist yet.

- [ ] **Step 3: Implement minimal service wrappers**

The first landing should:

- add a real `systemd` unit template
- add wrappers that render or report the intended service installation steps
- leave macOS and Windows native service registration documented as next-stage work

- [ ] **Step 4: Re-run the focused service tests**

Confirm PASS.

### Task 7: Update Deployment Docs And Release Bundle Docs

**Files:**
- Modify: `docs/部署/README.md`
- Modify: `artifacts/releases/README.md`
- Create: `docs/部署/server版本安装与初始化.md`
- Create: `docs/部署/server版本配置与PostgreSQL接入.md`
- Create: `docs/部署/server版本service托管标准.md`

- [ ] **Step 1: Add failing documentation assertions**

Require the docs to freeze:

- server package/install model
- archive vs native installer boundary
- external PostgreSQL config-file workflow
- canonical payload relation to `artifacts/releases`

- [ ] **Step 2: Run the full deployment-profile suite to verify RED/GREEN progression**

Run:

```bash
cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture
```

Expected: targeted server tests first fail, then the full suite passes after docs and scripts are added.

- [ ] **Step 3: Update docs minimally and accurately**

Do not promise `.deb`, `.rpm`, `.pkg`, or `.msi` generation in the first landing. Document them as the native-installer targets that build on the same payload standard.

- [ ] **Step 4: Re-run the full deployment-profile suite**

Confirm PASS.

- [ ] **Step 5: Run a final focused script smoke pass**

Run:

```bash
powershell -NoProfile -ExecutionPolicy Bypass -File bin/install-server.ps1 -Help
powershell -NoProfile -ExecutionPolicy Bypass -File bin/init-config-server.ps1 -Help
powershell -NoProfile -ExecutionPolicy Bypass -File bin/init-storage-server.ps1 -Help
powershell -NoProfile -ExecutionPolicy Bypass -File bin/verify-server.ps1 -Help
```

Expected: each command prints the new server command contract cleanly.

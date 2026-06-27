# SDKWork Chat PC Plugins

Application-local agent plugins for `apps/sdkwork-chat-pc` belong here.

Installable plugins must include `.codex-plugin/plugin.json` and must document contributed skills,
tools, apps, scripts, and verification. Plugin skills follow the same rules as
`.sdkwork/skills/`.

Do not store application/runtime source plugins here; runtime plugin source belongs in the
repository root `plugins/` directory or an approved application package.

Plugins must not store generated SDK output, secrets, runtime databases, caches, logs, or
user-private files.

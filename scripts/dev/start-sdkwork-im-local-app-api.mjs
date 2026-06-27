#!/usr/bin/env node

import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

export { resolveSdkworkImSharedDatabaseConfig } from './sdkwork-im-shared-database.mjs';

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.stderr.write(
    [
      'start-sdkwork-im-local-app-api.mjs no longer starts a separate app-api process.',
      'Sdkwork IM app-api is provided by the Rust unified server.',
      'Use `pnpm dev:server` or `node scripts/im-server-dev.mjs`.',
      '',
    ].join('\n'),
  );
  process.exitCode = 1;
}

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const rustPath = path.join(repoRoot, 'crates/sdkwork-im-realtime-api-paths/src/lib.rs');
const tsPath = path.join(
  repoRoot,
  'sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/realtime-api-paths.ts',
);

const RUST_TO_TS_EXPORTS = {
  PREFIX: 'IM_REALTIME_API_PREFIX',
  REALTIME_SUBSCRIPTIONS_SYNC: 'IM_REALTIME_SUBSCRIPTIONS_SYNC',
  REALTIME_WS: 'IM_REALTIME_WS',
  REALTIME_EVENTS_ACK: 'IM_REALTIME_EVENTS_ACK',
  REALTIME_EVENTS: 'IM_REALTIME_EVENTS',
  PRESENCE_HEARTBEAT: 'IM_PRESENCE_HEARTBEAT',
  PRESENCE_ME: 'IM_PRESENCE_ME',
};

export function readRustRealtimePathConstants() {
  const rustSource = fs.readFileSync(rustPath, 'utf8');
  const constants = new Map();

  for (const rustName of Object.keys(RUST_TO_TS_EXPORTS)) {
    const match = rustSource.match(
      new RegExp(`pub const ${rustName}: &str = "([^"]+)";`, 'u'),
    );
    if (!match) {
      throw new Error(`missing Rust realtime path constant ${rustName}`);
    }
    constants.set(rustName, match[1]);
  }

  return constants;
}

export function materializeSdkworkImRealtimeApiPaths() {
  const constants = readRustRealtimePathConstants();
  const exportLines = Object.entries(RUST_TO_TS_EXPORTS).map(([rustName, tsName]) => {
    const value = constants.get(rustName);
    return `export const ${tsName} = '${value}' as const;`;
  });

  const contents = [
    '/**',
    ' * Generated from `crates/sdkwork-im-realtime-api-paths` — do not hand-edit.',
    ' * Run `pnpm sdk:generate:realtime-api-paths` after changing Rust path constants.',
    ' */',
    '',
    ...exportLines,
    '',
    '/** @deprecated Use {@link IM_REALTIME_WS}. */',
    'export const IM_REALTIME_WEBSOCKET_PATH = IM_REALTIME_WS;',
    '',
  ].join('\n');

  fs.writeFileSync(tsPath, contents, 'utf8');
  return contents;
}

const isMain = process.argv[1] && pathToFileURL(process.argv[1]).href === import.meta.url;
if (isMain) {
  materializeSdkworkImRealtimeApiPaths();
  process.stdout.write('materialized sdkwork-im realtime api paths for TypeScript SDK\n');
}

import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const composedRoot = path.join(
  repoRoot,
  'sdks',
  'sdkwork-im-sdk',
  'sdkwork-im-sdk-flutter',
  'composed',
  'im_sdk_composed',
);
const flutterExecutable = process.platform === 'win32' ? 'flutter.bat' : 'flutter';

const result = spawnSync(flutterExecutable, ['test'], {
  cwd: composedRoot,
  stdio: 'inherit',
  shell: process.platform === 'win32',
});

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

console.log('im_sdk_composed unit tests passed.');

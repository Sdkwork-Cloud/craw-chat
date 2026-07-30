import assert from 'node:assert/strict';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import esbuild from '../../apps/sdkwork-im-pc/node_modules/esbuild/lib/main.js';
import type { Plugin } from '../../apps/sdkwork-im-pc/node_modules/esbuild/lib/main.d.ts';

const { build } = esbuild;

interface Deferred<T> {
  promise: Promise<T>;
  reject: (reason?: unknown) => void;
  resolve: (value: T) => void;
}

interface TestUploader {
  id: string;
}

interface DriveUploaderCacheTestApi {
  get(): Promise<TestUploader>;
  invalidate(): void;
}

interface DriveUploaderTestGlobal {
  __sdkworkDriveUploaderFactory?: () => Promise<TestUploader>;
}

function createDeferred<T>(): Deferred<T> {
  let reject!: (reason?: unknown) => void;
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    reject = rejectPromise;
    resolve = resolvePromise;
  });
  return { promise, reject, resolve };
}

async function waitForCondition(predicate: () => boolean, description: string): Promise<void> {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    if (predicate()) {
      return;
    }
    await new Promise<void>((resolve) => setImmediate(resolve));
  }
  assert.fail(`timed out waiting for ${description}`);
}

async function loadInstrumentedDriveCacheApi(): Promise<{
  api: DriveUploaderCacheTestApi;
  cleanup: () => Promise<void>;
}> {
  const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
  const appRoot = path.join(repositoryRoot, 'apps', 'sdkwork-im-pc');
  const chatServicePath = path.join(
    appRoot,
    'packages',
    'sdkwork-im-pc-chat',
    'src',
    'services',
    'ChatService.ts',
  );
  const runtimeDirectory = await mkdtemp(path.join(os.tmpdir(), 'sdkwork-im-drive-uploader-'));
  const outputPath = path.join(runtimeDirectory, `chat-service-drive-cache-${process.pid}.mjs`);

  const instrumentationPlugin: Plugin = {
    name: 'chat-service-drive-cache-generation-test',
    setup(buildApi) {
      buildApi.onResolve(
        { filter: /^@sdkwork\/im-pc-core\/sdk\/driveAppSdkClient$/ },
        () => ({ path: 'drive-app-sdk-client-test-double', namespace: 'drive-test' }),
      );
      buildApi.onLoad({ filter: /.*/, namespace: 'drive-test' }, () => ({
        contents: `
          export function getDriveAppSdkClientWithSession() {
            return { uploader: globalThis.__sdkworkDriveUploaderFactory() };
          }
        `,
        loader: 'js',
      }));
      buildApi.onLoad({ filter: /ChatService\.ts$/, namespace: 'file' }, async (args) => {
        if (path.resolve(args.path) !== path.resolve(chatServicePath)) {
          return undefined;
        }
        const source = await readFile(args.path, 'utf8');
        return {
          contents: `${source}
            export const __driveUploaderCacheTestApi = {
              get: getDefaultDriveUploader,
              invalidate() {
                driveUploaderClient = null;
                driveUploaderClientPromise = null;
                if (typeof driveUploaderClientGeneration !== 'undefined') {
                  driveUploaderClientGeneration += 1;
                }
              },
            };
          `,
          loader: 'ts',
          resolveDir: path.dirname(args.path),
        };
      });
    },
  };

  try {
    await build({
      absWorkingDir: appRoot,
      bundle: true,
      entryPoints: [chatServicePath],
      format: 'esm',
      outfile: outputPath,
      platform: 'node',
      plugins: [instrumentationPlugin],
      sourcemap: false,
      target: 'es2022',
    });
    const loaded = await import(`${pathToFileURL(outputPath).href}?v=${Date.now()}`) as {
      __driveUploaderCacheTestApi: DriveUploaderCacheTestApi;
    };
    return {
      api: loaded.__driveUploaderCacheTestApi,
      cleanup: async () => {
        await rm(runtimeDirectory, { force: true, recursive: true });
      },
    };
  } catch (error) {
    await rm(runtimeDirectory, { force: true, recursive: true });
    throw error;
  }
}

async function assertLateOldSuccessCannotReplaceNewUploader(): Promise<void> {
  const requests: Array<Deferred<TestUploader>> = [];
  (globalThis as DriveUploaderTestGlobal).__sdkworkDriveUploaderFactory = () => {
    const deferred = createDeferred<TestUploader>();
    requests.push(deferred);
    return deferred.promise;
  };
  const { api, cleanup } = await loadInstrumentedDriveCacheApi();
  try {
    const oldUploaderPromise = api.get();
    api.invalidate();
    const newUploaderPromise = api.get();
    await waitForCondition(() => requests.length === 2, 'both uploader initializations');

    const newUploader = { id: 'new-uploader' };
    requests[1]?.resolve(newUploader);
    assert.equal(await newUploaderPromise, newUploader);
    requests[0]?.resolve({ id: 'old-uploader' });
    await oldUploaderPromise;

    assert.equal(
      await api.get(),
      newUploader,
      'a late old uploader success must not replace the current generation cache',
    );
    assert.equal(requests.length, 2, 'reading the current cache must not initialize a third uploader');
  } finally {
    await cleanup();
  }
}

async function assertLateOldFailureCannotClearNewUploaderPromise(): Promise<void> {
  const requests: Array<Deferred<TestUploader>> = [];
  (globalThis as DriveUploaderTestGlobal).__sdkworkDriveUploaderFactory = () => {
    const deferred = createDeferred<TestUploader>();
    requests.push(deferred);
    return deferred.promise;
  };
  const { api, cleanup } = await loadInstrumentedDriveCacheApi();
  try {
    const oldUploaderPromise = api.get();
    api.invalidate();
    const newUploaderPromise = api.get();
    await waitForCondition(() => requests.length === 2, 'both uploader initialization promises');

    const oldError = new Error('old uploader initialization failed');
    requests[0]?.reject(oldError);
    await assert.rejects(oldUploaderPromise, (error: unknown) => error === oldError);
    const coalescedNewPromise = api.get();
    await new Promise<void>((resolve) => setImmediate(resolve));
    assert.equal(
      requests.length,
      2,
      'an old rejection must not clear the new generation uploader promise',
    );

    const newUploader = { id: 'new-uploader' };
    requests[1]?.resolve(newUploader);
    assert.equal(await newUploaderPromise, newUploader);
    assert.equal(await coalescedNewPromise, newUploader);
  } finally {
    await cleanup();
  }
}

async function main(): Promise<void> {
  const checks = new Map<string, () => Promise<void>>([
    ['late-success', assertLateOldSuccessCannotReplaceNewUploader],
    ['late-failure', assertLateOldFailureCannotClearNewUploaderPromise],
  ]);
  const selectedCheck = process.argv[2];
  try {
    if (selectedCheck) {
      const check = checks.get(selectedCheck);
      assert.ok(check, `unknown drive uploader generation check: ${selectedCheck}`);
      await check();
    } else {
      for (const check of checks.values()) {
        await check();
      }
    }
  } finally {
    delete (globalThis as DriveUploaderTestGlobal).__sdkworkDriveUploaderFactory;
  }
  console.log('sdkwork im pc drive uploader generation contract passed.');
}

void main();

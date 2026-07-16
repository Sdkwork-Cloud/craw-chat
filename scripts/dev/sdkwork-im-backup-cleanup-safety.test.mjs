#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const backupScript = path.join(repoRoot, 'scripts', 'backup.sh');
const fixedNowEpoch = '1784160000'; // 2026-07-16T00:00:00Z

function resolveBash() {
  const candidates = [
    process.env.SDKWORK_BACKUP_TEST_BASH,
    process.platform === 'win32' ? 'C:\\Program Files\\Git\\bin\\bash.exe' : undefined,
    '/bin/bash',
  ].filter(Boolean);

  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }
  throw new Error('A POSIX Bash executable is required for the backup cleanup safety test.');
}

function toBashPath(absolutePath) {
  const normalized = absolutePath.replaceAll('\\', '/');
  const driveMatch = /^([A-Za-z]):\//u.exec(normalized);
  return driveMatch ? `/${driveMatch[1].toLowerCase()}${normalized.slice(2)}` : normalized;
}

function shellQuote(value) {
  return `'${String(value).replaceAll("'", "'\\\"'\\\"'")}'`;
}

function writeExecutable(filePath, source) {
  fs.writeFileSync(filePath, source, { encoding: 'utf8', mode: 0o755 });
  fs.chmodSync(filePath, 0o755);
}

function readLogLines(logPath) {
  if (!fs.existsSync(logPath)) {
    return [];
  }
  return fs
    .readFileSync(logPath, 'utf8')
    .trim()
    .split(/\r?\n/u)
    .filter(Boolean);
}

const fakeAwsSource = String.raw`#!/usr/bin/env bash
set -euo pipefail

option_value() {
  local option_name="$1"
  shift
  while (( $# > 0 )); do
    if [[ "$1" == "$option_name" ]]; then
      shift
      if (( $# > 0 )); then
        printf '%s\n' "$1"
      fi
      return 0
    fi
    shift
  done
}

if [[ "$1" == "s3api" && "$2" == "list-objects-v2" ]]; then
  prefix="$(option_value --prefix "$@")"
  token="$(option_value --continuation-token "$@")"
  if [[ -z "$token" ]]; then
    offset=0
    token_log="-"
  elif [[ "$token" =~ ^[0-9]+$ ]]; then
    offset="$token"
    token_log="$token"
  else
    printf 'invalid continuation token\n' >&2
    exit 2
  fi
  printf 'list %s %s\n' "$prefix" "$token_log" >> "$SDKWORK_BACKUP_TEST_AWS_LOG"
  awk -F '\t' -v prefix="$prefix" -v offset="$offset" -v page_size="$SDKWORK_BACKUP_TEST_PAGE_SIZE" '
    index($1, prefix) == 1 {
      matches++
      if (matches > offset && selected < page_size) {
        keys[++selected] = $1
        modified[selected] = $2
      }
    }
    END {
      print "{"
      print "  \"Contents\":["
      for (item_index = 1; item_index <= selected; item_index++) {
        suffix = item_index < selected ? "," : ""
        printf "    {\"Key\":\"%s\",\"LastModified\":\"%s\"}%s\n", keys[item_index], modified[item_index], suffix
      }
      print "  ],"
      truncated = matches > offset + selected
      printf "  \"IsTruncated\":%s", truncated ? "true" : "false"
      if (truncated) {
        printf ",\n  \"NextContinuationToken\":\"%d\"", offset + selected
      }
      print ""
      print "}"
    }
  ' "$SDKWORK_BACKUP_TEST_STATE"
  exit 0
fi

if [[ "$1" == "s3api" && "$2" == "delete-object" ]]; then
  key="$(option_value --key "$@")"
  if [[ -z "$key" ]]; then
    printf 'missing delete key\n' >&2
    exit 2
  fi
  printf 'delete %s\n' "$key" >> "$SDKWORK_BACKUP_TEST_AWS_LOG"
  printf '{}\n'
  exit 0
fi

printf 'unexpected fake aws command: %s\n' "$*" >&2
exit 2
`;

const fakeJqSource = String.raw`#!/usr/bin/env bash
set -euo pipefail

mode="$1"
expression="$2"
input_file="$3"

if [[ "$expression" == *"Contents[]?"* ]]; then
  sed -nE 's/^[[:space:]]*\{"Key":"([^"]*)","LastModified":"([^"]*)"\},?$/\1\t\2/p' "$input_file"
  exit 0
fi

if [[ "$mode" == "-e" && "$expression" == *"NextContinuationToken"* ]]; then
  if grep -Eq '"NextContinuationToken":"[^"]+"' "$input_file"; then
    printf 'true\n'
    exit 0
  fi
  exit 1
fi

if [[ "$mode" == "-e" ]]; then
  printf 'true\n'
  exit 0
fi

if [[ "$expression" == *"IsTruncated"* ]]; then
  if grep -q '"IsTruncated":true' "$input_file"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
  exit 0
fi

if [[ "$expression" == *"NextContinuationToken"* ]]; then
  sed -nE 's/^[[:space:]]*"NextContinuationToken":"([^"]+)"$/\1/p' "$input_file"
  exit 0
fi

printf 'unexpected fake jq expression\n' >&2
exit 2
`;

const bash = resolveBash();
const testRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-backup-cleanup-'));

try {
  const binDir = path.join(testRoot, 'bin');
  const tempDir = path.join(testRoot, 'tmp');
  fs.mkdirSync(binDir, { recursive: true });
  fs.mkdirSync(tempDir, { recursive: true });

  writeExecutable(path.join(binDir, 'aws'), fakeAwsSource);
  writeExecutable(path.join(binDir, 'jq'), fakeJqSource);
  writeExecutable(
    path.join(binDir, 'date'),
    '#!/usr/bin/env bash\nif [[ "$#" -eq 2 && "$1" == "-u" && "$2" == "+%s" ]]; then printf "%s\\n" "$SDKWORK_BACKUP_TEST_NOW_EPOCH"; exit 0; fi\nexec /usr/bin/date "$@"\n',
  );

  let cleanupRunSequence = 0;

  function runCleanup({ objects, dryRun, deleteLimit, pageSize = 1000 }) {
    cleanupRunSequence += 1;
    const statePath = path.join(testRoot, `state-${cleanupRunSequence}.tsv`);
    const logPath = path.join(testRoot, `aws-${cleanupRunSequence}.log`);
    fs.writeFileSync(
      statePath,
      `${objects.map((object) => `${object.Key}\t${object.LastModified}`).join('\n')}\n`,
      'utf8',
    );

    const command = [
      `source ${shellQuote(toBashPath(backupScript))}`,
      `S3_BUCKET=${shellQuote('s3://backup-sdkwork-im/tenant-a')}`,
      'RETENTION_DAYS=30',
      `DELETE_LIMIT=${deleteLimit}`,
      `DRY_RUN=${dryRun ? 'true' : 'false'}`,
      'validate_cleanup_configuration',
      'cleanup_expired_backups',
    ].join('\n');
    const result = spawnSync(bash, ['-c', command], {
      cwd: repoRoot,
      encoding: 'utf8',
      env: {
        ...process.env,
        PATH: `${toBashPath(binDir)}:/usr/bin:/bin`,
        TMPDIR: toBashPath(tempDir),
        SDKWORK_BACKUP_TEST_STATE: toBashPath(statePath),
        SDKWORK_BACKUP_TEST_AWS_LOG: toBashPath(logPath),
        SDKWORK_BACKUP_TEST_PAGE_SIZE: String(pageSize),
        SDKWORK_BACKUP_TEST_NOW_EPOCH: fixedNowEpoch,
      },
    });

    return {
      ...result,
      output: `${result.stdout ?? ''}${result.stderr ?? ''}`,
      logLines: readLogLines(logPath),
    };
  }

  const objects = [
    {
      Key: 'tenant-a/config/sdkwork-im-config_20260501_010101.tar.gz',
      LastModified: '2026-05-01T01:01:01+00:00',
    },
    {
      // This companion is old by LastModified but belongs to the newest database recovery point.
      Key: 'tenant-a/config/sdkwork-im-config_20260715_010101.tar.gz',
      LastModified: '2026-05-01T01:01:01+00:00',
    },
    {
      Key: 'tenant-a/config/operator-note.txt',
      LastModified: '2026-05-01T01:01:01+00:00',
    },
    {
      Key: 'tenant-a/db-full/sdkwork-im-db_20260501_010101.dump',
      LastModified: '2026-05-01T01:01:01+00:00',
    },
    {
      Key: 'tenant-a/db-full/sdkwork-im-db_20260715_010101.dump',
      LastModified: '2026-07-15T01:01:01+00:00',
    },
    {
      Key: 'tenant-a/db-full/not-a-backup.dump',
      LastModified: '2026-05-01T01:01:01+00:00',
    },
    {
      Key: 'tenant-a/redis/sdkwork-im-redis_20260501_010101.rdb',
      LastModified: '2026-05-01T01:01:01+00:00',
    },
    {
      // The newest Redis snapshot is preserved even though it is outside the latest DB timestamp.
      Key: 'tenant-a/redis/sdkwork-im-redis_20260710_010101.rdb',
      LastModified: '2026-05-01T01:01:02+00:00',
    },
  ];

  const expectedExpiredKeys = [
    'tenant-a/config/sdkwork-im-config_20260501_010101.tar.gz',
    'tenant-a/db-full/sdkwork-im-db_20260501_010101.dump',
    'tenant-a/redis/sdkwork-im-redis_20260501_010101.rdb',
  ];

  const dryRun = runCleanup({ objects, dryRun: true, deleteLimit: 5, pageSize: 2 });
  assert.equal(dryRun.status, 0, dryRun.output);
  for (const key of expectedExpiredKeys) {
    assert.match(dryRun.output, new RegExp(`would delete .*${key}`, 'u'));
  }
  assert.doesNotMatch(dryRun.output, /sdkwork-im-config_20260715_010101/u);
  assert.doesNotMatch(dryRun.output, /sdkwork-im-db_20260715_010101/u);
  assert.doesNotMatch(dryRun.output, /sdkwork-im-redis_20260710_010101/u);
  assert.deepEqual(
    dryRun.logLines.filter((line) => line.startsWith('delete ')),
    [],
    'dry-run must not invoke delete-object',
  );
  assert.ok(
    dryRun.logLines.some((line) => /list tenant-a\/config\/ 2$/u.test(line)),
    'cleanup must follow continuation tokens instead of loading an unbounded listing',
  );

  const destructiveRun = runCleanup({ objects, dryRun: false, deleteLimit: 5 });
  assert.equal(destructiveRun.status, 0, destructiveRun.output);
  assert.deepEqual(
    destructiveRun.logLines
      .filter((line) => line.startsWith('delete '))
      .map((line) => line.slice('delete '.length))
      .sort(),
    [...expectedExpiredKeys].sort(),
    'only strictly named, expired, unprotected backup objects may be deleted',
  );

  const overLimit = runCleanup({ objects, dryRun: false, deleteLimit: 2 });
  assert.notEqual(overLimit.status, 0, 'cleanup must fail before deletion when the cap is exceeded');
  assert.match(overLimit.output, /more than the configured limit/u);
  assert.deepEqual(
    overLimit.logLines.filter((line) => line.startsWith('delete ')),
    [],
    'the deletion cap must be evaluated before any delete-object call',
  );

  const noRecoveryPoint = runCleanup({
    objects: objects.filter((object) => !object.Key.includes('/db-full/')),
    dryRun: false,
    deleteLimit: 5,
  });
  assert.notEqual(noRecoveryPoint.status, 0, 'cleanup must fail closed without a database restore point');
  assert.match(noRecoveryPoint.output, /No valid database backup recovery point/u);
  assert.deepEqual(noRecoveryPoint.logLines.filter((line) => line.startsWith('delete ')), []);

  const invalidMetadata = runCleanup({
    objects: [
      {
        Key: 'tenant-a/db-full/sdkwork-im-db_20260501_010101.dump',
        LastModified: 'not-a-valid-timestamp',
      },
    ],
    dryRun: false,
    deleteLimit: 5,
  });
  assert.notEqual(invalidMetadata.status, 0, 'cleanup must fail closed on malformed LastModified metadata');
  assert.match(invalidMetadata.output, /invalid LastModified value/u);
  assert.deepEqual(invalidMetadata.logLines.filter((line) => line.startsWith('delete ')), []);

  process.stdout.write('sdkwork-im backup cleanup safety test passed\n');
} finally {
  fs.rmSync(testRoot, { recursive: true, force: true });
}

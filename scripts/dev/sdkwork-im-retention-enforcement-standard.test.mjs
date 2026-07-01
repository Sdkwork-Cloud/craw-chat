import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, ...relativePath.split('/')), 'utf8');
}

function readExists(relativePath) {
  const absolutePath = path.join(repoRoot, ...relativePath.split('/'));
  assert.ok(fs.existsSync(absolutePath), `expected file ${relativePath}`);
  return fs.readFileSync(absolutePath, 'utf8');
}

const envExample = read('.env.postgres.example');
for (const required of [
  'SDKWORK_IM_RETENTION_PURGE_SCHEDULER_ENABLED=true',
  'SDKWORK_IM_RETENTION_PURGE_INTERVAL_SECONDS=3600',
  'SDKWORK_IM_RETENTION_PURGE_BATCH_SIZE=500',
  'SDKWORK_IM_RETENTION_PURGE_MAX_BATCHES_PER_TICK=100',
  'SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED=true',
  'SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_INTERVAL_SECONDS=60',
]) {
  assert.ok(envExample.includes(required), `.env.postgres.example must document ${required}`);
}

const schedulerSpawnSites = [
  'services/sdkwork-im-standalone-gateway/src/main.rs',
  'services/sdkwork-im-cloud-gateway/src/main.rs',
  'services/ops-service-bin/src/main.rs',
  'services/comms-social-service-bin/src/main.rs',
];

for (const site of schedulerSpawnSites) {
  const source = readExists(site);
  if (site.includes('comms-social')) {
    assert.match(
      source,
      /spawn_shared_channel_sync_stale_reclaim_scheduler_from_env/,
      `${site} must start shared-channel stale reclaim scheduler`,
    );
  } else {
    assert.match(
      source,
      /spawn_retention_purge_scheduler_from_env/,
      `${site} must start retention purge scheduler`,
    );
    if (site.includes('cloud-gateway') || site.includes('standalone-gateway') || site.includes('ops-service')) {
      assert.match(
        source,
        /handle\.shutdown\(\)/,
        `${site} must shut down retention purge scheduler on graceful exit`,
      );
      if (site.includes('cloud-gateway')) {
        assert.match(
          source,
          /RetentionPurgeSchedulerHandle/,
          `${site} must type retention scheduler as RetentionPurgeSchedulerHandle`,
        );
      }
    }
  }
}

const journalBootstrap = readExists(
  'services/sdkwork-comms-conversation-service/src/runtime/journal_bootstrap.rs',
);
assert.match(
  journalBootstrap,
  /PostgresRetentionScopeStore/,
  'conversation journal bootstrap must wire PostgresRetentionScopeStore for legal_hold reconcile',
);

const opsApp = readExists('services/ops-service/src/app.rs');
assert.match(
  opsApp,
  /\/backend\/v3\/api\/ops\/retention\/purge/,
  'ops-service must expose manual retention purge route',
);
const postgresJournal = readExists('adapters/postgres-journal/src/lib.rs');
assert.match(postgresJournal, /retention_purge_metrics/, 'ops metrics must merge retention purge metrics');

const retentionCleanup = readExists('adapters/postgres-journal/src/retention_cleanup.rs');
assert.match(
  retentionCleanup,
  /PURGE_MESSAGE_MEDIA_REFS_SQL/,
  'retention cleanup must purge im_message_media_refs alongside other retention stores',
);

const sharedChannelMetrics = readExists('services/social-service/src/shared_channel_sync_metrics.rs');
for (const metric of [
  'im_shared_channel_sync_stale_reclaim_ticks_total',
  'im_shared_channel_sync_stale_reclaim_failures_total',
  'im_shared_channel_sync_stale_reclaim_claims_reclaimed_total',
  'im_shared_channel_sync_delivery_proofs_recorded_total',
  'im_shared_channel_sync_delivery_deduplicated_total',
]) {
  assert.ok(sharedChannelMetrics.includes(metric), `shared-channel metrics must expose ${metric}`);
}

const ccpRegistry = readExists('crates/sdkwork-im-ccp-registry/src/lib.rs');
assert.match(
  ccpRegistry,
  /retention_classes: \["ephemeral", "standard", "extended", "legal_hold"\]/,
  'ccp registry must publish canonical retention classes in governance vocabulary',
);

const domainRetention = readExists('crates/im-domain-core/src/retention.rs');
assert.match(
  domainRetention,
  /CANONICAL_RETENTION_CLASSES/,
  'im-domain-core must define canonical retention classes for governance alignment',
);

const commercialGates = readExists('.github/workflows/im-commercial-gates.yml');
for (const required of [
  'cargo test -p projection-service --test timeline_projection_test retention',
  'cargo test -p ops-service --test http_smoke_test retention',
  'cargo test -p social-service shared_channel_sync',
  'cargo test -p social-service --test http_smoke_test',
  'ensure_im_service_process_identity',
  'cargo test -p im-domain-core retention',
  'cargo test -p im-adapters-postgres-journal retention',
]) {
  assert.ok(commercialGates.includes(required), `im-commercial-gates.yml must run ${required}`);
}

const retentionMetrics = readExists('adapters/postgres-journal/src/retention_metrics.rs');
for (const metric of [
  'im_retention_purge_batches_total',
  'im_retention_purge_skipped_lock_total',
  'im_retention_purge_failures_total',
  'im_retention_purge_rows_deleted_total',
  'message_media_refs',
  'im_retention_purge_last_duration_seconds',
]) {
  assert.ok(retentionMetrics.includes(metric), `retention metrics must expose ${metric}`);
}

process.stdout.write('sdkwork-im retention enforcement standard passed\n');

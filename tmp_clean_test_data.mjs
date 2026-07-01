import { Client } from 'pg';

const c = new Client({
  connectionString:
    'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev?sslmode=disable',
});

await c.connect();

// Clean test-polluted data left by previous runs with the buggy commit_offset logic.
// Only delete rows tied to test tenants/conversations; never touch production data.
const tables = [
  'im_conversation_messages',
  'im_conversation_seq_counters',
  'im_outbox_events',
  'im_commit_journal',
  'im_projection_conversation_members',
  'im_projection_conversation_summaries',
  'im_projection_read_cursors',
  'im_projection_timeline_entries',
  'im_projection_direct_chat_bindings',
  'im_projection_metadata_snapshots',
  'im_idempotency_keys',
  'im_message_pins',
  'im_message_reactions',
];

for (const t of tables) {
  const r = await c.query(`delete from ${t}`);
  console.log(`${t}: deleted ${r.rowCount}`);
}

await c.end();
console.log('cleanup done');

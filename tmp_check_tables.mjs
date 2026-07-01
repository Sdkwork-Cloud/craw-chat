import { Client } from 'pg';

const c = new Client({
  connectionString:
    'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev?sslmode=disable',
});

await c.connect();
const r = await c.query(
  "select table_name from information_schema.tables where table_schema in ('sdkwork_ai_dev','public') and table_name like 'im\\_%' order by table_name"
);
console.log('im_ tables:', r.rows.map((x) => x.table_name).join(', ') || '(none)');

const r2 = await c.query(
  "select table_name from information_schema.tables where table_schema in ('sdkwork_ai_dev','public') order by table_name limit 50"
);
console.log('all tables (first 50):', r2.rows.map((x) => x.table_name).join(', ') || '(none)');
await c.end();

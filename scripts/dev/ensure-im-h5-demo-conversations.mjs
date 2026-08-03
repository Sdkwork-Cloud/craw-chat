#!/usr/bin/env node
// Provisions IM H5 demo conversation data through the real API write path.
//
// Conversations, members, messages, and favorites are event-sourced by the
// conversation service; creating them through the HTTP API keeps counters,
// payload hashes, search vectors, and the commit journal consistent. The
// script is idempotent: conversation creation uses fixed client request keys
// and messages are only posted into conversations that have none.
//
// Prerequisites:
//   1. The standalone gateway is running (pnpm gateway:run:standalone).
//   2. Demo users exist (node scripts/dev/ensure-im-h5-demo-users.mjs).
//   3. The IM demo seed was applied (pnpm db:seed).
//
// Usage:
//   node scripts/dev/ensure-im-h5-demo-conversations.mjs [--base-url http://127.0.0.1:18079]

import crypto from 'node:crypto';
import path from 'node:path';
import fs from 'node:fs';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { createDevBootstrapAccessTokenJwt } from '../../../sdkwork-iam/scripts/dev/create-dev-bootstrap-access-token-env.mjs';
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const DEFAULT_BASE_URL = 'http://127.0.0.1:18079';

const DEMO_PASSWORD = {
  owner: 'Owner#2026',
  alice: 'Alice#2026',
  bob: 'Bob#2026',
  guest: 'Guest#2026',
  grace: 'Grace#2026',
};

const CONVERSATIONS = [
  {
    key: 'demo-create-owner-alice',
    kind: 'direct',
    members: ['alice'],
    messages: [
      { from: 'owner', text: '你好李婷，我是张明，很高兴认识你' },
      { from: 'alice', text: '你好张明！项目周报我已经更新好了' },
      { from: 'owner', text: '明天下午三点开评审会，你有空吗' },
      { from: 'alice', text: '有空，设计稿已经上传到共享盘了' },
      { from: 'owner', text: '好的，我确认一下评审会议室' },
      { from: 'alice', text: '对了，新版首页的原型图也一起评审吧' },
      { from: 'owner', text: '没问题，下午见' },
    ],
    favorite: { title: '项目周报更新', preview: '你好张明！项目周报我已经更新好了' },
  },
  {
    key: 'demo-create-owner-bob',
    kind: 'direct',
    members: ['bob'],
    messages: [
      { from: 'bob', text: '老张，服务端的问题定位到了' },
      { from: 'owner', text: '辛苦王强了，具体是什么原因' },
      { from: 'bob', text: '内存泄漏，重启后已经修复了' },
      { from: 'owner', text: '好的，记得补一下回归测试' },
    ],
  },
  {
    key: 'demo-create-owner-guest',
    kind: 'direct',
    members: ['guest'],
    messages: [
      { from: 'guest', text: '你好，我是访客，请多关照' },
      { from: 'owner', text: '你好，欢迎体验 Sdkwork IM' },
      { from: 'guest', text: '功能很完整，界面也很清爽' },
    ],
  },
  {
    key: 'demo-create-owner-grace',
    kind: 'direct',
    members: ['grace'],
    messages: [
      { from: 'owner', text: '陈静你好，我是张明，欢迎加入团队' },
    ],
  },
  {
    key: 'demo-create-group-product',
    kind: 'group',
    groupName: '产品研发群',
    members: ['alice', 'bob', 'grace'],
    messages: [
      { from: 'owner', text: '大家好，我是张明，产品研发群正式成立' },
      { from: 'alice', text: '欢迎大家，我是李婷，负责产品设计' },
      { from: 'bob', text: '我是王强，负责服务端研发' },
      { from: 'grace', text: '我是陈静，负责测试' },
      { from: 'owner', text: '每周五下午同步进度，周报模板发在群里了' },
      { from: 'bob', text: '收到，本周排期下午发出来' },
    ],
  },
];

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function parseArgs(argv) {
  const settings = { baseUrl: DEFAULT_BASE_URL, reset: false };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--reset') {
      settings.reset = true;
      continue;
    }
    if (arg === '--base-url') {
      settings.baseUrl = normalizeText(argv[index + 1]) ?? DEFAULT_BASE_URL;
      index += 1;
      continue;
    }
    if (arg === '--help' || arg === '-h') {
      settings.help = true;
      continue;
    }
    throw new Error(`unknown argument: ${arg}`);
  }
  return settings;
}

function bootstrapAccessToken() {
  return createDevBootstrapAccessTokenJwt({
    appId: 'sdkwork-im-pc',
    environment: 'development',
  });
}

async function registerDemoUser({ baseUrl, accessToken, username }) {
  const response = await fetch(`${baseUrl}/app/v3/api/auth/registrations`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      'access-token': accessToken,
    },
    body: JSON.stringify({
      username,
      password: DEMO_PASSWORD[username],
      confirmPassword: DEMO_PASSWORD[username],
      email: `${username}@sdkwork.local`,
    }),
  });
  if (response.ok) {
    return 'registered';
  }
  const body = await response.json().catch(() => ({}));
  const code = String(body?.code ?? '');
  if (response.status === 400 && (code.includes('exists') || code.includes('conflict'))) {
    return 'exists';
  }
  if (response.status === 409) {
    return 'exists';
  }
  throw new Error(`register ${username} failed (${response.status}): ${JSON.stringify(body).slice(0, 300)}`);
}

async function login({ baseUrl, accessToken, username }) {
  const response = await fetch(`${baseUrl}/app/v3/api/auth/sessions`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      'access-token': accessToken,
    },
    body: JSON.stringify({
      grantType: 'password',
      username,
      password: DEMO_PASSWORD[username],
    }),
  });
  if (!response.ok) {
    const detail = await response.text().catch(() => '');
    throw new Error(`login ${username} failed (${response.status}): ${detail.slice(0, 300)}`);
  }
  const session = (await response.json()).data ?? {};
  return {
    accessToken: session.accessToken,
    authToken: session.authToken,
    userId: session.user?.id,
  };
}

function imHeaders(session) {
  return {
    'content-type': 'application/json',
    authorization: `Bearer ${session.authToken}`,
    'access-token': session.accessToken,
  };
}

function unwrapData(body) {
  return body?.data ?? body;
}

async function imRequest({ baseUrl, session, method, pathname, body, idempotencyKey }) {
  const serialized = body === undefined ? undefined : JSON.stringify(body);
  const headers = imHeaders(session);
  if (idempotencyKey) {
    headers['idempotency-key'] = idempotencyKey;
    if (serialized !== undefined) {
      // Idempotent commands with a body must pin a content fingerprint.
      headers['x-idempotency-fingerprint'] = crypto
        .createHash('sha256')
        .update(serialized)
        .digest('hex');
    }
  }
  const response = await fetch(`${baseUrl}/im/v3/api${pathname}`, {
    method,
    headers,
    body: serialized,
  });
  if (!response.ok) {
    const detail = await response.text().catch(() => '');
    throw new Error(`IM ${method} ${pathname} failed (${response.status}): ${detail.slice(0, 300)}`);
  }
  if (response.status === 204) {
    return undefined;
  }
  return response.json();
}

async function ensureConversation({ baseUrl, session, spec }) {
  if (spec.kind === 'group') {
    try {
      const result = await imRequest({
        baseUrl,
        session,
        method: 'POST',
        pathname: '/chat/conversations',
        body: {
          clientRequestKey: spec.key,
          conversationType: 'group',
          groupName: spec.groupName,
          memberUserIds: spec.members,
        },
      });
      const created = unwrapData(result).item ?? unwrapData(result);
      return created.conversationId ?? created.conversation?.conversationId ?? created.id;
    } catch (error) {
      // After a gateway restart the in-memory idempotency record is gone and
      // the create replays as a 409; the conflict detail embeds the
      // server-derived group id (`g_...`), which is the conversation to reuse.
      const groupId = String(error.message).match(/(?:#|id: )(g_[A-Za-z0-9_]+)/)
        ?? String(error.message).match(/#(g_[A-Za-z0-9_]+)/);
      if (groupId) {
        return groupId[1];
      }
      throw error;
    }
  }
  // Direct conversations accept a client-supplied id; members are attached
  // afterwards through the member add endpoint.
  const conversationId = `demo-${spec.key}`;
  try {
    await imRequest({
      baseUrl,
      session,
      method: 'POST',
      pathname: '/chat/conversations',
      body: {
        conversationId,
        conversationType: 'direct',
      },
    });
  } catch (error) {
    if (!String(error.message).includes('already exists')
      && !String(error.message).includes('conflicts with existing conversation id')) {
      throw error;
    }
  }
  const membersPage = await imRequest({
    baseUrl,
    session,
    method: 'GET',
    pathname: `/chat/conversations/${conversationId}/members?page_size=100`,
  }).catch(() => ({ items: [] }));
  const existingMemberIds = new Set(
    (unwrapData(membersPage).items ?? []).map((member) => member.principalId ?? member.id),
  );
  for (const memberUserId of spec.members) {
    if (existingMemberIds.has(memberUserId)) {
      continue;
    }
    await imRequest({
      baseUrl,
      session,
      method: 'POST',
      pathname: `/chat/conversations/${conversationId}/members/add`,
      body: {
        principalId: memberUserId,
        principalKind: 'user',
        role: 'member',
      },
    }).catch((error) => {
      if (existingMemberIds.has(memberUserId) || String(error.message).includes('already')) {
        return;
      }
      throw error;
    });
  }
  return conversationId;
}

async function ensureMessages({ baseUrl, session, conversationId, spec, usernameForUserId }) {
  const page = await imRequest({
    baseUrl,
    session,
    method: 'GET',
    pathname: `/chat/conversations/${conversationId}/messages?page_size=1`,
  });
  const pageData = unwrapData(page);
  if ((pageData.items?.length ?? 0) > 0) {
    return 'existing';
  }
  for (let index = 0; index < spec.messages.length; index += 1) {
    const message = spec.messages[index];
    const sender = session.userId === message.from ? session : await login({
      baseUrl,
      accessToken: bootstrapAccessToken(),
      username: usernameForUserId(message.from),
    });
    await imRequest({
      baseUrl,
      session: sender,
      method: 'POST',
      pathname: `/chat/conversations/${conversationId}/messages`,
      body: {
        text: message.text,
        parts: [{ kind: 'text', text: message.text }],
        clientMsgId: `demo-msg-${conversationId}-${index + 1}`,
      },
    });
  }
  return 'posted';
}

async function ensureFavorite({ baseUrl, session, conversationId, spec }) {
  if (!spec.favorite) {
    return 'skipped';
  }
  const page = await imRequest({
    baseUrl,
    session,
    method: 'GET',
    pathname: `/chat/conversations/${conversationId}/messages?page_size=50`,
  });
  const pageData = unwrapData(page);
  const target = pageData.items?.find((item) => {
    const textPart = item.body?.parts?.find((part) => part.kind === 'text');
    return textPart?.text === spec.favorite.preview || item.summary === spec.favorite.preview;
  });
  if (!target) {
    return 'no-target';
  }
  await imRequest({
    baseUrl,
    session,
    method: 'POST',
    pathname: `/chat/messages/${target.messageId}/favorites`,
    idempotencyKey: `demo-favorite-${conversationId}-${target.messageId}`,
    body: {
      conversationId,
      contentPreview: spec.favorite.preview,
      favoriteType: 'chat',
      title: spec.favorite.title,
      sourceDisplayName: '张明',
    },
  });
  return 'favorited';
}

async function resolveDemoUserIds() {
  // Resolve IM principals from IAM users by login name (read-only lookup).
  const envText = fs.readFileSync(path.join(__dirname, '..', '..', '.env.postgres'), 'utf8');
  const env = Object.fromEntries(
    envText
      .split(/\r?\n/u)
      .map((line) => line.trim())
      .filter((line) => line && !line.startsWith('#'))
      .map((line) => {
        const separator = line.indexOf('=');
        return [line.slice(0, separator), line.slice(separator + 1)];
      }),
  );
  const { default: pg } = await import('pg');
  const client = new pg.Client({
    host: env.SDKWORK_DATABASE_HOST ?? '127.0.0.1',
    port: Number(env.SDKWORK_DATABASE_PORT ?? 5432),
    user: env.SDKWORK_DATABASE_USERNAME ?? 'sdkwork_ai_dev',
    password: env.SDKWORK_DATABASE_PASSWORD,
    database: env.SDKWORK_DATABASE_NAME ?? 'sdkwork_ai_dev',
  });
  await client.connect();
  try {
    await client.query(`SET search_path TO ${env.SDKWORK_DATABASE_SCHEMA ?? 'sdkwork_ai_dev'}`);
    const result = await client.query(
      'SELECT username, id FROM iam_user WHERE username = ANY($1) AND is_deleted = 0',
      [Object.keys(DEMO_PASSWORD)],
    );
    return new Map(result.rows.map((row) => [row.username, row.id]));
  } finally {
    await client.end();
  }
}

async function resetDemoConversations() {
  // Conversation state is event-sourced hot state; a fresh gateway process
  // must observe the demo conversations through the write path. Reset removes
  // the demo rows (including the immutable commit journal) so the script can
  // recreate everything deterministically.
  //
  // Demo conversation ids are discovered from the journal footprint instead of
  // pattern-matched by id: direct conversations use client ids (`demo-...`),
  // but group conversations get a server-derived canonical `g_...` id that the
  // script cannot compute. Every demo group carries `demo-msg-...` message
  // idempotency keys in the journal, which exposes the group id reliably.
  const envText = fs.readFileSync(path.join(__dirname, '..', '..', '.env.postgres'), 'utf8');
  const env = Object.fromEntries(
    envText
      .split(/\r?\n/u)
      .map((line) => line.trim())
      .filter((line) => line && !line.startsWith('#'))
      .map((line) => {
        const separator = line.indexOf('=');
        return [line.slice(0, separator), line.slice(separator + 1)];
      }),
  );
  const { default: pg } = await import('pg');
  const client = new pg.Client({
    host: env.SDKWORK_DATABASE_HOST ?? '127.0.0.1',
    port: Number(env.SDKWORK_DATABASE_PORT ?? 5432),
    user: env.SDKWORK_DATABASE_USERNAME ?? 'sdkwork_ai_dev',
    password: env.SDKWORK_DATABASE_PASSWORD,
    database: env.SDKWORK_DATABASE_NAME ?? 'sdkwork_ai_dev',
  });
  await client.connect();
  try {
    await client.query(`SET search_path TO ${env.SDKWORK_DATABASE_SCHEMA ?? 'sdkwork_ai_dev'}`);
    const discovered = await client.query(`
      SELECT DISTINCT conversation_id FROM (
        SELECT conversation_id FROM im_conversations
        WHERE conversation_id LIKE 'demo-%'
        UNION
        SELECT aggregate_id AS conversation_id FROM im_commit_journal
        WHERE aggregate_id LIKE 'demo-%'
           OR event_id LIKE 'evt_demo-%'
           OR event_id LIKE 'evt_cm_demo-%'
           OR idempotency_key LIKE 'demo-%'
      ) demo_ids`);
    const ids = discovered.rows.map((row) => row.conversation_id);
    if (ids.length === 0) {
      console.log('[demo-conversations] reset demo conversations (DB): nothing to reset');
      return;
    }
    const deleteBy = async (table, column) => {
      const result = await client.query(`DELETE FROM ${table} WHERE ${column} = ANY($1)`, [ids]);
      return result.rowCount;
    };
    const counts = [];
    counts.push(['members', await deleteBy('im_conversation_members', 'conversation_id')]);
    counts.push(['messages', await deleteBy('im_conversation_messages', 'conversation_id')]);
    counts.push(['seq_counters', await deleteBy('im_conversation_seq_counters', 'conversation_id')]);
    counts.push(['read_cursors', await deleteBy('im_conversation_read_cursors', 'conversation_id')]);
    counts.push(['agent_assignments', await deleteBy('im_conversation_agent_assignments', 'conversation_id')]);
    counts.push(['conversations', await deleteBy('im_conversations', 'conversation_id')]);
    counts.push(['commit_journal', await deleteBy('im_commit_journal', 'aggregate_id')]);
    counts.push(['outbox_events', await deleteBy('im_outbox_events', 'aggregate_id')]);
    await client.query("DELETE FROM im_idempotency_keys WHERE idempotency_key LIKE '%demo%' OR request_scope LIKE '%demo%'");
    const summary = counts.map(([table, count]) => `${table}=${count}`).join(', ');
    console.log(`[demo-conversations] reset demo conversations (DB): ${summary}, idempotency_keys=cleared`);
  } finally {
    await client.end();
  }
}

async function main() {
  const settings = parseArgs(process.argv.slice(2));
  if (settings.help) {
    console.log('Provision IM H5 demo conversations through the API.');
    console.log('Usage: node scripts/dev/ensure-im-h5-demo-conversations.mjs [--reset] [--base-url URL]');
    console.log('  --reset  delete demo conversations (DB) and recreate them through the API');
    return;
  }

  if (settings.reset) {
    await resetDemoConversations();
  }

  const userIds = await resolveDemoUserIds();
  for (const username of Object.keys(DEMO_PASSWORD)) {
    if (!userIds.has(username)) {
      throw new Error(`demo user ${username} is missing; run ensure-im-h5-demo-users.mjs first`);
    }
  }

  const accessToken = bootstrapAccessToken();
  for (const username of Object.keys(DEMO_PASSWORD)) {
    const outcome = await registerDemoUser({ baseUrl: settings.baseUrl, accessToken, username });
    console.log(`[demo-conversations] user ${username}: ${outcome}`);
  }

  const owner = await login({ baseUrl: settings.baseUrl, accessToken, username: 'owner' });
  console.log(`[demo-conversations] logged in as owner (${owner.userId})`);

  const resolveUserId = (username) => {
    const userId = userIds.get(username);
    if (!userId) {
      throw new Error(`unknown demo user ${username}`);
    }
    return userId;
  };
  const usernameByUserId = new Map([...userIds].map(([username, userId]) => [userId, username]));
  const usernameForUserId = (userId) => {
    const username = usernameByUserId.get(userId);
    if (!username) {
      throw new Error(`unknown demo user id ${userId}`);
    }
    return username;
  };

  for (const spec of CONVERSATIONS) {
    const resolvedSpec = {
      ...spec,
      members: spec.members.map(resolveUserId),
      messages: spec.messages.map((message) => ({ ...message, from: resolveUserId(message.from) })),
    };
    const conversationId = await ensureConversation({ baseUrl: settings.baseUrl, session: owner, spec: resolvedSpec });
    const messagesOutcome = await ensureMessages({ baseUrl: settings.baseUrl, session: owner, conversationId, spec: resolvedSpec, usernameForUserId });
    const favoriteOutcome = await ensureFavorite({ baseUrl: settings.baseUrl, session: owner, conversationId, spec: resolvedSpec });
    console.log(`[demo-conversations] ${spec.key}: ${conversationId} (messages=${messagesOutcome}, favorite=${favoriteOutcome})`);
  }
}

main().catch((error) => {
  console.error(`[demo-conversations] failed: ${error.message}`);
  process.exit(1);
});

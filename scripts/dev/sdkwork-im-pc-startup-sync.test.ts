import assert from 'node:assert/strict';
import { createSdkworkImSyncCoordinatorService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ImSyncCoordinatorService';

const calls: Array<{
  method: string;
}> = [];

async function main(): Promise<void> {
  const service = createSdkworkImSyncCoordinatorService({
    chatService: {
      async syncOfflineMessages() {
        calls.push({ method: 'chat.syncOfflineMessages' });
        return {
          appliedMessages: 2,
          refreshedChats: 1,
        };
      },
    },
    contactService: {
      async syncContacts() {
        calls.push({ method: 'contact.syncContacts' });
        return {
          contacts: [],
          refreshedContacts: 0,
        };
      },
    },
    groupService: {
      async syncGroupMembers() {
        calls.push({ method: 'group.syncGroupMembers' });
        return [];
      },
    },
  });

  const result = await service.syncStartup();

  assert.deepEqual(
    calls,
    [
      { method: 'chat.syncOfflineMessages' },
      { method: 'contact.syncContacts' },
      { method: 'group.syncGroupMembers' },
    ],
    'startup sync must refresh IM state through chat, contact, and group SDK windows without IM device feed state',
  );
  assert.deepEqual(result.chat, {
    appliedMessages: 2,
    refreshedChats: 1,
  });
  assert.deepEqual(result.contacts, {
    contacts: [],
    refreshedContacts: 0,
  });
  assert.deepEqual(result.groups, []);
  assert.deepEqual(result.recoveredRtcSessions, []);
  assert.equal(result.errors.length, 0);

  console.log('sdkwork-im-pc startup sync orchestration contract passed');
}

void main();

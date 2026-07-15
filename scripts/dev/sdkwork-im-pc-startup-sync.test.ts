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
          appliedMessages: 0,
          refreshedChats: 1,
        };
      },
    },
    contactService: {
      async syncContacts() {
        calls.push({ method: 'contact.syncContacts' });
        throw new Error('startup must not enumerate contact pages');
      },
    },
  });

  const result = await service.syncStartup();

  assert.deepEqual(
    calls,
    [
      { method: 'chat.syncOfflineMessages' },
    ],
    'startup sync must refresh chat inbox metadata without preloading contacts or every group member list',
  );
  assert.deepEqual(result.chat, {
    appliedMessages: 0,
    refreshedChats: 1,
  });
  assert.equal(result.contacts, undefined);
  assert.deepEqual(result.recoveredRtcSessions, []);
  assert.equal(result.errors.length, 0);

  console.log('sdkwork-im-pc startup sync orchestration contract passed');
}

void main();

import assert from 'node:assert/strict';
import type {
  ContactPreferencesView,
  ContactRecommendationView,
  ContactTagView,
  ContactView,
  CreateContactRecommendationRequest,
  CreateContactTagRequest,
  ImSdkClient,
  UpdateContactPreferencesRequest,
  UpdateContactTagRequest,
} from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ContactService';

const preferenceReads: string[] = [];
const preferenceUpdates: Array<{
  targetUserId: string;
  body: UpdateContactPreferencesRequest;
}> = [];
const userSearchCalls: Array<{ pageSize?: number; q?: string }> = [];
const userBlockCreates: Array<{ blockedUserId: string; scope: string }> = [];
const tagCreates: CreateContactTagRequest[] = [];
const tagUpdates: Array<{
  tagId: string;
  body: UpdateContactTagRequest;
}> = [];
const tagDeletes: string[] = [];
const recommendationCreates: Array<{
  targetUserId: string;
  body: CreateContactRecommendationRequest;
}> = [];

const contactItems: ContactView[] = [
  {
    conversationId: 'c_direct_contact_alice',
    contactType: 'friendship',
    directChatId: 'dc_contact_alice',
    establishedAt: '2026-06-04T00:00:00.000Z',
    friendshipId: 'fs_contact_alice',
    lastInteractionAt: '2026-06-04T00:00:00.000Z',
    ownerUserId: 'current-user',
    relationshipState: 'active',
    targetUserId: 'u_alice',
    tenantId: '100001',
  },
  {
    contactType: 'friendship',
    establishedAt: '2026-06-04T00:00:00.000Z',
    friendshipId: 'fs_contact_bob',
    lastInteractionAt: '2026-06-04T00:00:00.000Z',
    ownerUserId: 'current-user',
    relationshipState: 'active',
    targetUserId: 'u_bob',
    tenantId: '100001',
  },
];

const contactPreferences = new Map<string, ContactPreferencesView>([
  [
    'u_alice',
    {
      isBlocked: false,
      isStarred: true,
      ownerUserId: 'current-user',
      remark: 'Alice Ops',
      targetUserId: 'u_alice',
      tenantId: '100001',
      updatedAt: '2026-06-04T00:00:00.000Z',
    },
  ],
  [
    'u_bob',
    {
      isBlocked: true,
      isStarred: false,
      ownerUserId: 'current-user',
      remark: 'Blocked Bob',
      targetUserId: 'u_bob',
      tenantId: '100001',
      updatedAt: '2026-06-04T00:00:00.000Z',
    },
  ],
]);

const contactTags = new Map<string, ContactTagView>([
  [
    'tag_family',
    {
      bg: 'bg-red-500/10',
      border: 'border-red-500/20',
      color: 'bg-red-500',
      count: 2,
      createdAt: '2026-06-04T00:00:00.000Z',
      name: 'Family',
      ownerUserId: 'current-user',
      tagId: 'tag_family',
      tenantId: '100001',
      updatedAt: '2026-06-04T00:00:00.000Z',
    },
  ],
]);

const fakeClient = {
  chat: {
    contacts: {
      async list() {
        return {
          hasMore: false,
          items: contactItems,
        };
      },
    },
  },
  social: {
    users: {
      async list(params: { pageSize?: number; q?: string }) {
        userSearchCalls.push(params);
        if (params.q === 'u_alice') {
          return {
            hasMore: false,
            items: [
              {
                avatarUrl: 'https://cdn.example.test/alice.png',
                displayName: 'Alice Chen',
                relationshipState: 'active',
                tenantId: '100001',
                userId: 'u_alice',
              },
            ],
          };
        }
        if (params.q === 'u_bob') {
          return {
            hasMore: false,
            items: [
              {
                avatarUrl: 'https://cdn.example.test/bob.png',
                displayName: 'Bob Stone',
                relationshipState: 'active',
                tenantId: '100001',
                userId: 'u_bob',
              },
            ],
          };
        }
        return {
          hasMore: false,
          items: [],
        };
      },
    },
    contacts: {
      async list() {
        return {
          hasMore: false,
          items: contactItems,
        };
      },
      recommendations: {
        async create(targetUserId: string, body: CreateContactRecommendationRequest) {
          recommendationCreates.push({ targetUserId, body });
          return {
            createdAt: '2026-06-04T00:00:01.000Z',
            ownerUserId: 'current-user',
            recommendationId: `rec_${targetUserId}`,
            targetConversationId: body.targetConversationId ?? '',
            targetUserId,
            tenantId: '100001',
          } satisfies ContactRecommendationView;
        },
      },
      preferences: {
        async retrieve(targetUserId: string) {
          preferenceReads.push(targetUserId);
          return contactPreferences.get(targetUserId) ?? {
            isBlocked: false,
            isStarred: false,
            ownerUserId: 'current-user',
            remark: '',
            targetUserId,
            tenantId: '100001',
            updatedAt: '2026-06-04T00:00:00.000Z',
          };
        },
        async update(targetUserId: string, body: UpdateContactPreferencesRequest) {
          preferenceUpdates.push({ targetUserId, body });
          const previous = contactPreferences.get(targetUserId) ?? {
            isBlocked: false,
            isStarred: false,
            ownerUserId: 'current-user',
            remark: '',
            targetUserId,
            tenantId: '100001',
            updatedAt: '2026-06-04T00:00:00.000Z',
          };
          const next = {
            ...previous,
            ...body,
            updatedAt: '2026-06-04T00:00:01.000Z',
          };
          contactPreferences.set(targetUserId, next);
          return next;
        },
      },
      tags: {
        async list() {
          return {
            hasMore: false,
            items: [...contactTags.values()],
          };
        },
        async create(body: CreateContactTagRequest) {
          tagCreates.push(body);
          const tag = {
            bg: body.bg ?? '',
            border: body.border ?? '',
            color: body.color,
            count: body.count ?? 0,
            createdAt: '2026-06-04T00:00:01.000Z',
            name: body.name,
            ownerUserId: 'current-user',
            tagId: 'tag_created',
            tenantId: '100001',
            updatedAt: '2026-06-04T00:00:01.000Z',
          };
          contactTags.set(tag.tagId, tag);
          return tag;
        },
        async update(tagId: string, body: UpdateContactTagRequest) {
          tagUpdates.push({ tagId, body });
          const previous = contactTags.get(tagId);
          if (!previous) {
            throw new Error(`missing tag ${tagId}`);
          }
          const next = {
            ...previous,
            ...body,
            updatedAt: '2026-06-04T00:00:02.000Z',
          };
          contactTags.set(tagId, next);
          return next;
        },
        async delete(tagId: string) {
          tagDeletes.push(tagId);
          contactTags.delete(tagId);
          return {
            deleted: true,
            tagId,
          };
        },
      },
    },
    userBlocks: {
      async create(body: { blockedUserId: string; scope: string }) {
        userBlockCreates.push(body);
        return {
          blockedUserId: body.blockedUserId,
          scope: body.scope,
          tenantId: '100001',
          userBlockId: `block_${body.blockedUserId}`,
        };
      },
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkContactService(() => fakeClient);
  const contacts = await service.getContacts();

  assert.deepEqual(
    preferenceReads,
    [],
    'contact list loading must not issue per-contact preference reads',
  );
  assert.deepEqual(
    contacts.map((contact) => contact.id),
    ['u_alice', 'u_bob'],
    'contact list loading must use the paged contact projection without filtering through per-contact preferences',
  );
  assert.equal(
    contacts[0]?.name,
    'u_alice',
    'contact list loading must not require per-contact remarks before rendering the first page',
  );
  assert.equal(
    contacts[0]?.conversationId,
    'c_direct_contact_alice',
    'contact list hydration must preserve the projected direct chat conversation id for chat opening',
  );
  assert.equal(
    contacts[0]?.directChatId,
    'dc_contact_alice',
    'contact list hydration must preserve the projected direct chat id for chat opening',
  );
  const cachedContact = await service.getUserById('u_alice');
  assert.equal(
    cachedContact?.conversationId,
    'c_direct_contact_alice',
    'contact lookup must keep the projected direct chat conversation id from the lightweight contact projection',
  );
  assert.deepEqual(
    userSearchCalls,
    [],
    'contact list and cached contact lookup must not resolve every contact through social user search',
  );

  const alicePreferences = await service.getContactPreferences('u_alice');
  assert.equal(alicePreferences.remark, 'Alice Ops');
  assert.deepEqual(
    preferenceReads,
    ['u_alice'],
    'contact preferences must be loaded only for an explicitly selected contact',
  );

  const starred = await service.getStarredContacts();
  assert.deepEqual(
    starred.map((contact) => contact.id),
    ['u_alice'],
    'starred contacts must come from locally loaded contact preference cache',
  );

  await service.toggleStarContact('u_alice', false);
  await service.setContactRemark('u_alice', '  Alice Lead  ');
  await service.addToBlacklist('u_alice');

  assert.deepEqual(
    userBlockCreates,
    [{ blockedUserId: 'u_alice', scope: 'all' }],
    'contact blacklist actions must create a durable user block through the generated IM SDK userBlocks API',
  );

  assert.deepEqual(
    preferenceUpdates,
    [
      { targetUserId: 'u_alice', body: { isStarred: false } },
      { targetUserId: 'u_alice', body: { remark: 'Alice Lead' } },
      { targetUserId: 'u_alice', body: { isBlocked: true, isStarred: false } },
    ],
    'contact star, remark, and blacklist actions must persist through the generated IM SDK contact preferences API',
  );

  const tags = await service.getTags();
  assert.deepEqual(
    tags.map((tag) => tag.id),
    ['tag_family'],
    'contact tags must be listed through the generated IM SDK contact tags API',
  );
  assert.equal(tags[0]?.name, 'Family');

  const createdTag = await service.addTag({
    bg: 'bg-blue-500/10',
    border: 'border-blue-500/20',
    color: 'bg-blue-500',
    count: 0,
    name: 'Classmates',
  });
  assert.equal(createdTag.id, 'tag_created');

  const updatedTag = await service.updateTag('tag_family', {
    count: 3,
    name: 'Family Team',
  });
  assert.equal(updatedTag.name, 'Family Team');
  assert.equal(updatedTag.count, 3);

  await service.removeTag('tag_family');
  await service.recommendToFriend('u_alice');

  assert.deepEqual(
    tagCreates,
    [
      {
        bg: 'bg-blue-500/10',
        border: 'border-blue-500/20',
        color: 'bg-blue-500',
        count: 0,
        name: 'Classmates',
      },
    ],
    'contact tag creation must persist through the generated IM SDK contact tags API',
  );
  assert.deepEqual(
    tagUpdates,
    [
      {
        tagId: 'tag_family',
        body: {
          count: 3,
          name: 'Family Team',
        },
      },
    ],
    'contact tag updates must persist through the generated IM SDK contact tags API',
  );
  assert.deepEqual(
    tagDeletes,
    ['tag_family'],
    'contact tag removal must persist through the generated IM SDK contact tags API',
  );
  assert.deepEqual(
    recommendationCreates,
    [
      {
        targetUserId: 'u_alice',
        body: {},
      },
    ],
    'contact recommendation actions must persist through the generated IM SDK contact recommendation API',
  );

  console.log('sdkwork-im-pc contact preferences sync contract passed');
}

void main();

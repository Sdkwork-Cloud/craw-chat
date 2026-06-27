> Migrated from `docs/sites/api-reference/im/media.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Media

<p class="api-page-intro">
  Media in Sdkwork IM is a message-domain usage contract. File upload, binary storage, object
  versioning, download authorization, and retention are owned by <code>sdkwork-drive</code>.
  IM carries standardized references to Drive nodes through <code>ContentPart.drive</code> and
  <code>MediaResource</code>.
</p>

<div class="api-link-list">
  <a href="/api-reference/im/messages"><code>Messages</code> Send media by placing a Drive-backed media part in the message body</a>
  <a href="/sdk/typescript-sdk"><code>@sdkwork/im-sdk</code> Message builders accept Drive references and MediaResource snapshots</a>
  <a href="/sdk/flutter-sdk"><code>im_sdk_generated</code> Flutter consumers use Drive first, then send IM message references</a>
</div>

## Authority Boundary

`sdkwork-drive` is the authority for file lifecycle work:

- create or choose the Drive space
- upload bytes into a Drive node
- resolve versions, permissions, previews, and download access
- enforce storage provider and tenant policies

Sdkwork IM does not create a parallel media storage system. Its IM contract only records how a
message uses a file that already exists in Drive.

The canonical Drive URI shape is:

```text
drive://spaces/{spaceId}/nodes/{nodeId}
```

## Message Contract

A media message part carries two related structures:

| Field | Role |
| --- | --- |
| `ContentPart.drive` | `DriveReference`, the authoritative Drive node pointer used for lifecycle and authorization |
| `ContentPart.resource` | `MediaResource`, the normalized usage snapshot used by clients, renderers, AI pipelines, and search |
| `ContentPart.mediaRole` | Optional semantic role such as `attachment`, `preview`, `voice`, `sticker`, or `generated_result` |

`DriveReference` is the durable identity:

```ts
type DriveReference = {
  driveUri: 'drive://spaces/{spaceId}/nodes/{nodeId}';
  spaceId: string;
  nodeId: string;
  nodeVersion?: string;
};
```

`MediaResource` is not storage identity. It is the standardized description of the media as it is
used inside the message:

```ts
type MediaResource = {
  id?: string;
  kind: 'image' | 'video' | 'audio' | 'voice' | 'document' | 'archive' | 'model' | 'other';
  source: 'provider_asset' | 'external_url' | 'data_url' | 'generated';
  uri?: string;
  fileName?: string;
  mimeType?: string;
  sizeBytes?: string;
  width?: number;
  height?: number;
  durationMillis?: number;
};
```

## Drive-Backed Example

Upload the binary through `sdkwork-drive`, then send the IM message with the returned Drive node
reference:

```ts
const drive = {
  driveUri: 'drive://spaces/space_app_upload_demo/nodes/node_storefront_png',
  spaceId: 'space_app_upload_demo',
  nodeId: 'node_storefront_png',
  nodeVersion: '1',
};

const message = sdk.createImageMessage({
  conversationId: 'conversation-1',
  drive,
  resource: {
    id: drive.nodeId,
    kind: 'image',
    source: 'provider_asset',
    uri: drive.driveUri,
    fileName: 'storefront.png',
    mimeType: 'image/png',
    sizeBytes: String(file.size),
  },
  mediaRole: 'attachment',
  text: 'Latest storefront concept',
  summary: 'Storefront concept',
});

await sdk.send(message);
```

The same body can be sent through route-aligned message submission:

```json
{
  "clientMsgId": "msg-client-001",
  "summary": "Storefront concept",
  "text": "Latest storefront concept",
  "parts": [
    {
      "kind": "media",
      "mediaRole": "attachment",
      "drive": {
        "driveUri": "drive://spaces/space_app_upload_demo/nodes/node_storefront_png",
        "spaceId": "space_app_upload_demo",
        "nodeId": "node_storefront_png",
        "nodeVersion": "1"
      },
      "resource": {
        "id": "node_storefront_png",
        "kind": "image",
        "source": "provider_asset",
        "uri": "drive://spaces/space_app_upload_demo/nodes/node_storefront_png",
        "fileName": "storefront.png",
        "mimeType": "image/png",
        "sizeBytes": "184220"
      }
    }
  ]
}
```

## Runtime Expectations

- IM message creation validates media parts as message content, not as file storage commands.
- Drive node visibility and retrieval are checked through `sdkwork-drive`.
- Message timelines preserve the Drive reference and the resource snapshot so clients can render,
  index, summarize, and audit media usage without owning file bytes.
- RTC recordings and AI-generated artifacts follow the same shape: Drive owns the file, IM or RTC
  surfaces carry `DriveReference` plus `MediaResource`.



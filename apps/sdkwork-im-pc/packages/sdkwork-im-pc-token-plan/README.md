# @sdkwork/im-pc-token-plan

Thin IM host adapter for the shared Token Plan catalog and Membership default checkout.

| Concern | Owner |
| --- | --- |
| Membership catalog, default checkout adapter, reservation, entitlement refresh | `sdkwork-membership` |
| QR payment dialog, polling, retry, completion state | `sdkwork-order` |
| IM screen composition, notification bridge, membership refresh | `@sdkwork/im-pc-token-plan` |
| IAM session and shared TokenManager | `@sdkwork/im-pc-core` |

The adapter does not create SDK clients, issue HTTP requests, or manage payment providers.

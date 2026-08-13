/**
 * H5 SDK client construction.
 *
 * Generated TypeScript app SDK clients are constructed here in bootstrap/core
 * code and injected into services or providers. UI packages MUST NOT construct
 * raw HTTP calls, manual auth headers, or generated SDK clients.
 */

import {
  getDriveAppSdkClient,
  initImSdkClient,
  resetImSdkClient,
  initDriveAppSdkClient,
  resetDriveAppSdkClient,
  createDriveAppSdkClientConfig,
  initOrderAppSdkClient,
  resetOrderAppSdkClient,
  createOrderAppSdkClientConfig,
  initAccountAppSdkClient,
  resetAccountAppSdkClient,
  createAccountAppSdkClientConfig,
  initKnowledgebaseAppSdkClient,
  resetKnowledgebaseAppSdkClient,
  createKnowledgebaseAppSdkClientConfig,
  initAgentsAppSdkClient,
  resetAgentsAppSdkClient,
  createAgentsAppSdkClientConfig,
  initVoiceAppSdkClient,
  resetVoiceAppSdkClient,
  createVoiceAppSdkClientConfig,
  initCmsAppSdkClient,
  resetCmsAppSdkClient,
  createCmsAppSdkClientConfig,
  type ImSdkClient,
  type CmsAppSdkClient,
  type SdkworkDriveAppClient,
  type SdkworkAppClient as SdkworkOrderAppClient,
  type SdkworkAccountAppClient,
  type SdkworkKnowledgebaseAppClient,
  type SdkworkAgentsAppClient,
  type SdkworkVoiceAppClient,
} from '@sdkwork/im-h5-core/sdk';
import {
  createCommunityAppSdkClient,
  createGeneratedCommunityAppSdkPort,
  type CommunityAppSdkClient,
} from '@sdkwork/community-runtime';
import { createClient as createFeedsOpenClient, type SdkworkCustomClient as SdkworkFeedsOpenClient } from '@sdkwork/feeds-sdk';
import type { SdkworkCommunityAppSdkPort } from '@sdkwork/community-sdk-ports';
import {
  createCourseAppSdkClient,
  createGeneratedCourseAppSdkPort,
  type CourseAppSdkClient as CourseAppSdkClientWrapper,
} from '@sdkwork/course-runtime';
import type { CourseAppSdkPort } from '@sdkwork/course-sdk-ports';
import {
  createCompanyAppSdkClient,
  createGeneratedCompanyAppSdkPort,
  type CompanyAppSdkClient as CompanyAppSdkClientWrapper,
} from '@sdkwork/company-runtime';
import type { SdkworkCompanyAppSdkPort } from '@sdkwork/company-sdk-ports';
import {
  initIamAppSdkClient,
  resetIamAppSdkClient,
  createIamAppSdkClientConfig,
  type SdkworkIamAppClient,
} from '@sdkwork/im-h5-core/sdk';
import {
  createNotaryH5ComposedApi,
  initNotaryH5AppSdkClient,
  resetNotaryH5SdkClients,
  type NotaryH5ComposedApi,
} from '@sdkwork/notary-h5-core';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { resolveH5RuntimeEnvironment } from './environment';
import { getTokenManagerBinding } from './tokenManager';

export interface H5SdkClientComposition {
  readonly driveAppSdkClient: SdkworkDriveAppClient;
  readonly imSdkClient: ImSdkClient;
  readonly orderAppSdkClient: SdkworkOrderAppClient;
  readonly accountAppSdkClient: SdkworkAccountAppClient;
  readonly iamAppSdkClient: SdkworkIamAppClient;
  readonly notaryAppSdkClient: ReturnType<typeof initNotaryH5AppSdkClient>;
  readonly notaryApi: NotaryH5ComposedApi;
  readonly knowledgebaseAppSdkClient: SdkworkKnowledgebaseAppClient;
  readonly agentsAppSdkClient: SdkworkAgentsAppClient;
  readonly voiceAppSdkClient: SdkworkVoiceAppClient;
  readonly cmsAppSdkClient: CmsAppSdkClient;
  readonly communityAppSdkClient: CommunityAppSdkClient;
  readonly communityAppSdkPort: SdkworkCommunityAppSdkPort;
  /** Standard feeds stream client (open surface, anonymous reads). */
  readonly feedsOpenSdkClient: SdkworkFeedsOpenClient;
  readonly courseAppSdkClient: CourseAppSdkClientWrapper;
  readonly courseAppSdkPort: CourseAppSdkPort;
  readonly companyAppSdkClient: CompanyAppSdkClientWrapper;
  readonly companyAppSdkPort: SdkworkCompanyAppSdkPort;
}

let sdkClientComposition: H5SdkClientComposition | null = null;



export function initSdkClients(
  tokenManager: AuthTokenManager = getTokenManagerBinding(),
): H5SdkClientComposition {
  if (sdkClientComposition) {
    return sdkClientComposition;
  }

  const environment = resolveH5RuntimeEnvironment();
  const imSdkClient = initImSdkClient({
    apiBaseUrl: environment.imApiBaseUrl,
    // Explicit realtime endpoint: without it the SDK derives the WebSocket URL
    // from the HTTP base and falls back to the frontend origin, which breaks
    // the CCP connection (no dev server proxies /im/v3/api/realtime/ws).
    websocketBaseUrl: environment.imWebsocketBaseUrl,
    platform: "h5",
    tokenManager,
  });

  const driveAppSdkClient = initDriveAppSdkClient(
    createDriveAppSdkClientConfig({
      baseUrl: environment.driveAppApiBaseUrl,
      tokenManager,
    }),
  );

  const orderAppSdkClient = initOrderAppSdkClient(
    createOrderAppSdkClientConfig({
      baseUrl: environment.orderAppApiBaseUrl,
      tokenManager,
    }),
  );

  const accountAppSdkClient = initAccountAppSdkClient(
    createAccountAppSdkClientConfig({
      baseUrl: environment.sdkGatewayApiBaseUrl,
      tokenManager,
    }),
  );

  const iamAppSdkClient = initIamAppSdkClient(
    createIamAppSdkClientConfig({
      baseUrl: environment.iamApiBaseUrl,
      tokenManager,
    }),
  );

  const notaryAppSdkClient = initNotaryH5AppSdkClient({
    baseUrl: environment.sdkGatewayApiBaseUrl,
    authMode: 'dual-token',
    platform: 'h5',
    tokenManager,
  });
  const notaryApi = createNotaryH5ComposedApi({
    drive: driveAppSdkClient,
    appbase: {},
  });

  const knowledgebaseAppSdkClient = initKnowledgebaseAppSdkClient(
    createKnowledgebaseAppSdkClientConfig({
      baseUrl: environment.knowledgebaseAppApiBaseUrl,
      tokenManager,
    }),
  );

  const agentsAppSdkClient = initAgentsAppSdkClient(
    createAgentsAppSdkClientConfig({
      baseUrl: environment.agentsAppApiBaseUrl,
      tokenManager,
    }),
  );

  const voiceAppSdkClient = initVoiceAppSdkClient(
    createVoiceAppSdkClientConfig({
      baseUrl: environment.voiceAppApiBaseUrl,
      tokenManager,
    }),
  );

  const cmsAppSdkClient = initCmsAppSdkClient(
    createCmsAppSdkClientConfig({
      baseUrl: environment.cmsAppApiBaseUrl,
      tokenManager,
    }),
  );

  const communityAppSdkClient = createCommunityAppSdkClient({
    config: {
      appApiBaseUrl: environment.imApiBaseUrl,
    },
    tokenManager,
  });
  const communityAppSdkPort = createGeneratedCommunityAppSdkPort(
    communityAppSdkClient.client,
  );

  // Standard feeds stream client: anonymous open-surface reads (moments feed,
  // community circle feeds, inspiration streams) go through the standard
  // feeds stream system; content write operations keep using the community
  // app SDK port. The base URL comes from the feeds gateway override
  // (SDKWORK_IM_H5_FEEDS_OPEN_API_BASE_URL, feeds standalone gateway in
  // standalone dev) or the platform gateway for cloud profiles.
  const feedsOpenSdkClient = createFeedsOpenClient({
    baseUrl: environment.feedsOpenApiBaseUrl,
    platform: 'h5',
  });

  const courseAppSdkClient = createCourseAppSdkClient({
    config: {
      appApiBaseUrl: environment.imApiBaseUrl,
    },
    tokenManager,
  });
  const courseAppSdkPort = createGeneratedCourseAppSdkPort(
    courseAppSdkClient.client,
  );

  const companyAppSdkClient = createCompanyAppSdkClient({
    config: {
      appApiBaseUrl: environment.companyAppApiBaseUrl,
    },
    tokenManager,
  });
  const companyAppSdkPort = createGeneratedCompanyAppSdkPort(
    companyAppSdkClient.client,
  );

  sdkClientComposition = {
    driveAppSdkClient,
    imSdkClient,
    orderAppSdkClient,
    accountAppSdkClient,
    iamAppSdkClient,
    notaryAppSdkClient,
    notaryApi,
    knowledgebaseAppSdkClient,
    agentsAppSdkClient,
    voiceAppSdkClient,
    cmsAppSdkClient,
    communityAppSdkClient,
    communityAppSdkPort,
    feedsOpenSdkClient,
    courseAppSdkClient,
    courseAppSdkPort,
    companyAppSdkClient,
    companyAppSdkPort,
  };
  return sdkClientComposition;
}

export function getSdkClients(): H5SdkClientComposition {
  return sdkClientComposition ?? initSdkClients();
}

export function getDriveAppSdkClientFromBootstrap(): SdkworkDriveAppClient {
  return getSdkClients().driveAppSdkClient;
}

export function resetSdkClients(): void {
  resetNotaryH5SdkClients();
  resetDriveAppSdkClient();
  resetOrderAppSdkClient();
  resetKnowledgebaseAppSdkClient();
  resetAgentsAppSdkClient();
  resetVoiceAppSdkClient();
  resetCmsAppSdkClient();
  resetImSdkClient();
  sdkClientComposition = null;
}

export type { SdkworkDriveAppClient };
export { getDriveAppSdkClient };

import assert from 'node:assert/strict';
import type { SdkworkImAppClient } from '@sdkwork/im-pc-core';
import {
  createSdkworkEnterpriseService,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-enterprise/src/services/EnterpriseService';

async function main(): Promise<void> {
  const homeCalls: string[] = [];
  const homeBackedClient = {
    portal: {
      home: {
        async retrieve() {
          homeCalls.push('portal.home.retrieve');
          return {
            enterpriseCatalog: {
              items: [
                {
                  enterpriseId: 'ent-real-1',
                  name: 'Real Enterprise',
                  logoUrl: 'https://cdn.example.test/enterprise.png',
                  industry: 'Industrial AI',
                  location: 'Shanghai',
                  size: '100-499',
                  description: 'Real backend enterprise catalog entry.',
                  tags: ['verified', 'ai'],
                  website: 'real.example.test',
                  verified: true,
                },
                {
                  id: '',
                  name: 'Invalid Enterprise',
                },
              ],
            },
          };
        },
      },
      workspace: {
        async retrieve() {
          throw new Error('workspace fallback should not be used when home catalog has items');
        },
      },
    },
  } as unknown as SdkworkImAppClient;

  const homeBackedService = createSdkworkEnterpriseService(() => homeBackedClient);
  const homeEnterprises = await homeBackedService.getEnterprises();

  assert.deepEqual(homeCalls, ['portal.home.retrieve']);
  assert.deepEqual(
    homeEnterprises,
    [
      {
        description: 'Real backend enterprise catalog entry.',
        id: 'ent-real-1',
        industry: 'Industrial AI',
        isVerified: true,
        location: 'Shanghai',
        logo: 'https://cdn.example.test/enterprise.png',
        name: 'Real Enterprise',
        size: '100-499',
        tags: ['verified', 'ai'],
        website: 'real.example.test',
      },
    ],
    'enterprise catalog must be parsed from the generated app SDK portal home snapshot without local mock entries',
  );

  const fallbackCalls: string[] = [];
  const workspaceFallbackClient = {
    portal: {
      home: {
        async retrieve() {
          fallbackCalls.push('portal.home.retrieve');
          return {};
        },
      },
      workspace: {
        async retrieve() {
          fallbackCalls.push('portal.workspace.retrieve');
          return {
            activeBrands: 2,
            name: 'Tenant Workspace',
            region: 'cn-south',
            seats: 42,
            slug: 'tenant-workspace',
            supportPlan: 'gold',
            tier: 'enterprise',
            uptime: '99.95%',
          };
        },
      },
    },
  } as unknown as SdkworkImAppClient;

  const fallbackService = createSdkworkEnterpriseService(() => workspaceFallbackClient);
  const fallbackEnterprises = await fallbackService.getEnterprises();

  assert.deepEqual(fallbackCalls, ['portal.home.retrieve', 'portal.workspace.retrieve']);
  assert.deepEqual(
    fallbackEnterprises,
    [
      {
        description: 'gold support, 2 active brands, uptime 99.95%',
        id: 'tenant-workspace',
        industry: 'enterprise',
        isVerified: true,
        location: 'cn-south',
        logo: '',
        name: 'Tenant Workspace',
        size: '42 seats',
        tags: ['enterprise', 'gold'],
        website: '',
      },
    ],
    'enterprise catalog must fall back to the real workspace snapshot when no directory catalog is returned',
  );

  console.log('sdkwork-im-pc enterprise catalog real-logic contract passed');
}

void main();

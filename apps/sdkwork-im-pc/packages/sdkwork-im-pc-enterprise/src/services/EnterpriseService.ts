import {
  getAppSdkClientWithSession,
  type SdkworkImAppClient,
} from '@sdkwork/im-pc-core/sdk/appSdkClient';
import type { EnterpriseData } from '../components/EnterpriseDetail';

type RecordLike = Record<string, unknown>;

function isRecord(value: unknown): value is RecordLike {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function asString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function asStringFrom(record: RecordLike, keys: string[]): string | undefined {
  for (const key of keys) {
    const value = asString(record[key]);
    if (value) {
      return value;
    }
  }
  return undefined;
}

function asBooleanFrom(record: RecordLike, keys: string[]): boolean | undefined {
  for (const key of keys) {
    if (typeof record[key] === 'boolean') {
      return record[key];
    }
  }
  return undefined;
}

function asStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }

  return value
    .map((item) => asString(item))
    .filter((item): item is string => Boolean(item));
}

function extractArray(value: unknown): unknown[] {
  if (Array.isArray(value)) {
    return value;
  }
  if (!isRecord(value)) {
    return [];
  }

  for (const key of ['items', 'data', 'enterprises', 'companies', 'records', 'list']) {
    if (Array.isArray(value[key])) {
      return value[key] as unknown[];
    }
  }

  return [];
}

function collectEnterpriseRecords(snapshot: unknown): RecordLike[] {
  if (!isRecord(snapshot)) {
    return [];
  }

  const records: RecordLike[] = [];
  for (const key of [
    'enterprises',
    'enterpriseCatalog',
    'enterprise_catalog',
    'companies',
    'companyCatalog',
    'company_catalog',
    'organizations',
    'organizationCatalog',
    'organization_catalog',
  ]) {
    for (const item of extractArray(snapshot[key])) {
      if (isRecord(item)) {
        records.push(item);
      }
    }
  }
  return records;
}

function normalizeEnterprise(record: RecordLike): EnterpriseData | undefined {
  const id = asStringFrom(record, [
    'id',
    'enterpriseId',
    'enterprise_id',
    'companyId',
    'company_id',
    'organizationId',
    'organization_id',
    'tenantId',
    'tenant_id',
    'slug',
  ]);
  const name = asStringFrom(record, ['name', 'displayName', 'display_name', 'companyName', 'company_name', 'title']);

  if (!id || !name) {
    return undefined;
  }

  return {
    id,
    name,
    logo: asStringFrom(record, ['logo', 'logoUrl', 'logo_url', 'avatar', 'avatarUrl', 'avatar_url']) ?? '',
    industry: asStringFrom(record, ['industry', 'sector', 'category', 'tier']) ?? '',
    location: asStringFrom(record, ['location', 'region', 'city', 'address']) ?? '',
    size: asStringFrom(record, ['size', 'staffSize', 'staff_size', 'employeeRange', 'employee_range', 'seats']) ?? '',
    description: asStringFrom(record, ['description', 'desc', 'summary', 'intro']) ?? '',
    tags: asStringArray(record.tags),
    website: asStringFrom(record, ['website', 'site', 'homepage', 'domain']) ?? '',
    isVerified: asBooleanFrom(record, ['isVerified', 'verified', 'is_verified']) ?? false,
  };
}

function uniqueEnterprises(records: RecordLike[]): EnterpriseData[] {
  const enterprises = new Map<string, EnterpriseData>();

  for (const record of records) {
    const enterprise = normalizeEnterprise(record);
    if (enterprise) {
      enterprises.set(enterprise.id, enterprise);
    }
  }

  return Array.from(enterprises.values());
}

function workspaceToEnterprise(workspace: unknown): EnterpriseData | undefined {
  if (!isRecord(workspace)) {
    return undefined;
  }

  const id = asStringFrom(workspace, ['slug', 'tenantId', 'tenant_id', 'organizationId', 'organization_id', 'name']);
  const name = asStringFrom(workspace, ['name', 'displayName', 'display_name']);

  if (!id || !name) {
    return undefined;
  }

  const tier = asString(workspace.tier);
  const supportPlan = asString(workspace.supportPlan);
  const activeBrands = typeof workspace.activeBrands === 'number' ? workspace.activeBrands : undefined;
  const uptime = asString(workspace.uptime);
  const seats = typeof workspace.seats === 'number' ? workspace.seats : undefined;

  return {
    id,
    name,
    logo: '',
    industry: tier ?? '',
    location: asString(workspace.region) ?? '',
    size: seats === undefined ? '' : `${seats} seats`,
    description: [
      supportPlan ? `${supportPlan} support` : undefined,
      activeBrands === undefined ? undefined : `${activeBrands} active brands`,
      uptime ? `uptime ${uptime}` : undefined,
    ]
      .filter(Boolean)
      .join(', '),
    tags: [tier, supportPlan].filter((tag): tag is string => Boolean(tag)),
    website: '',
    isVerified: true,
  };
}

export interface EnterpriseService {
  getEnterprises(): Promise<EnterpriseData[]>;
}

class SdkworkEnterpriseService implements EnterpriseService {
  constructor(private readonly getClient: () => SdkworkImAppClient = getAppSdkClientWithSession) {}

  async getEnterprises(): Promise<EnterpriseData[]> {
    const client = this.getClient();
    const homeSnapshot = await client.portal.home.retrieve();
    const enterprises = uniqueEnterprises(collectEnterpriseRecords(homeSnapshot));

    if (enterprises.length > 0) {
      return enterprises;
    }

    const workspace = workspaceToEnterprise(await client.portal.workspace.retrieve());
    return workspace ? [workspace] : [];
  }
}

export function createSdkworkEnterpriseService(getClient?: () => SdkworkImAppClient): EnterpriseService {
  return new SdkworkEnterpriseService(getClient);
}

export const enterpriseService = createSdkworkEnterpriseService();

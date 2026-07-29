import {
  getNotaryComposedApi,
  type NotaryComposedApi,
} from "@sdkwork/im-h5-core";
import {
  DEFAULT_LIST_PAGE_SIZE,
  MAX_LIST_PAGE_SIZE,
} from "@sdkwork/utils";

const MATTER_PAGE_SIZE = 50;
const STAFF_PAGE_SIZE = 50;

export type NotaryCaseStatus =
  | "PENDING_REVIEW"
  | "PROCESSING"
  | "COMPLETED"
  | "REJECTED"
  | "CANCELLED"
  | "CREATE_FAILED";

export type NotaryRecordFilter = "ALL" | NotaryCaseStatus;

export interface NotaryPageInfo {
  mode: "cursor";
  hasMore: boolean;
  nextCursor?: string;
}

export interface NotaryRecord {
  id: string;
  title: string;
  type: string;
  date: string;
  status: NotaryCaseStatus;
}

export interface NotaryRecordPage {
  records: NotaryRecord[];
  pageInfo: NotaryPageInfo;
}

export interface NotaryMatter {
  id: string;
  title: string;
  description?: string;
}

export interface NotaryMatterPage {
  matters: NotaryMatter[];
  pageInfo: NotaryPageInfo;
}

export interface NotaryStaffMember {
  id: string;
  name: string;
  organization: string;
  active: boolean;
  initial: string;
}

export interface NotaryStaffPage {
  staff: NotaryStaffMember[];
  pageInfo: NotaryPageInfo;
}

export interface NotaryFileTag {
  label: string;
  color: "blue" | "green" | "orange" | "red" | "gray";
}

export interface NotaryFile {
  id: string;
  name: string;
  size: string;
  uploadTime: string;
  fileType: "image" | "video" | "pdf" | "word" | "excel" | "zip" | "unknown";
  tags: NotaryFileTag[];
  uploader: string;
  previewUrl?: string;
  downloadUrl?: string;
}

export interface NotaryParty {
  id: string;
  name: string;
  role: string;
  status: "pending" | "verified" | "failed" | "expired";
  phone?: string;
  gender?: string;
  dob?: string;
  address?: string;
  idCard?: string;
  idStartDate?: string;
  idEndDate?: string;
}

export interface NotaryDetailData {
  id: string;
  title: string;
  time: string;
  item: string;
  notaryName: string;
  notaryNo: string;
  status: NotaryCaseStatus;
  remarks: string;
  parties: NotaryParty[];
  materials: NotaryFile[];
}

export interface NotaryRecordsStatistics {
  pendingReview: number;
  completedToday: number;
  anomaliesIntercepted: number;
  monthlyTotal: number;
}

export interface NotaryAccessSummary {
  visible: boolean;
  organizationVerified: boolean;
  businessEnabled: boolean;
  memberId: string;
  roles: string[];
  permissions: string[];
  reason?: string;
}

export interface NotaryPartyVideoInvite {
  inviteId: string;
  conversationId: string;
  inviteUrl: string;
  expiresAt: string;
}

export interface NotaryDraftParty {
  name: string;
  role?: string;
  idCard: string;
  phone?: string;
  gender?: string;
  dob?: string;
  address?: string;
  remarks?: string;
  idStartDate?: string;
  idEndDate?: string;
}

interface NotaryDraftAttachmentBase {
  id: string;
  name: string;
  file: File;
  size: string;
}

export type NotaryDraftAttachment =
  | (NotaryDraftAttachmentBase & {
    type: "image" | "video";
    previewUrl: string;
  })
  | (NotaryDraftAttachmentBase & {
    type: "file";
    previewUrl?: undefined;
  });

export interface CreateNotaryCaseDraft {
  skuId: string;
  title: string;
  applicantName: string;
  description: string;
  primaryNotaryMembershipId?: string;
  parties: NotaryDraftParty[];
  attachments: NotaryDraftAttachment[];
  idempotencyKey: string;
}

export interface NotaryApiPort {
  getAccess(): Promise<unknown>;
  listMatters(input?: unknown): Promise<unknown>;
  listStaff(input?: unknown): Promise<unknown>;
  listCases(input?: unknown): Promise<unknown>;
  getCase(caseId: string): Promise<unknown>;
  createCase(input: Parameters<NotaryComposedApi["createCase"]>[0]): Promise<unknown>;
  uploadCaseFile(input: Parameters<NotaryComposedApi["uploadCaseFile"]>[0]): Promise<unknown>;
  attachPartySignature(
    caseId: string,
    partyId: string,
    input: Parameters<NotaryComposedApi["attachPartySignature"]>[2],
  ): Promise<unknown>;
  createPartyVideoInvite(
    caseId: string,
    partyId: string,
    input?: Parameters<NotaryComposedApi["createPartyVideoInvite"]>[2],
  ): Promise<unknown>;
  getDashboardStatistics(): Promise<unknown>;
}

export class NotaryResponseContractError extends Error {
  readonly code = "NOTARY_RESPONSE_CONTRACT_INVALID";

  constructor(message: string) {
    super(message);
    this.name = "NotaryResponseContractError";
  }
}

export class NotaryCapabilityUnavailableError extends Error {
  readonly code = "NOTARY_CAPABILITY_UNAVAILABLE";

  constructor(capability: string) {
    super(`${capability} is not available through the Notary App SDK`);
    this.name = "NotaryCapabilityUnavailableError";
  }
}

const RECORD_TABS: ReadonlyArray<{
  id: NotaryRecordFilter;
  labelKey: string;
}> = [
  { id: "ALL", labelKey: "notary.records.tabs.all" },
  { id: "PENDING_REVIEW", labelKey: "notary.records.tabs.pending_review" },
  { id: "PROCESSING", labelKey: "notary.records.tabs.processing" },
  { id: "COMPLETED", labelKey: "notary.records.tabs.completed" },
  { id: "REJECTED", labelKey: "notary.records.tabs.rejected" },
  { id: "CANCELLED", labelKey: "notary.records.tabs.cancelled" },
  { id: "CREATE_FAILED", labelKey: "notary.records.tabs.create_failed" },
];

export function createNotaryService(
  resolveApi: () => NotaryApiPort = getNotaryComposedApi,
) {
  return {
    getRecordTabs() {
      return RECORD_TABS;
    },

    async getNotaryRecords(
      filter: NotaryRecordFilter,
      cursor?: string,
    ): Promise<NotaryRecordPage> {
      assertRecordFilter(filter);
      const response = await resolveApi().listCases({
        ...(filter !== "ALL" ? { status: filter } : {}),
        pageSize: DEFAULT_LIST_PAGE_SIZE,
        ...(cursor ? { cursor } : {}),
      });
      const page = readCursorPage(response, "notary case list");
      return {
        records: page.items.map(mapNotaryRecord),
        pageInfo: page.pageInfo,
      };
    },

    async getNotaryTypes(cursor?: string): Promise<NotaryMatterPage> {
      const response = await resolveApi().listMatters({
        pageSize: MATTER_PAGE_SIZE,
        ...(cursor ? { cursor } : {}),
      });
      const page = readCursorPage(response, "notary matter list");
      return {
        matters: page.items.map((item) => {
          const matter = asRecord(item);
          return {
            id: requireString(matter, "skuId", "notary matter"),
            title: requireString(matter, "title", "notary matter"),
            ...(optionalString(matter, "description")
              ? { description: optionalString(matter, "description") }
              : {}),
          };
        }),
        pageInfo: page.pageInfo,
      };
    },

    async getNotarySearchList(
      query = "",
      cursor?: string,
    ): Promise<NotaryStaffPage> {
      const response = await resolveApi().listStaff({
        staffRole: "notary",
        ...(query.trim() ? { q: query.trim() } : {}),
        pageSize: STAFF_PAGE_SIZE,
        ...(cursor ? { cursor } : {}),
      });
      const page = readCursorPage(response, "notary staff list");
      return {
        staff: page.items.map(mapNotaryStaff),
        pageInfo: page.pageInfo,
      };
    },

    async createCase(draft: CreateNotaryCaseDraft): Promise<NotaryRecord> {
      const skuId = requireNonEmpty(draft.skuId, "skuId");
      const title = requireNonEmpty(draft.title, "title");
      const applicantName = requireNonEmpty(draft.applicantName, "applicantName");
      const description = requireNonEmpty(draft.description, "description");
      const idempotencyKey = requireNonEmpty(draft.idempotencyKey, "idempotencyKey");
      if (draft.parties.length === 0) {
        throw new TypeError("At least one notary party is required");
      }

      const api = resolveApi();
      const created = await api.createCase({
        skuId,
        title,
        applicantName,
        description,
        primaryNotaryMembershipId: draft.primaryNotaryMembershipId,
        parties: draft.parties.map(mapDraftParty),
        idempotencyKey,
      });
      const createdCase = readResource(created, "created notary case");
      const caseId = requireString(createdCase, "id", "created notary case");

      // Upload sequentially to keep browser memory and outbound concurrency bounded.
      for (const attachment of draft.attachments) {
        await api.uploadCaseFile({
          caseId,
          file: attachment.file,
          category: "evidence",
          uploadIntentId: attachment.id,
          source: "sdkwork-im-h5-notary",
        });
      }

      return mapNotaryRecord(createdCase);
    },

    async getNotaryDetail(caseId: string): Promise<NotaryDetailData> {
      const normalizedCaseId = requireNonEmpty(caseId, "caseId");
      const response = await resolveApi().getCase(normalizedCaseId);
      return mapNotaryDetail(readResource(response, "notary case detail"));
    },

    async getRecordsStatistics(): Promise<NotaryRecordsStatistics> {
      const response = await resolveApi().getDashboardStatistics();
      const statistics = readResource(response, "notary dashboard statistics");
      return {
        pendingReview: readCount(statistics, "pendingReviewQueue"),
        completedToday: readCount(statistics, "todayCompleted"),
        anomaliesIntercepted: readCount(statistics, "anomalyIntercepted"),
        monthlyTotal: readCount(statistics, "monthlyPreservationTotal"),
      };
    },

    async getAccess(): Promise<NotaryAccessSummary> {
      const access = readResource(await resolveApi().getAccess(), "notary access");
      return {
        visible: requireBoolean(access, "visible", "notary access"),
        organizationVerified: requireBoolean(
          access,
          "organizationVerified",
          "notary access",
        ),
        businessEnabled: requireBoolean(
          access,
          "notaryBusinessEnabled",
          "notary access",
        ),
        memberId: requireString(access, "memberId", "notary access"),
        roles: optionalStringArray(access, "roles"),
        permissions: optionalStringArray(access, "permissions"),
        ...(optionalString(access, "reason") ? { reason: optionalString(access, "reason") } : {}),
      };
    },

    async attachPartySignature(
      caseId: string,
      partyId: string,
      file: File,
    ): Promise<void> {
      const normalizedCaseId = requireNonEmpty(caseId, "caseId");
      const normalizedPartyId = requireNonEmpty(partyId, "partyId");
      if (!(file instanceof File) || file.size === 0) {
        throw new TypeError("A non-empty signature file is required");
      }
      readResource(
        await resolveApi().attachPartySignature(
          normalizedCaseId,
          normalizedPartyId,
          { file, source: "sdkwork-im-h5-notary" },
        ),
        "notary party signature",
      );
    },

    async createPartyVideoInvite(
      caseId: string,
      partyId: string,
    ): Promise<NotaryPartyVideoInvite> {
      const invite = readResource(
        await resolveApi().createPartyVideoInvite(
          requireNonEmpty(caseId, "caseId"),
          requireNonEmpty(partyId, "partyId"),
          { purpose: "identity_verification" },
        ),
        "notary party video invite",
      );
      return {
        inviteId: requireString(invite, "inviteId", "notary party video invite"),
        conversationId: requireString(
          invite,
          "conversationId",
          "notary party video invite",
        ),
        inviteUrl: requireHttpsUrl(invite, "inviteUrl", "notary party video invite"),
        expiresAt: requireString(invite, "expiresAt", "notary party video invite"),
      };
    },

    async getNotaryMessages(): Promise<never> {
      throw new NotaryCapabilityUnavailableError("Notary notifications");
    },

    async getCloudFiles(): Promise<never> {
      throw new NotaryCapabilityUnavailableError("Notary cloud file browser");
    },
  };
}

export const notaryService = createNotaryService();

export const NOTARY_CLIENT_WINDOW_LIMIT = MAX_LIST_PAGE_SIZE;

export function appendBoundedUnique<T>(
  previous: readonly T[],
  incoming: readonly T[],
  identify: (value: T) => string,
  limit = NOTARY_CLIENT_WINDOW_LIMIT,
): T[] {
  if (!Number.isSafeInteger(limit) || limit < 1) {
    throw new TypeError("limit must be a positive safe integer");
  }
  const result: T[] = [];
  const ids = new Set<string>();
  for (const value of previous) {
    if (result.length >= limit) {
      break;
    }
    const id = identify(value);
    if (!ids.has(id)) {
      ids.add(id);
      result.push(value);
    }
  }
  for (const value of incoming) {
    if (result.length >= limit) {
      break;
    }
    const id = identify(value);
    if (!ids.has(id)) {
      ids.add(id);
      result.push(value);
    }
  }
  return result;
}

function assertRecordFilter(filter: string): asserts filter is NotaryRecordFilter {
  if (!RECORD_TABS.some((tab) => tab.id === filter)) {
    throw new TypeError(`Unsupported notary record filter: ${filter}`);
  }
}

function mapDraftParty(party: NotaryDraftParty): Record<string, unknown> {
  return {
    name: requireNonEmpty(party.name, "party.name"),
    role: requireNonEmpty(party.role ?? "applicant", "party.role"),
    partyType: "natural_person",
    identityType: "national_id",
    identityNo: requireNonEmpty(party.idCard, "party.idCard"),
    ...(party.phone ? { phone: party.phone } : {}),
    ...(party.gender ? { gender: party.gender } : {}),
    ...(party.dob ? { birthDate: party.dob } : {}),
    ...(party.address ? { address: party.address } : {}),
    ...(party.remarks ? { remarks: party.remarks } : {}),
    ...(party.idStartDate ? { identityValidDateStart: party.idStartDate } : {}),
    ...(party.idEndDate ? { identityValidDateEnd: party.idEndDate } : {}),
  };
}

function mapNotaryRecord(value: unknown): NotaryRecord {
  const record = asRecord(value);
  return {
    id: requireString(record, "id", "notary case"),
    title: requireString(record, "title", "notary case"),
    type: optionalString(record, "type") ?? "",
    date: requireString(record, "createTime", "notary case"),
    status: requireCaseStatus(record.status),
  };
}

function mapNotaryDetail(record: Record<string, unknown>): NotaryDetailData {
  const parties = optionalArray(record, "parties").map(mapNotaryParty);
  const documents = optionalArray(record, "documents").map(mapNotaryFile);
  return {
    id: requireString(record, "id", "notary case detail"),
    title: requireString(record, "title", "notary case detail"),
    time: optionalString(record, "processTime")
      ?? requireString(record, "createTime", "notary case detail"),
    item: optionalString(record, "type") ?? "",
    notaryName: optionalString(record, "notary") ?? "",
    notaryNo: optionalString(record, "primaryNotaryMembershipId") ?? "",
    status: requireCaseStatus(record.status),
    remarks: optionalString(record, "remarks") ?? "",
    parties,
    materials: documents,
  };
}

function mapNotaryParty(value: unknown): NotaryParty {
  const party = asRecord(value);
  const verificationStatus = optionalString(party, "verificationStatus") ?? "pending";
  if (!["pending", "verified", "failed", "expired"].includes(verificationStatus)) {
    throw new NotaryResponseContractError(
      `notary party returned unsupported verificationStatus: ${verificationStatus}`,
    );
  }
  return {
    id: requireString(party, "id", "notary party"),
    name: requireString(party, "name", "notary party"),
    role: requireString(party, "role", "notary party"),
    status: verificationStatus as NotaryParty["status"],
    ...(optionalString(party, "phone") ? { phone: optionalString(party, "phone") } : {}),
    ...(optionalString(party, "gender") ? { gender: optionalString(party, "gender") } : {}),
    ...(optionalString(party, "birthDate") ? { dob: optionalString(party, "birthDate") } : {}),
    ...(optionalString(party, "address") ? { address: optionalString(party, "address") } : {}),
    ...(optionalString(party, "identityId") ? { idCard: optionalString(party, "identityId") } : {}),
    ...(optionalString(party, "identityValidDateStart")
      ? { idStartDate: optionalString(party, "identityValidDateStart") }
      : {}),
    ...(optionalString(party, "identityValidDateEnd")
      ? { idEndDate: optionalString(party, "identityValidDateEnd") }
      : {}),
  };
}

function mapNotaryFile(value: unknown): NotaryFile {
  const file = asRecord(value);
  const media = asRecord(file.mediaResource);
  const status = optionalString(file, "status") ?? "pending";
  const category = optionalString(file, "category") ?? "evidence";
  const name = requireString(file, "name", "notary document");
  const previewUrl = optionalString(media, "previewUrl");
  const downloadUrl = optionalString(media, "downloadUrl") ?? optionalString(media, "url");
  return {
    id: optionalString(file, "nodeId") ?? name,
    name,
    size: optionalString(file, "size") ?? "",
    uploadTime: optionalString(file, "createTime") ?? "",
    fileType: inferFileType(name, optionalString(media, "contentType")),
    tags: [
      { label: category, color: "blue" },
      { label: status, color: status === "error" ? "red" : status === "verified" ? "green" : "orange" },
    ],
    uploader: "",
    ...(previewUrl ? { previewUrl } : {}),
    ...(downloadUrl ? { downloadUrl } : {}),
  };
}

function mapNotaryStaff(value: unknown): NotaryStaffMember {
  const staff = asRecord(value);
  const name = requireString(staff, "displayName", "notary staff member");
  const departments = optionalStringArray(staff, "departments");
  return {
    id: requireString(staff, "membershipId", "notary staff member"),
    name,
    organization: departments.join(" / "),
    active: (optionalString(staff, "status") ?? "").toLowerCase() === "active",
    initial: resolveInitial(name),
  };
}

function readCursorPage(
  value: unknown,
  resourceName: string,
): { items: unknown[]; pageInfo: NotaryPageInfo } {
  const data = readResource(value, resourceName);
  if (!Array.isArray(data.items)) {
    throw new NotaryResponseContractError(`${resourceName} is missing data.items`);
  }
  const pageInfo = asRecord(data.pageInfo);
  if (pageInfo.mode !== "cursor") {
    throw new NotaryResponseContractError(`${resourceName} must use cursor pagination`);
  }
  const nextCursor = optionalString(pageInfo, "nextCursor");
  const hasMore = pageInfo.hasMore;
  if (typeof hasMore !== "boolean") {
    throw new NotaryResponseContractError(`${resourceName} is missing pageInfo.hasMore`);
  }
  if (hasMore && !nextCursor) {
    throw new NotaryResponseContractError(
      `${resourceName} hasMore=true without pageInfo.nextCursor`,
    );
  }
  return {
    items: data.items,
    pageInfo: {
      mode: "cursor",
      hasMore,
      ...(nextCursor ? { nextCursor } : {}),
    },
  };
}

function readResource(value: unknown, resourceName: string): Record<string, unknown> {
  const root = asRecord(value);
  const data = root.data;
  if (data !== undefined) {
    if (!data || typeof data !== "object" || Array.isArray(data)) {
      throw new NotaryResponseContractError(`${resourceName} returned invalid data`);
    }
    return data as Record<string, unknown>;
  }
  if (Object.keys(root).length === 0) {
    throw new NotaryResponseContractError(`${resourceName} returned an empty response`);
  }
  return root;
}

function readCount(record: Record<string, unknown>, field: string): number {
  const value = asRecord(record[field]).count;
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) {
    throw new NotaryResponseContractError(
      `notary dashboard statistics returned invalid ${field}.count`,
    );
  }
  return value;
}

function requireCaseStatus(value: unknown): NotaryCaseStatus {
  if (
    value === "PENDING_REVIEW"
    || value === "PROCESSING"
    || value === "COMPLETED"
    || value === "REJECTED"
    || value === "CANCELLED"
    || value === "CREATE_FAILED"
  ) {
    return value;
  }
  throw new NotaryResponseContractError(`notary case returned unsupported status: ${String(value)}`);
}

function inferFileType(
  name: string,
  contentType?: string,
): NotaryFile["fileType"] {
  const normalizedContentType = contentType?.toLowerCase() ?? "";
  const extension = name.split(".").pop()?.toLowerCase() ?? "";
  if (normalizedContentType.startsWith("image/") || ["jpg", "jpeg", "png", "gif", "webp"].includes(extension)) {
    return "image";
  }
  if (normalizedContentType.startsWith("video/") || ["mp4", "mov", "webm"].includes(extension)) {
    return "video";
  }
  if (normalizedContentType === "application/pdf" || extension === "pdf") {
    return "pdf";
  }
  if (["doc", "docx"].includes(extension)) {
    return "word";
  }
  if (["xls", "xlsx"].includes(extension)) {
    return "excel";
  }
  if (["zip", "rar", "7z"].includes(extension)) {
    return "zip";
  }
  return "unknown";
}

function resolveInitial(value: string): string {
  const firstCharacter = value.trim().charAt(0).toUpperCase();
  return /^[A-Z]$/.test(firstCharacter) ? firstCharacter : "#";
}

function asRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function requireString(
  record: Record<string, unknown>,
  field: string,
  resourceName: string,
): string {
  const value = optionalString(record, field);
  if (!value) {
    throw new NotaryResponseContractError(`${resourceName} is missing ${field}`);
  }
  return value;
}

function requireBoolean(
  record: Record<string, unknown>,
  field: string,
  resourceName: string,
): boolean {
  const value = record[field];
  if (typeof value !== "boolean") {
    throw new NotaryResponseContractError(`${resourceName} is missing ${field}`);
  }
  return value;
}

function requireHttpsUrl(
  record: Record<string, unknown>,
  field: string,
  resourceName: string,
): string {
  const value = requireString(record, field, resourceName);
  let url: URL;
  try {
    url = new URL(value);
  } catch {
    throw new NotaryResponseContractError(
      `${resourceName} returned an invalid ${field}`,
    );
  }
  if (url.protocol !== "https:") {
    throw new NotaryResponseContractError(
      `${resourceName} returned a non-HTTPS ${field}`,
    );
  }
  return url.toString();
}

function optionalString(record: Record<string, unknown>, field: string): string | undefined {
  const value = record[field];
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

function optionalArray(record: Record<string, unknown>, field: string): unknown[] {
  const value = record[field];
  return Array.isArray(value) ? value : [];
}

function optionalStringArray(record: Record<string, unknown>, field: string): string[] {
  return optionalArray(record, field).filter(
    (value): value is string => typeof value === "string" && value.trim().length > 0,
  );
}

function requireNonEmpty(value: string, field: string): string {
  const normalized = value.trim();
  if (!normalized) {
    throw new TypeError(`${field} is required`);
  }
  return normalized;
}

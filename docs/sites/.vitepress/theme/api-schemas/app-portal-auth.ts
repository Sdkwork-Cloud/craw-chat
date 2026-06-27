import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

const portalUserFields: ApiSchemaField[] = [
  field("id", "string", "Portal user identifier.", { required: true }),
  field("login", "string", "Portal login name.", { required: true }),
  field("name", "string", "Display name.", { required: true }),
  field("role", "string", "Portal-facing role label.", { required: true }),
  field("email", "string", "Primary email address.", { required: true }),
  field("actorKind", "string", "Underlying actor kind.", { required: true }),
  field("clientKind", "string", "Client class used during portal sign-in.", { required: true }),
  field("permissions", "string[]", "Resolved portal and control permissions.", { required: true }),
];

const portalWorkspaceFields: ApiSchemaField[] = [
  field("name", "string", "Workspace display name.", { required: true }),
  field("slug", "string", "Workspace slug.", { required: true }),
  field("tier", "string", "Commercial or support tier.", { required: true }),
  field("region", "string", "Primary deployment region label.", { required: true }),
  field("supportPlan", "string", "Support plan label.", { required: true }),
  field("seats", "uint64", "Allocated or active seat count.", { required: true }),
  field("activeBrands", "uint64", "Active brand or tenant-brand count.", { required: true }),
  field("uptime", "string", "Rendered uptime summary.", { required: true }),
];

const portalSnapshotMetricFields: ApiSchemaField[] = [
  field("label", "string", "Metric label.", { required: true }),
  field("value", "string", "Rendered metric value.", { required: true }),
  field("delta", "string | null", "Change indicator such as +12% or -18%."),
  field("tone", "string | null", "Presentation tone such as positive, warning, or critical."),
  field("caption", "string | null", "Supporting short description."),
  field("percent", "uint64 | null", "Progress or share percentage when rendered as a bar."),
  field("status", "string | null", "Human-readable status label."),
];

const portalSnapshotHeroFields: ApiSchemaField[] = [
  field("eyebrow", "string | null", "Short pre-title label."),
  field("title", "string", "Primary page title.", { required: true }),
  field("description", "string | null", "Supporting descriptive copy."),
  arrayField(
    "kpis",
    "PortalSnapshotMetric",
    "Optional metric cards shown in the hero region.",
    portalSnapshotMetricFields,
    {
      summary: "View nested fields for kpis",
    },
  ),
];

const portalSnapshotDetailFields: ApiSchemaField[] = [
  field("label", "string", "Detail label.", { required: true }),
  field("value", "string", "Detail value.", { required: true }),
];

const portalSnapshotItemFields: ApiSchemaField[] = [
  field("title", "string | null", "Item title."),
  field("topic", "string | null", "Topic or grouping label."),
  field("description", "string | null", "Supporting descriptive copy."),
  field("label", "string | null", "Compact item label."),
  field("value", "string | null", "Compact rendered value."),
  field("caption", "string | null", "Short supporting caption."),
  field("status", "string | null", "Rendered status."),
  field("tone", "string | null", "Presentation tone."),
  field("owner", "string | null", "Responsible user or team."),
  field("state", "string | null", "Runtime or workflow state."),
  field("priority", "string | null", "Priority label."),
  field("note", "string | null", "Free-form operational note."),
  field("percent", "uint64 | null", "Progress percentage."),
];

export const appPortalAuthSchemas: ApiSchemaDefinitionMap = {
  PortalUserView: {
    fields: portalUserFields,
  },
  PortalWorkspaceView: {
    fields: portalWorkspaceFields,
  },
  PortalLoginRequest: {
    fields: [
      field("tenantId", "string", "Tenant identifier or slug used for portal sign-in.", {
        required: true,
      }),
      field("login", "string", "Portal login name.", { required: true }),
      field("password", "string", "Portal password.", { required: true }),
      field("deviceId", "string | null", "Optional client device identifier."),
      field("sessionId", "string | null", "Optional prior session identifier."),
      field("clientKind", "string | null", "Portal client kind such as portal_operator."),
    ],
  },
  PortalLoginResponse: {
    fields: [
      field("accessToken", "string", "Bearer token issued for subsequent requests.", {
        required: true,
      }),
      field("refreshToken", "string", "Refresh token when the deployment enables token rotation.", {
        required: true,
      }),
      field("expiresAt", "uint64", "Unix timestamp or epoch seconds for access-token expiry.", {
        required: true,
      }),
      objectField("user", "Resolved portal user.", portalUserFields, {
        required: true,
        summary: "View nested fields for user",
      }),
      objectField("workspace", "Resolved workspace snapshot.", portalWorkspaceFields, {
        summary: "View nested fields for workspace",
      }),
    ],
  },
  PortalMeResponse: {
    fields: [
      field("tenantId", "string", "Current tenant identifier.", { required: true }),
      objectField("user", "Current portal user.", portalUserFields, {
        required: true,
        summary: "View nested fields for user",
      }),
      objectField("workspace", "Resolved workspace snapshot.", portalWorkspaceFields, {
        summary: "View nested fields for workspace",
      }),
    ],
  },
  PortalSnapshotMetric: {
    fields: portalSnapshotMetricFields,
  },
  PortalSnapshotDetail: {
    fields: portalSnapshotDetailFields,
  },
  PortalSnapshotItem: {
    fields: portalSnapshotItemFields,
  },
  PortalSnapshotHero: {
    fields: portalSnapshotHeroFields,
  },
  PortalSnapshot: {
    fields: [
      objectField(
        "hero",
        "Primary hero block. Checked-in portal mocks consistently expose a hero object for top-level copy and KPIs.",
        portalSnapshotHeroFields,
        {
          summary: "View nested fields for hero",
        },
      ),
      arrayField(
        "details",
        "PortalSnapshotDetail",
        "Optional compact label-value details used on auth and overview screens.",
        portalSnapshotDetailFields,
        {
          summary: "View nested fields for details",
        },
      ),
      arrayField(
        "items",
        "PortalSnapshotItem",
        "Optional generic cards, lists, or queue items. The portal modules treat snapshot records as flexible view models.",
        portalSnapshotItemFields,
        {
          summary: "View nested fields for items",
        },
      ),
      field(
        "metadata",
        "Record<string, unknown> | null",
        "Module-specific supplemental snapshot data not normalized into a fixed schema.",
      ),
    ],
  },
};

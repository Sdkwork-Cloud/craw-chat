import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

const providerPluginDescriptorFields: ApiSchemaField[] = [
  field("pluginId", "string", "Plugin identifier.", { required: true }),
  field(
    "domain",
    "string",
    "Provider domain. Wire values are rtc, object-storage, user-module, iot-access, and iot-protocol.",
    {
      required: true,
    },
  ),
  field("providerKind", "string", "Provider implementation kind.", { required: true }),
  field("displayName", "string", "Display name.", { required: true }),
  field("interfaceVersion", "string", "Provider interface version.", { required: true }),
  field("configSchemaRef", "string", "Configuration schema reference.", { required: true }),
  field("defaultSelected", "boolean", "Whether the plugin is the default selection.", {
    required: true,
  }),
  field("tenantOverrideAllowed", "boolean", "Whether tenant-level override is allowed.", {
    required: true,
  }),
  field("requiredCapabilities", "string[]", "Required capabilities.", { required: true }),
  field("optionalCapabilities", "string[]", "Optional capabilities.", { required: true }),
  field("unsupportedFeatures", "string[]", "Unsupported features.", { required: true }),
  field("degradedBehaviors", "string[]", "Known degraded behaviors.", { required: true }),
];

const effectiveProviderBindingFields: ApiSchemaField[] = [
  field("domain", "string", "Provider domain.", { required: true }),
  field("defaultPluginId", "string | null", "Global default plugin identifier."),
  field("selectedPluginId", "string | null", "Effective plugin identifier."),
  field("selectionSource", "string", "Selection source.", { required: true }),
  field("tenantOverrideAllowed", "boolean", "Whether tenant-level override is allowed.", {
    required: true,
  }),
];

const providerPolicySelectionFields: ApiSchemaField[] = [
  field("domain", "string", "Provider domain.", { required: true }),
  field("pluginId", "string", "Plugin identifier.", { required: true }),
];

const tenantProviderPolicySelectionFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  arrayField("bindings", "ProviderPolicySelection", "Tenant-specific provider bindings.", providerPolicySelectionFields, {
    required: true,
    summary: "View nested fields for bindings",
  }),
];

const providerPolicySnapshotFields: ApiSchemaField[] = [
  field("version", "uint64", "Policy version.", { required: true }),
  field("recordedAt", "date-time string", "Policy record timestamp.", { required: true }),
  field("rollbackFromVersion", "uint64 | null", "Source version when produced by a rollback."),
  arrayField("deploymentProfiles", "ProviderPolicySelection", "Deployment-level provider bindings.", providerPolicySelectionFields, {
    required: true,
    summary: "View nested fields for deploymentProfiles",
  }),
  arrayField(
    "tenantOverrides",
    "TenantProviderPolicySelection",
    "Tenant-level provider binding overrides.",
    tenantProviderPolicySelectionFields,
    {
      required: true,
      summary: "View nested fields for tenantOverrides",
    },
  ),
];

const providerPolicyChangeFields: ApiSchemaField[] = [
  field("domain", "string", "Provider domain.", { required: true }),
  field("changeKind", "string", "Change kind. Supported values: added, removed, changed.", {
    required: true,
  }),
  field("fromPluginId", "string | null", "Previous plugin identifier."),
  field("toPluginId", "string | null", "Next plugin identifier."),
];

const tenantProviderPolicyChangeFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("domain", "string", "Provider domain.", { required: true }),
  field("changeKind", "string", "Change kind. Supported values: added, removed, changed.", {
    required: true,
  }),
  field("fromPluginId", "string | null", "Previous plugin identifier."),
  field("toPluginId", "string | null", "Next plugin identifier."),
];

const providerPolicyDiffFields: ApiSchemaField[] = [
  field("fromVersion", "uint64", "Base version.", { required: true }),
  field("toVersion", "uint64", "Target version.", { required: true }),
  field("fromRecordedAt", "date-time string", "Base version record timestamp.", { required: true }),
  field("toRecordedAt", "date-time string", "Target version record timestamp.", { required: true }),
  arrayField(
    "deploymentProfileChanges",
    "ProviderPolicyChange",
    "Deployment-level binding changes.",
    providerPolicyChangeFields,
    {
      required: true,
      summary: "View nested fields for deploymentProfileChanges",
    },
  ),
  arrayField(
    "tenantOverrideChanges",
    "TenantProviderPolicyChange",
    "Tenant override changes.",
    tenantProviderPolicyChangeFields,
    {
      required: true,
      summary: "View nested fields for tenantOverrideChanges",
    },
  ),
];

const nodeLifecycleFields: ApiSchemaField[] = [
  field("nodeId", "string", "Node identifier.", { required: true }),
  field("drainStatus", "string", "Drain state.", { required: true }),
  field("rebalanceState", "string", "Rebalance state.", { required: true }),
  field("ownedRouteCount", "uint64", "Number of currently owned routes.", { required: true }),
];

export const controlPlaneProviderSchemas: ApiSchemaDefinitionMap = {
  ProviderPluginDescriptor: { fields: providerPluginDescriptorFields },
  EffectiveProviderBinding: { fields: effectiveProviderBindingFields },
  ProviderRegistrySnapshotResponse: {
    fields: [
      field("status", "string", "Read status. Fixed to registry for this endpoint.", {
        required: true,
      }),
      field("interfaceVersion", "string", "Provider registry interface version.", {
        required: true,
      }),
      arrayField("plugins", "ProviderPluginDescriptor", "Installed provider plugins.", providerPluginDescriptorFields, {
        required: true,
        summary: "View nested fields for plugins",
      }),
      arrayField(
        "effectiveBindings",
        "EffectiveProviderBinding",
        "Effective global provider bindings.",
        effectiveProviderBindingFields,
        {
          required: true,
          summary: "View nested fields for effectiveBindings",
        },
      ),
      field("precedence", "string[]", "Binding precedence chain.", { required: true }),
    ],
  },
  ProviderBindingsResponse: {
    fields: [
      field("status", "string", "Read status. Fixed to bindings for this endpoint.", {
        required: true,
      }),
      field("interfaceVersion", "string", "Provider registry interface version.", {
        required: true,
      }),
      field("tenantId", "string | null", "Tenant identifier for the selected scope."),
      arrayField(
        "effectiveBindings",
        "EffectiveProviderBinding",
        "Effective provider bindings for the selected scope.",
        effectiveProviderBindingFields,
        {
          required: true,
          summary: "View nested fields for effectiveBindings",
        },
      ),
      field("precedence", "string[]", "Binding precedence chain.", { required: true }),
    ],
  },
  UpsertProviderBindingPolicyRequest: {
    fields: [
      field("tenantId", "string | null", "Tenant identifier for override scope. Omit for deployment-level policy."),
      field(
        "domain",
        "string",
        "Provider domain. Accepted values: rtc, object-storage, user-module, iot-access, iot-protocol.",
        { required: true },
      ),
      field("pluginId", "string", "Target plugin identifier.", { required: true }),
      field("expectedBaseVersion", "uint64 | null", "Expected base version for optimistic concurrency."),
    ],
  },
  ProviderPolicySelection: { fields: providerPolicySelectionFields },
  TenantProviderPolicySelection: { fields: tenantProviderPolicySelectionFields },
  ProviderPolicySnapshot: { fields: providerPolicySnapshotFields },
  ProviderPolicyHistoryResponse: {
    fields: [
      field("status", "string", "Read status. Values include history and rolled_back.", {
        required: true,
      }),
      field("currentVersion", "uint64", "Current policy version.", { required: true }),
      arrayField("items", "ProviderPolicySnapshot", "Provider policy history entries.", providerPolicySnapshotFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  ProviderPolicyChange: { fields: providerPolicyChangeFields },
  TenantProviderPolicyChange: { fields: tenantProviderPolicyChangeFields },
  ProviderPolicyDiffResponse: {
    fields: [
      field("status", "string", "Read status. Fixed to diff for this endpoint.", { required: true }),
      ...providerPolicyDiffFields,
    ],
  },
  ProviderPolicyPreview: {
    fields: [
      field("status", "string", "Result status. Fixed to preview for preview responses.", {
        required: true,
      }),
      field("baseVersion", "uint64", "Base version.", { required: true }),
      field("previewVersion", "uint64", "Preview version.", { required: true }),
      field("tenantId", "string | null", "Tenant identifier for the preview scope."),
      objectField("previewBinding", "Effective binding after the preview is applied.", effectiveProviderBindingFields, {
        required: true,
        summary: "View nested fields for previewBinding",
      }),
      objectField("diff", "Preview diff.", providerPolicyDiffFields, {
        required: true,
        summary: "View nested fields for diff",
      }),
    ],
  },
  ProviderBindingCommitResponse: {
    fields: [
      field("status", "string", "Commit result status. Values include applied and noop.", {
        required: true,
      }),
      field("applied", "boolean", "Whether the change was persisted.", { required: true }),
      field("interfaceVersion", "string", "Provider registry interface version.", {
        required: true,
      }),
      field("tenantId", "string | null", "Tenant identifier for the commit scope."),
      field("currentVersion", "uint64", "Current version after the commit.", { required: true }),
      objectField("committedBinding", "Binding touched by the commit.", effectiveProviderBindingFields, {
        required: true,
        summary: "View nested fields for committedBinding",
      }),
      objectField("diff", "Commit diff.", providerPolicyDiffFields, {
        required: true,
        summary: "View nested fields for diff",
      }),
      arrayField(
        "effectiveBindings",
        "EffectiveProviderBinding",
        "Effective bindings after the commit.",
        effectiveProviderBindingFields,
        {
          required: true,
          summary: "View nested fields for effectiveBindings",
        },
      ),
      field("precedence", "string[]", "Binding precedence chain.", { required: true }),
    ],
  },
  ProviderPolicyRollbackRequest: {
    fields: [field("targetVersion", "uint64", "Target version to roll back to.", { required: true })],
  },
  MigrateRoutesRequest: {
    fields: [field("targetNodeId", "string", "Target node identifier.", { required: true })],
  },
  RouteNodeLifecycle: { fields: nodeLifecycleFields },
  RouteMigrationResult: {
    fields: [
      field("sourceNodeId", "string", "Source node identifier.", { required: true }),
      field("targetNodeId", "string", "Target node identifier.", { required: true }),
      field("migratedRouteCount", "uint64", "Number of migrated routes.", { required: true }),
      field("sourceDrainStatus", "string", "Source node drain state.", { required: true }),
      field("sourceRebalanceState", "string", "Source node rebalance state.", { required: true }),
      field("targetDrainStatus", "string", "Target node drain state.", { required: true }),
      field("targetRebalanceState", "string", "Target node rebalance state.", { required: true }),
    ],
  },
};

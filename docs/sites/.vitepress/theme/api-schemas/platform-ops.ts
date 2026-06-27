import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

const serviceHealthViewFields: ApiSchemaField[] = [
  field("service", "string", "Service name.", { required: true }),
  field("status", "string", "Service health state.", { required: true }),
];

const lagItemFields: ApiSchemaField[] = [
  field("component", "string", "Measured component.", { required: true }),
  field("scopeId", "string", "Scope identifier.", { required: true }),
  field("currentOffset", "uint64", "Current offset.", { required: true }),
  field("committedOffset", "uint64", "Committed offset.", { required: true }),
  field("lag", "uint64", "Lag value.", { required: true }),
];

const projectionPlaneMetricCounterFields: ApiSchemaField[] = [
  field("attemptCount", "uint64", "Attempt count.", { required: true }),
  field("successCount", "uint64", "Successful attempt count.", { required: true }),
  field("failureCount", "uint64", "Failed attempt count.", { required: true }),
];

const projectionPlaneMetricsFields: ApiSchemaField[] = [
  objectField(
    "conversationSnapshotPersist",
    "Conversation snapshot persist counters.",
    projectionPlaneMetricCounterFields,
    { required: true, summary: "View nested fields for conversationSnapshotPersist" },
  ),
  objectField(
    "conversationSnapshotRestore",
    "Conversation snapshot restore counters.",
    projectionPlaneMetricCounterFields,
    { required: true, summary: "View nested fields for conversationSnapshotRestore" },
  ),
  objectField(
    "deviceSyncSnapshotPersist",
    "Client-route sync snapshot persist counters.",
    projectionPlaneMetricCounterFields,
    { required: true, summary: "View nested fields for deviceSyncSnapshotPersist" },
  ),
  objectField(
    "deviceSyncSnapshotRestore",
    "Client-route sync snapshot restore counters.",
    projectionPlaneMetricCounterFields,
    { required: true, summary: "View nested fields for deviceSyncSnapshotRestore" },
  ),
];

const projectionReplayMetricsFields: ApiSchemaField[] = [
  field("backlogSize", "uint64", "Replay backlog size.", { required: true }),
  field("replayedEventCount", "uint64", "Replayed event count.", { required: true }),
  field("durationMs", "uint64", "Replay duration in milliseconds.", { required: true }),
];

const projectionUpdateDelayFields: ApiSchemaField[] = [
  field("timelineMs", "uint64", "Timeline projection delay in milliseconds.", { required: true }),
  field("inboxMs", "uint64", "Inbox projection delay in milliseconds.", { required: true }),
  field("sourceEventType", "string | null", "Most recent observed source event type."),
  field("scopeId", "string | null", "Most recent observed scope identifier."),
  field("recordedAt", "date-time string | null", "Timestamp of the most recent measurement."),
];

const projectionPlaneHealthFields: ApiSchemaField[] = [
  field("status", "string", "Projection-plane health state.", { required: true }),
  objectField("metrics", "Projection-plane counters.", projectionPlaneMetricsFields, {
    required: true,
    summary: "View nested fields for metrics",
  }),
  objectField("replay", "Projection replay metrics.", projectionReplayMetricsFields, {
    required: true,
    summary: "View nested fields for replay",
  }),
  field("rebuildDurationMs", "uint64", "Projection rebuild duration in milliseconds.", {
    required: true,
  }),
  objectField("updateDelay", "Projection update delay metrics.", projectionUpdateDelayFields, {
    required: true,
    summary: "View nested fields for updateDelay",
  }),
  field("lastFailureCode", "string | null", "Most recent failure code."),
  field("lastFailureMessage", "string | null", "Most recent failure message."),
];

const clusterNodeViewFields: ApiSchemaField[] = [
  field("nodeId", "string", "Node identifier.", { required: true }),
  field("profile", "string", "Node deployment profile.", { required: true }),
  field("bindAddr", "string", "Listen address.", { required: true }),
  field("drainStatus", "string", "Drain state.", { required: true }),
  field("rebalanceState", "string", "Rebalance state.", { required: true }),
  field("deviceRouteCount", "uint64", "Number of owned client routes.", { required: true }),
  field("ownedScopes", "string[]", "Scope identifiers currently owned by the node.", {
    required: true,
  }),
  arrayField("services", "ServiceHealthView", "Service health entries for the node.", serviceHealthViewFields, {
    required: true,
    summary: "View nested fields for services",
  }),
];

const runtimeDirInspectionItemFields: ApiSchemaField[] = [
  field("fileName", "string", "File name.", { required: true }),
  field("path", "string", "Absolute file path.", { required: true }),
  field("required", "boolean", "Whether the file is required.", { required: true }),
  field("exists", "boolean", "Whether the file currently exists.", { required: true }),
  field("parseable", "boolean", "Whether the file can be parsed.", { required: true }),
  field("status", "string", "Inspection status.", { required: true }),
  field("sizeBytes", "uint64 | null", "File size in bytes."),
  field("parseError", "string | null", "Parse error, when inspection failed."),
  field("recommendedAction", "string", "Recommended remediation action.", { required: true }),
];

const routeOwnershipViewFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("deviceId", "string", "Device identifier.", { required: true }),
  field("ownerNodeId", "string", "Owning node identifier.", { required: true }),
  field("connectionKind", "string", "Connection kind.", { required: true }),
  field("boundAt", "date-time string", "Route bind timestamp.", { required: true }),
];

const providerBindingItemViewFields: ApiSchemaField[] = [
  field("domain", "string", "Provider domain.", { required: true }),
  field("defaultPluginId", "string | null", "Global default plugin identifier."),
  field("selectedPluginId", "string | null", "Currently selected plugin identifier."),
  field("selectionSource", "string", "Selection source.", { required: true }),
  field("tenantOverrideAllowed", "boolean", "Whether tenant-level override is allowed.", {
    required: true,
  }),
];

const providerBindingSnapshotViewFields: ApiSchemaField[] = [
  field("interfaceVersion", "string", "Provider registry interface version.", { required: true }),
  field("tenantId", "string | null", "Tenant identifier for the current snapshot scope."),
  arrayField(
    "effectiveBindings",
    "ProviderBindingItemView",
    "Effective provider bindings in the selected scope.",
    providerBindingItemViewFields,
    {
      required: true,
      summary: "View nested fields for effectiveBindings",
    },
  ),
  field("precedence", "string[]", "Binding precedence chain.", { required: true }),
];

const providerBindingDriftItemViewFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier exhibiting drift.", { required: true }),
  field("domain", "string", "Provider domain.", { required: true }),
  field("baselineSelectedPluginId", "string | null", "Selected plugin identifier in the baseline."),
  field("selectedPluginId", "string | null", "Selected plugin identifier in the current view."),
  field("baselineSelectionSource", "string", "Baseline selection source.", { required: true }),
  field("selectionSource", "string", "Current selection source.", { required: true }),
  field("driftKind", "string", "Drift classification.", { required: true }),
];

const projectionPlaneTraceFields: ApiSchemaField[] = [
  field("traceId", "string", "Trace identifier.", { required: true }),
  field("operation", "string", "Observed operation.", { required: true }),
  field("scopeType", "string", "Scope type.", { required: true }),
  field("scopeId", "string", "Scope identifier.", { required: true }),
  field("outcome", "string", "Operation outcome.", { required: true }),
  field("recordedAt", "date-time string", "Trace timestamp.", { required: true }),
];

const projectionPlaneLogFields: ApiSchemaField[] = [
  field("level", "string", "Log level.", { required: true }),
  field("code", "string", "Log code.", { required: true }),
  field("operation", "string", "Observed operation.", { required: true }),
  field("scopeType", "string", "Scope type.", { required: true }),
  field("scopeId", "string", "Scope identifier.", { required: true }),
  field("message", "string", "Log message.", { required: true }),
  field("recordedAt", "date-time string", "Log timestamp.", { required: true }),
];

export const platformOpsSchemas: ApiSchemaDefinitionMap = {
  ServiceHealthView: { fields: serviceHealthViewFields },
  OpsHealthResponse: {
    fields: [
      arrayField("items", "ServiceHealthView", "Service health list.", serviceHealthViewFields, {
        required: true,
        summary: "View nested fields for items",
      }),
      objectField("projectionPlane", "Projection-plane health state.", projectionPlaneHealthFields, {
        required: true,
        summary: "View nested fields for projectionPlane",
      }),
    ],
  },
  ClusterView: {
    fields: [
      arrayField("nodes", "ClusterNodeView", "Visible cluster nodes.", clusterNodeViewFields, {
        required: true,
        summary: "View nested fields for nodes",
      }),
    ],
  },
  ClusterNodeView: { fields: clusterNodeViewFields },
  LagItem: { fields: lagItemFields },
  LagView: {
    fields: [
      arrayField("items", "LagItem", "Lag measurements.", lagItemFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  ProjectionReplayStatusView: {
    fields: [
      field("generatedAt", "date-time string", "Status generation timestamp.", { required: true }),
      field("status", "string", "Replay status.", { required: true }),
      objectField("replay", "Replay metrics.", projectionReplayMetricsFields, {
        required: true,
        summary: "View nested fields for replay",
      }),
      field("replayThroughputPerSecond", "uint64", "Replay throughput per second.", {
        required: true,
      }),
      arrayField("lag", "LagItem", "Lag metrics associated with projection replay.", lagItemFields, {
        required: true,
        summary: "View nested fields for lag",
      }),
    ],
  },
  RuntimeDirInspectionView: {
    fields: [
      field("status", "string", "Runtime directory health status.", { required: true }),
      field("runtimeDir", "string | null", "Runtime root directory."),
      field("stateDir", "string | null", "State directory."),
      field("healthyFileCount", "uint64", "Count of healthy files.", { required: true }),
      field("missingFileCount", "uint64", "Count of missing files.", { required: true }),
      field("corruptFileCount", "uint64", "Count of corrupt files.", { required: true }),
      arrayField("files", "RuntimeDirInspectionItem", "Per-file inspection results.", runtimeDirInspectionItemFields, {
        required: true,
        summary: "View nested fields for files",
      }),
    ],
  },
  RuntimeDirInspectionItem: { fields: runtimeDirInspectionItemFields },
  ProviderBindingItemView: { fields: providerBindingItemViewFields },
  ProviderBindingSnapshotView: { fields: providerBindingSnapshotViewFields },
  ProviderBindingsView: {
    fields: [
      arrayField("items", "ProviderBindingSnapshotView", "Provider binding snapshots.", providerBindingSnapshotViewFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  ProviderBindingDriftItemView: { fields: providerBindingDriftItemViewFields },
  ProviderBindingDriftView: {
    fields: [
      field("baselineTenantId", "string | null", "Baseline tenant identifier."),
      arrayField("items", "ProviderBindingDriftItemView", "Provider binding drift items.", providerBindingDriftItemViewFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
  RouteOwnershipView: { fields: routeOwnershipViewFields },
  ProjectionPlaneMetricCounterView: { fields: projectionPlaneMetricCounterFields },
  ProjectionPlaneMetricsView: { fields: projectionPlaneMetricsFields },
  ProjectionReplayMetricsView: { fields: projectionReplayMetricsFields },
  ProjectionUpdateDelayView: { fields: projectionUpdateDelayFields },
  ProjectionPlaneHealthView: { fields: projectionPlaneHealthFields },
  ProjectionPlaneTraceView: { fields: projectionPlaneTraceFields },
  ProjectionPlaneLogView: { fields: projectionPlaneLogFields },
  ProjectionPlaneDiagnosticsView: {
    fields: [
      ...projectionPlaneHealthFields,
      arrayField("traces", "ProjectionPlaneTraceView", "Recent traces.", projectionPlaneTraceFields, {
        required: true,
        summary: "View nested fields for traces",
      }),
      arrayField("logs", "ProjectionPlaneLogView", "Recent logs.", projectionPlaneLogFields, {
        required: true,
        summary: "View nested fields for logs",
      }),
    ],
  },
  DiagnosticBundle: {
    fields: [
      field("generatedAt", "date-time string", "Bundle generation timestamp.", { required: true }),
      field("profile", "string", "Runtime profile.", { required: true }),
      field("nodeId", "string", "Node identifier.", { required: true }),
      field("bindAddr", "string", "Listen address.", { required: true }),
      field("drainStatus", "string", "Drain state.", { required: true }),
      field("rebalanceState", "string", "Rebalance state.", { required: true }),
      field("ownedScopes", "string[]", "Scopes currently owned by the node.", { required: true }),
      arrayField("services", "ServiceHealthView", "Service health entries.", serviceHealthViewFields, {
        required: true,
        summary: "View nested fields for services",
      }),
      arrayField("lag", "LagItem", "Lag measurements.", lagItemFields, {
        required: true,
        summary: "View nested fields for lag",
      }),
      arrayField("deviceRoutes", "RouteOwnershipView", "Client route ownership records.", routeOwnershipViewFields, {
        required: true,
        summary: "View nested fields for deviceRoutes",
      }),
      arrayField(
        "providerBindings",
        "ProviderBindingSnapshotView",
        "Provider binding snapshots.",
        providerBindingSnapshotViewFields,
        {
          required: true,
          summary: "View nested fields for providerBindings",
        },
      ),
      objectField(
        "providerBindingDrift",
        "Provider binding drift view.",
        [
          field("baselineTenantId", "string | null", "Baseline tenant identifier."),
          arrayField("items", "ProviderBindingDriftItemView", "Provider binding drift items.", providerBindingDriftItemViewFields, {
            required: true,
            summary: "View nested fields for items",
          }),
        ],
        {
          required: true,
          summary: "View nested fields for providerBindingDrift",
        },
      ),
      objectField(
        "projectionPlane",
        "Projection-plane diagnostics.",
        [
          ...projectionPlaneHealthFields,
          arrayField("traces", "ProjectionPlaneTraceView", "Recent traces.", projectionPlaneTraceFields, {
            required: true,
            summary: "View nested fields for traces",
          }),
          arrayField("logs", "ProjectionPlaneLogView", "Recent logs.", projectionPlaneLogFields, {
            required: true,
            summary: "View nested fields for logs",
          }),
        ],
        {
          required: true,
          summary: "View nested fields for projectionPlane",
        },
      ),
    ],
  },
};

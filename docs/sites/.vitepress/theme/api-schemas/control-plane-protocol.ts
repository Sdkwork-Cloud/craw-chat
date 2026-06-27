import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

const protocolSchemaResponseFields: ApiSchemaField[] = [
  field("schema", "string", "Schema identifier.", { required: true }),
  field("kind", "string", "Schema kind.", { required: true }),
  field("stage", "string", "Schema lifecycle stage.", { required: true }),
  field("bindingProtocols", "string[]", "Supported binding protocols.", { required: true }),
  field("requiredCapabilities", "string[]", "Required capabilities.", { required: true }),
  field("supportedConsumers", "string[]", "Supported consumer types.", { required: true }),
];

const clientCompatibilityFields: ApiSchemaField[] = [
  field("clientType", "string", "Client type.", { required: true }),
  field("minimumProtocolVersion", "string", "Minimum supported protocol version.", { required: true }),
  field("supportedBindings", "string[]", "Supported bindings.", { required: true }),
  field("supportedCodecs", "string[]", "Supported codecs.", { required: true }),
  field("supportedCapabilities", "string[]", "Supported capabilities.", { required: true }),
  field(
    "blockedExperimentalCapabilities",
    "string[]",
    "Experimental capabilities blocked for the client type.",
    { required: true },
  ),
];

const capabilityProfileFields: ApiSchemaField[] = [
  field("profileId", "string", "Capability profile identifier.", { required: true }),
  field("releaseChannel", "string", "Release channel.", { required: true }),
  field("enabledCapabilities", "string[]", "Enabled capabilities.", { required: true }),
  field("experimentalCapabilities", "string[]", "Experimental capabilities enabled in the profile.", {
    required: true,
  }),
];

const quotaProfileFields: ApiSchemaField[] = [
  field("profileId", "string", "Quota profile identifier.", { required: true }),
  field("maxConcurrentSessionsPerTenant", "uint32", "Maximum concurrent sessions per tenant.", {
    required: true,
  }),
  field("maxSubscriptionsPerSession", "uint32", "Maximum realtime subscriptions per session.", {
    required: true,
  }),
  field("maxInflightMessages", "uint32", "Maximum inflight messages.", { required: true }),
  field("maxPayloadBytes", "uint64", "Maximum payload size in bytes.", { required: true }),
];

const rolloutPolicyFields: ApiSchemaField[] = [
  field("policyId", "string", "Rollout policy identifier.", { required: true }),
  field("releaseChannel", "string", "Release channel.", { required: true }),
  field("trafficPercent", "uint8", "Traffic percentage.", { required: true }),
  field("cellSelector", "string", "Cell selector.", { required: true }),
  field("regionSelector", "string", "Region selector.", { required: true }),
  field("operatorOverride", "boolean", "Whether operator override is allowed.", { required: true }),
  field("tenantAllowlist", "string[]", "Tenant allowlist.", { required: true }),
];

const killSwitchFields: ApiSchemaField[] = [
  field("ruleId", "string", "Kill-switch rule identifier.", { required: true }),
  field("active", "boolean", "Whether the rule is active.", { required: true }),
  field("reason", "string", "Activation reason.", { required: true }),
  field("disabledCapabilities", "string[]", "Disabled capabilities.", { required: true }),
  field("disabledBindings", "string[]", "Disabled bindings.", { required: true }),
  field("disabledCodecs", "string[]", "Disabled codecs.", { required: true }),
];

const effectiveProtocolSnapshotFields: ApiSchemaField[] = [
  field("protocolVersion", "string", "Effective protocol version.", { required: true }),
  field("releaseChannel", "string", "Effective release channel.", { required: true }),
  field("enabledCapabilities", "string[]", "Enabled capabilities after governance resolution.", {
    required: true,
  }),
  field("allowedBindings", "string[]", "Allowed bindings.", { required: true }),
  field("allowedCodecs", "string[]", "Allowed codecs.", { required: true }),
  field("quotaProfileId", "string", "Effective quota profile identifier.", { required: true }),
  field("killSwitchActive", "boolean", "Whether a kill-switch rule is currently active.", {
    required: true,
  }),
  field("precedence", "string[]", "Governance precedence chain.", { required: true }),
];

const sdkCompatibilityBaselineFields: ApiSchemaField[] = [
  field("appSdkFacade", "string", "Application SDK facade package.", { required: true }),
  field("adminSdkFacade", "string", "Administrative SDK facade package.", { required: true }),
  field("matrixClientTypes", "string[]", "Client types represented in the compatibility matrix.", {
    required: true,
  }),
  field("protocolRegistryPath", "string", "Path to the protocol registry endpoint.", {
    required: true,
  }),
  field("protocolGovernancePath", "string", "Path to the protocol governance endpoint.", {
    required: true,
  }),
];

export const controlPlaneProtocolSchemas: ApiSchemaDefinitionMap = {
  ProtocolSchemaResponse: {
    fields: protocolSchemaResponseFields,
  },
  ClientCompatibilityResponse: {
    fields: clientCompatibilityFields,
  },
  ProtocolRegistryResponse: {
    fields: [
      field("protocolVersion", "string", "Protocol version.", { required: true }),
      field("bindings", "string[]", "Supported bindings.", { required: true }),
      field("codecs", "string[]", "Supported codecs.", { required: true }),
      arrayField("schemas", "ProtocolSchemaResponse", "Protocol schema inventory.", protocolSchemaResponseFields, {
        required: true,
        summary: "View nested fields for schemas",
      }),
      arrayField(
        "compatibilityMatrix",
        "ClientCompatibilityResponse",
        "Client compatibility matrix.",
        clientCompatibilityFields,
        {
          required: true,
          summary: "View nested fields for compatibilityMatrix",
        },
      ),
    ],
  },
  CapabilityProfileResponse: { fields: capabilityProfileFields },
  QuotaProfileResponse: { fields: quotaProfileFields },
  RolloutPolicyResponse: { fields: rolloutPolicyFields },
  KillSwitchResponse: { fields: killSwitchFields },
  EffectiveProtocolSnapshotResponse: { fields: effectiveProtocolSnapshotFields },
  SdkCompatibilityBaselineResponse: { fields: sdkCompatibilityBaselineFields },
  ProtocolGovernanceResponse: {
    fields: [
      objectField("capabilityProfile", "Capability profile.", capabilityProfileFields, {
        required: true,
        summary: "View nested fields for capabilityProfile",
      }),
      objectField("quotaProfile", "Quota profile.", quotaProfileFields, {
        required: true,
        summary: "View nested fields for quotaProfile",
      }),
      objectField("rolloutPolicy", "Rollout policy.", rolloutPolicyFields, {
        required: true,
        summary: "View nested fields for rolloutPolicy",
      }),
      objectField("killSwitch", "Kill-switch configuration.", killSwitchFields, {
        required: true,
        summary: "View nested fields for killSwitch",
      }),
      objectField("effectiveSnapshot", "Resolved effective protocol snapshot.", effectiveProtocolSnapshotFields, {
        required: true,
        summary: "View nested fields for effectiveSnapshot",
      }),
      objectField(
        "sdkCompatibilityBaseline",
        "SDK compatibility baseline.",
        sdkCompatibilityBaselineFields,
        {
          required: true,
          summary: "View nested fields for sdkCompatibilityBaseline",
        },
      ),
    ],
  },
};

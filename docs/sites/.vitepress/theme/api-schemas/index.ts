import { appConversationSchemas } from "./app-conversation";
import { appMediaStreamSchemas } from "./app-media-stream";
import { appPortalAuthSchemas } from "./app-portal-auth";
import { appRtcIotSchemas } from "./app-rtc-iot";
import { appSessionSchemas } from "./app-session";
import { commonApiSchemas } from "./common";
import { controlPlaneProtocolSchemas } from "./control-plane-protocol";
import { controlPlaneProviderSchemas } from "./control-plane-provider";
import { controlPlaneSocialSchemas } from "./control-plane-social";
import { platformBusinessSchemas } from "./platform-business";
import { platformOpsSchemas } from "./platform-ops";

export type {
  ApiSchemaDefinition,
  ApiSchemaDefinitionMap,
  ApiSchemaField,
} from "./schema-types";

export const apiSchemas = {
  ...commonApiSchemas,
  ...appPortalAuthSchemas,
  ...appSessionSchemas,
  ...appConversationSchemas,
  ...appMediaStreamSchemas,
  ...appRtcIotSchemas,
  ...platformBusinessSchemas,
  ...platformOpsSchemas,
  ...controlPlaneProtocolSchemas,
  ...controlPlaneProviderSchemas,
  ...controlPlaneSocialSchemas,
};

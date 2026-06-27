import { createIamRuntime } from "./iamRuntime";
import { bootstrapSdkClients } from "./sdkClients";

export function bootstrap() {
  createIamRuntime();
  bootstrapSdkClients();
}

export interface SdkworkAppConfig {
  apiBaseUrl?: string;
  authToken?: string;
  accessToken?: string;
}

export interface SdkworkAppClient {
  readonly config: SdkworkAppConfig;
  setAuthToken(token: string): this;
  setAccessToken(token: string): this;
}

export function createClient(options?: SdkworkAppConfig): SdkworkAppClient {
  return {
    config: options ?? {},
    setAuthToken(token: string) {
      this.config.authToken = token;
      return this;
    },
    setAccessToken(token: string) {
      this.config.accessToken = token;
      return this;
    },
  };
}

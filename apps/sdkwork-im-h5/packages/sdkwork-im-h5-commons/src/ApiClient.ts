export interface ApiConfig {
  baseURL: string;
  timeout: number;
  headers: Record<string, string>;
}

export interface ApiResponse<T = unknown> {
  code: number;
  data: T;
  message: string;
}

export class RawApiClientForbiddenError extends Error {
  constructor() {
    super("Raw HTTP is forbidden. Compose the generated owner SDK instead.");
    this.name = "RawApiClientForbiddenError";
  }
}

export class ApiClient {
  constructor(_config: ApiConfig) {}

  public async get<T>(
    _url: string,
    _params?: Record<string, string | number | boolean>,
  ): Promise<T> {
    throw new RawApiClientForbiddenError();
  }

  public async post<T>(_url: string, _data?: unknown): Promise<T> {
    throw new RawApiClientForbiddenError();
  }

  public async put<T>(_url: string, _data?: unknown): Promise<T> {
    throw new RawApiClientForbiddenError();
  }

  public async delete<T>(
    _url: string,
    _params?: Record<string, string | number | boolean>,
  ): Promise<T> {
    throw new RawApiClientForbiddenError();
  }
}

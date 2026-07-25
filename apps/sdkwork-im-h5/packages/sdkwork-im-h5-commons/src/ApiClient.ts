import { useTranslation } from "react-i18next";
export interface ApiConfig {
  baseURL: string;
  timeout: number;
  headers: Record<string, string>;
}

export interface ApiResponse<T = any> {
  code: number;
  data: T;
  message: string;
}

export class ApiClient {
  private config: ApiConfig;

  constructor(config: ApiConfig) {
    this.config = config;
  }

  private async request<T>(endpoint: string, options: RequestInit): Promise<T> {
    const url = `${this.config.baseURL}${endpoint}`;

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.config.timeout);

    try {
      const response = await fetch(url, {
        ...options,
        headers: {
          ...this.config.headers,
          ...(options.headers || {}),
        },
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        throw new Error(
          `HTTP Error: ${response.status} ${response.statusText}`,
        );
      }

      const responseData: ApiResponse<T> = await response.json();

      if (responseData.code !== 200 && responseData.code !== 0) {
        throw new Error(responseData.message || "API requests failed");
      }

      return responseData.data;
    } catch (error) {
      clearTimeout(timeoutId);
      console.error(
        `[ApiClient Error] ${options.method} ${url}:`,
        error.message,
      );
      throw error;
    }
  }

  public async get<T>(
    url: string,
    params?: Record<string, string | number | boolean>,
  ): Promise<T> {
    const query = params
      ? `?${new URLSearchParams(params as Record<string, string>).toString()}`
      : "";
    return this.request<T>(`${url}${query}`, {
      method: "GET",
    });
  }

  public async post<T>(url: string, data?: unknown): Promise<T> {
    return this.request<T>(url, {
      method: "POST",
      body: JSON.stringify(data),
    });
  }

  public async put<T>(url: string, data?: unknown): Promise<T> {
    return this.request<T>(url, {
      method: "PUT",
      body: JSON.stringify(data),
    });
  }

  public async delete<T>(
    url: string,
    params?: Record<string, string | number | boolean>,
  ): Promise<T> {
    const query = params
      ? `?${new URLSearchParams(params as Record<string, string>).toString()}`
      : "";
    return this.request<T>(`${url}${query}`, {
      method: "DELETE",
    });
  }
}

export const defaultApiClient = new ApiClient({
  baseURL: "https://api.yourcommericaldomain.com/v1",
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

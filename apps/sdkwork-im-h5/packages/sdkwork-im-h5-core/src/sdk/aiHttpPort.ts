export interface AiHttpResult<T> {
  ok: boolean;
  data?: T;
}

export interface AiHttpPort {
  postJson<T = unknown>(url: string, body: unknown): Promise<AiHttpResult<T>>;
  getText(url: string): Promise<AiHttpResult<string>>;
  getBlob(url: string): Promise<AiHttpResult<Blob>>;
}

export function createDefaultAiHttpPort(): AiHttpPort {
  return {
    async postJson<T = unknown>(url: string, body: unknown): Promise<AiHttpResult<T>> {
      const res = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      if (!res.ok) return { ok: false };
      const data = (await res.json()) as T;
      return { ok: true, data };
    },
    async getText(url: string): Promise<AiHttpResult<string>> {
      const res = await fetch(url);
      if (!res.ok) return { ok: false };
      const text = await res.text();
      return { ok: true, data: text };
    },
    async getBlob(url: string): Promise<AiHttpResult<Blob>> {
      const res = await fetch(url);
      if (!res.ok) return { ok: false };
      const blob = await res.blob();
      return { ok: true, data: blob };
    },
  };
}

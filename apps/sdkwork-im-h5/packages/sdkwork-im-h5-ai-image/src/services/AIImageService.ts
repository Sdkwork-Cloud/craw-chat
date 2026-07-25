import { useTranslation } from "react-i18next";
import { createDefaultAiHttpPort, type AiHttpPort } from "@sdkwork/im-h5-core";
export interface AIImageOptions {
  prompt: string;
  negativePrompt?: string;
  aspectRatio: "1:1" | "16:9" | "9:16" | "4:3";
  style: string;
}

export interface ImageTask {
  id: string;
  options: AIImageOptions;
  status: "pending" | "generating" | "completed" | "failed";
  progress: number;
  imageUrl?: string;
  createdAt: number;
}

const STORAGE_KEY = "sdkwork_im_h5_ai_image_history";

const aiHttp: AiHttpPort = createDefaultAiHttpPort();

export class AIImageService {
  private static getStoredHistory(): ImageTask[] {
    try {
      const data = localStorage.getItem(STORAGE_KEY);
      return data ? JSON.parse(data) : [];
    } catch {
      return [];
    }
  }

  public static deleteFromHistory(id: string) {
    const history = this.getStoredHistory().filter((t) => t.id !== id);
    this.saveHistory(history);
  }

  public static async optimizePrompt(prompt: string): Promise<string> {
    if (!prompt.trim()) return prompt;
    try {
      const result = await aiHttp.postJson<{ result?: string }>("/api/ai/optimize-prompt", { prompt });
      if (result.ok && result.data?.result) return result.data.result;
    } catch (e) {
      console.error("Optimize prompt backend failed:", e);
    }

    try {
      const p = `Optimize this image generation prompt to be extremely detailed, beautiful and artistic: "${prompt}". Respond ONLY with the optimized English prompt and absolutely nothing else. Keep it brief and vivid.`;
      const result = await aiHttp.getText(`https://text.pollinations.ai/${encodeURIComponent(p)}`);
      if (result.ok && result.data) return result.data.trim();
    } catch (e) {
      console.error("Pollinations prompt optimization failed:", e);
    }

    return `${prompt}, masterpiece, 8k resolution, highly detailed, photorealistic, cinematic lighting`;
  }

  private static saveHistory(history: ImageTask[]) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(history.slice(0, 20)));
    } catch (e) {
      console.error("Failed to save to local storage", e);
    }
  }

  public static async generateImage(
    options: AIImageOptions,
    onProgress?: (progress: number) => void,
  ): Promise<ImageTask> {
    const task: ImageTask = {
      id: Date.now().toString(),
      options,
      status: "generating",
      progress: 0,
      createdAt: Date.now(),
    };

    // Save initial state
    const history = this.getStoredHistory();
    this.saveHistory([task, ...history]);

    try {
      await aiHttp.postJson("/api/ai/image", options);
      // We'll skip this and use the simulation fallback below to maintain the visual generation delay
      // because our backend currently just returns the mock after 2s anyway.
    } catch (e) {
      console.error("Backend image generation failed", e);
    }

    // Simulation fallback
    return new Promise((resolve) => {
      let currentProgress = 0;

      const interval = setInterval(() => {
        currentProgress += Math.floor(Math.random() * 15) + 5;
        if (currentProgress > 95) currentProgress = 95;

        onProgress?.(currentProgress);
      }, 500);

      setTimeout(
        () => {
          clearInterval(interval);

          let w = 1080;
          let h = 1080;
          if (options.aspectRatio === "16:9") {
            w = 1920;
            h = 1080;
          } else if (options.aspectRatio === "9:16") {
            w = 1080;
            h = 1920;
          } else if (options.aspectRatio === "4:3") {
            w = 1600;
            h = 1200;
          }

          // Use pollinations.ai for actual AI generation based on prompt in fallback
          let promptStr = options.prompt;
          if (options.style && options.style !== "None")
            promptStr += `, ${options.style} style`;
          let imgUrl = `https://image.pollinations.ai/prompt/${encodeURIComponent(promptStr)}?width=${w}&height=${h}&nologo=true&seed=${Math.floor(Math.random() * 10000)}`;

          const completedTask: ImageTask = {
            ...task,
            status: "completed",
            progress: 100,
            imageUrl: imgUrl,
          };

          const updatedHistory = this.getStoredHistory().map((t) =>
            t.id === task.id ? completedTask : t,
          );
          this.saveHistory(updatedHistory);

          onProgress?.(100);
          resolve(completedTask);
        },
        4000 + Math.random() * 2000,
      );
    });
  }

  public static async getHistory(): Promise<ImageTask[]> {
    return this.getStoredHistory();
  }

  public static async downloadBlob(url: string): Promise<Blob> {
    const result = await aiHttp.getBlob(url);
    if (!result.ok || !result.data) {
      throw new Error("Image download failed");
    }
    return result.data;
  }
}

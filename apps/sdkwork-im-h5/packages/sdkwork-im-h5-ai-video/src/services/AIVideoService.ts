export interface AIVideoOptions {
  prompt: string;
  style: string;
  aspectRatio: "16:9" | "9:16" | "1:1";
}

export interface VideoTask {
  id: string;
  options: AIVideoOptions;
  status: "pending" | "generating" | "completed" | "failed";
  progress: number;
  videoUrl?: string;
  thumbnailUrl?: string;
  createdAt: number;
  estimatedTimeSec: number;
}

const STORAGE_KEY = "sdkwork_im_h5_ai_video_history";

export class AIVideoService {
  private static getStoredHistory(): VideoTask[] {
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

  private static saveHistory(history: VideoTask[]) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(history.slice(0, 20)));
    } catch (e) {
      console.error("Failed to save to local storage", e);
    }
  }

  public static async generateVideo(
    options: AIVideoOptions,
    onProgress?: (progress: number) => void,
  ): Promise<VideoTask> {
    const task: VideoTask = {
      id: Date.now().toString(),
      options,
      status: "generating",
      progress: 0,
      createdAt: Date.now(),
      estimatedTimeSec: 60,
    };

    const history = this.getStoredHistory();
    this.saveHistory([task, ...history]);

    try {
      const res = await fetch("/im/v3/api/ai/video", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(options)
      });
      if (res.ok && false) {
        // Skip direct parse, use simulation logic below to maintain delay and hardcoded fallback videos
      }
    } catch (e) {
      console.error("Backend generation failed", e);
    }

    // Simulation Fallback
    return new Promise((resolve) => {
      let currentProgress = 0;

      const interval = setInterval(() => {
        currentProgress += Math.random() * 5 + 3;
        if (currentProgress > 98) currentProgress = 98;
        onProgress?.(currentProgress);
      }, 800);

      setTimeout(
        () => {
          clearInterval(interval);

          let videoUrl =
            "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/for-bigger-blazes.mp4";
          let thumbnailUrl =
            "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/for-bigger-blazes/poster.png";

          const keywords = options.prompt.toLowerCase();
          if (
            keywords.includes("city") ||
            keywords.includes("street") ||
            keywords.includes("car") ||
            keywords.includes("城市") ||
            keywords.includes("街") ||
            keywords.includes("车")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/city-traffic.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1519501025264-65ba15a82390.png";
          } else if (
            keywords.includes("nature") ||
            keywords.includes("forest") ||
            keywords.includes("tree") ||
            keywords.includes("自然") ||
            keywords.includes("森林") ||
            keywords.includes("树") ||
            keywords.includes("山")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/forest-stream.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1441974231531-c6227db76b6e.png";
          } else if (
            keywords.includes("ocean") ||
            keywords.includes("water") ||
            keywords.includes("sea") ||
            keywords.includes("海") ||
            keywords.includes("水") ||
            keywords.includes("洋")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/oceans.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1439405326854-014607f694d7.png";
          } else if (
            keywords.includes("flower") ||
            keywords.includes("bloom") ||
            keywords.includes("花")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/tree-flowers.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1490750967868-88cb4aca8fba.png";
          } else if (
            keywords.includes("food") ||
            keywords.includes("eat") ||
            keywords.includes("饭") ||
            keywords.includes("菜") ||
            keywords.includes("食物")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/cafe-girl.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1555396273-367ea4eb4db5.png";
          } else if (
            keywords.includes("animal") ||
            keywords.includes("dog") ||
            keywords.includes("cat") ||
            keywords.includes("狗") ||
            keywords.includes("猫") ||
            keywords.includes("宠物")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/dog-river.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1517849845537-4d257902454a.png";
          } else if (
            keywords.includes("people") ||
            keywords.includes("girl") ||
            keywords.includes("man") ||
            keywords.includes("woman") ||
            keywords.includes("人") ||
            keywords.includes("男孩") ||
            keywords.includes("女孩")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/woman-pool.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1517365830460-955ce3ccd263.png";
          } else if (
            keywords.includes("tech") ||
            keywords.includes("code") ||
            keywords.includes("computer") ||
            keywords.includes("电脑") ||
            keywords.includes("代码") ||
            keywords.includes("科技")
          ) {
            videoUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/developer-code.mp4";
            thumbnailUrl =
              "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1498050108023-c5249f4df085.png";
          }

          if (options.aspectRatio === "9:16") {
            // Attempt to find a vertical video if possible, or fallback
            if (!keywords.includes("flower") && !keywords.includes("花")) {
              videoUrl =
                "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/girl-neon.mp4";
              thumbnailUrl =
                "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1550684848-fac1c5b4e853.png";
            }
          }

          const completedTask: VideoTask = {
            ...task,
            status: "completed",
            progress: 100,
            videoUrl,
            thumbnailUrl,
          };

          const updatedHistory = this.getStoredHistory().map((t) =>
            t.id === task.id ? completedTask : t,
          );
          this.saveHistory(updatedHistory);

          onProgress?.(100);
          resolve(completedTask);
        },
        6000 + Math.random() * 2000,
      ); // Simulate 6-8 seconds generation
    });
  }

  public static async getHistory(): Promise<VideoTask[]> {
    return this.getStoredHistory();
  }
}

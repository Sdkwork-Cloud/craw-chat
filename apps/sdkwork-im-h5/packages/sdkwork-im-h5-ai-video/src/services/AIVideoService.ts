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

export class AIVideoCapabilityUnavailableError extends Error {
  constructor(capability: "video generation" | "video history") {
    super(`AI ${capability} is unavailable because no owner SDK is composed.`);
    this.name = "AIVideoCapabilityUnavailableError";
  }
}

export class AIVideoService {
  public static deleteFromHistory(_id: string): never {
    throw new AIVideoCapabilityUnavailableError("video history");
  }

  public static async generateVideo(
    _options: AIVideoOptions,
    _onProgress?: (progress: number) => void,
  ): Promise<VideoTask> {
    throw new AIVideoCapabilityUnavailableError("video generation");
  }

  public static async getHistory(): Promise<VideoTask[]> {
    throw new AIVideoCapabilityUnavailableError("video history");
  }
}

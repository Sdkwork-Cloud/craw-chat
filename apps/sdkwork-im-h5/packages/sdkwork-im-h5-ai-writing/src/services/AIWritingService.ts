export interface AIWritingOptions {
  topic: string;
  style: string;
  length: "short" | "medium" | "long";
  language: "English" | "Chinese";
}

export interface WritingTask {
  id: string;
  options: AIWritingOptions;
  status: "pending" | "generating" | "completed" | "failed";
  content?: string;
  createdAt: number;
}

export class AIWritingCapabilityUnavailableError extends Error {
  constructor(capability: "article generation" | "writing history") {
    super(`AI ${capability} is unavailable because no owner SDK is composed.`);
    this.name = "AIWritingCapabilityUnavailableError";
  }
}

export class AIWritingService {
  public static deleteFromHistory(_id: string): never {
    throw new AIWritingCapabilityUnavailableError("writing history");
  }

  public static async generateArticle(
    _options: AIWritingOptions,
    _onChunk?: (chunk: string) => void,
  ): Promise<WritingTask> {
    throw new AIWritingCapabilityUnavailableError("article generation");
  }

  public static async getHistory(): Promise<WritingTask[]> {
    throw new AIWritingCapabilityUnavailableError("writing history");
  }
}

import { useTranslation } from "react-i18next";
import { createDefaultAiHttpPort, type AiHttpPort } from "@sdkwork/im-h5-core";
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

const STORAGE_KEY = "sdkwork_im_h5_ai_writing_history";

const aiHttp: AiHttpPort = createDefaultAiHttpPort();

export class AIWritingService {
  private static getStoredHistory(): WritingTask[] {
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

  private static saveHistory(history: WritingTask[]) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(history.slice(0, 20)));
    } catch (e) {
      console.error("Failed to save to local storage", e);
    }
  }

  public static async generateArticle(
    options: AIWritingOptions,
    onChunk?: (chunk: string) => void,
  ): Promise<WritingTask> {
    const task: WritingTask = {
      id: Date.now().toString(),
      options,
      status: "generating",
      createdAt: Date.now(),
    };

    const history = this.getStoredHistory();
    this.saveHistory([task, ...history]);

    let content = "";

    try {
      const result = await aiHttp.postJson<{ content?: string }>("/api/ai/writing", {
        prompt: `Topic: ${options.topic}\nLanguage: ${options.language}`,
        type: "article",
        tone: options.style,
        length: options.length,
      });
      if (result.ok && result.data?.content) {
        const backendContent = result.data.content;
        const words = options.language === "English" ? backendContent.split(/(?<=\s)/) : backendContent.split("");
        let i = 0;
        const chunkSize = options.language === "English" ? 4 : 8;
        return new Promise((resolve) => {
          const interval = setInterval(() => {
            if (i < words.length) {
              const chunkStr = words.slice(i, i + chunkSize).join("");
              content += chunkStr;
              onChunk?.(content);
              i += chunkSize;
            } else {
              clearInterval(interval);
              const completedTask: WritingTask = { ...task, status: "completed", content };
              const updatedHistory = this.getStoredHistory().map((t) => t.id === task.id ? completedTask : t);
              this.saveHistory(updatedHistory);
              resolve(completedTask);
            }
          }, 50);
        });
      }
    } catch (e) {
      console.error("Backend generation failed, falling back to simulation", e);
    }

    // Simulation Fallback
    return new Promise(async (resolve) => {
      let content = "";
      let generatedText = "";

      try {
        const prompt = `Write a ${options.length} article about "${options.topic}" in ${options.language} language. Style/Tone: ${options.style}. Use clean Markdown formatting with ## headings, **bold**, and bullet points.`;
        const result = await aiHttp.getText(
          `https://text.pollinations.ai/${encodeURIComponent(prompt)}`,
        );
        if (result.ok && result.data) {
          generatedText = result.data;
        }
      } catch (e) {
        console.warn("Pollinations text API failed, using local mock.");
      }

      if (!generatedText) {
        if (options.language === "English") {
          const styleText = options.style.toLowerCase();
          generatedText = `# ${options.topic}

Welcome to this ${styleText} overview of ${options.topic}. We are seeing unprecedented opportunities and challenges in this domain.

## The Core Concept
At its heart, **${options.topic}** is about shifting paradigms. We've seen a massive transition recently. This isn't just about technology—it's about culture and adaptation within a ${styleText} context.

## Key Insights
1. **Enhanced Efficiency**: Processes are taking dramatically less time.
2. **Global Reach**: Breaking down geographical barriers effortlessly.
3. **Innovative Horizons**: Sparking creativity in unforeseen ways.

### Future Outlook
The trajectory for ${options.topic} is incredibly promising. Leaders in the space emphasize the need for continuous learning.

*Conclusion*
Embracing these changes allows us to stay ahead of the curve. The future favors those who adapt.`;
        } else {
          const styleText =
            options.style === "Professional"
              ? "专业"
              : options.style === "Casual"
                ? "轻松"
                : "创新";
          generatedText = `# 关于 ${options.topic} 的${styleText}探讨

随着时代的发展，**${options.topic}** 已经成为我们无法忽视的核心话题。在此，我们将以${styleText}的视角探讨它的发展脉络和未来趋势。

## 核心概念
从本质上讲，${options.topic} 代表着一种范式的转变。不仅仅是基础架构的升级，更是思维方式的革新。

## 关键优势
- **卓越的效率提升**：将耗时的繁琐流程大幅缩减。
- **无界连接**：打破地域限制，连接更加广阔的生态。
- **无限创新空间**：激发潜能，拓宽创意的边界。

### 行业展望
针对 ${options.topic} 的未来，业内充满期待。我们需要保持敏锐的洞察力，跟随前沿。

*总结*
拥抱变化是持续发展的唯一途径。未来属于拥抱创新的先行者。`;
        }

        if (options.length === "short") {
          generatedText =
            generatedText.split("## Key Insights")[0] ||
            generatedText.split("## 关键优势")[0];
        } else if (options.length === "long") {
          generatedText +=
            options.language === "English"
              ? `\n\n## Deep Dive Analysis\nWhen analyzing ${options.topic} further, we find that the implications are vast. Organizations that integrate these concepts early often see exponential returns on investment. The key is strategic implementation rather than hasty adoption. By continuing to focus on core fundamentals while exploring ${options.topic}, a sustainable advantage can be achieved.`
              : `\n\n## 深度解析\n进一步分析 ${options.topic}，我们会发现其影响是极其深远的。尽早整合这些概念的组织往往能看到指数级的投资回报。关键在于战略性的实施，而非仓促跟风。在坚持核心基础的同时，深入探索 ${options.topic}，方能取得可持续的竞争优势。`;
        }
      }

      const words =
        options.language === "English"
          ? generatedText.split(/(?<=\s)/)
          : generatedText.split("");
      let i = 0;
      const chunkSize = options.language === "English" ? 4 : 8;

      const interval = setInterval(() => {
        if (i < words.length) {
          const chunkElements = words.slice(i, i + chunkSize);
          const chunkStr = chunkElements.join("");
          content += chunkStr;
          onChunk?.(content);
          i += chunkSize;
        } else {
          clearInterval(interval);

          const completedTask: WritingTask = {
            ...task,
            status: "completed",
            content,
          };

          const updatedHistory = this.getStoredHistory().map((t) =>
            t.id === task.id ? completedTask : t,
          );
          this.saveHistory(updatedHistory);

          resolve(completedTask);
        }
      }, 50); // Simulate streaming
    });
  }

  public static async getHistory(): Promise<WritingTask[]> {
    return this.getStoredHistory();
  }
}

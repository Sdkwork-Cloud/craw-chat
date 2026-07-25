import express from "express";
import path from "path";
import { createServer as createViteServer } from "vite";
import { GoogleGenAI } from "@google/genai";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function startServer() {
  const app = express();
  const PORT = 3000;

  app.use(express.json());

  // Helper checking for Gemini Key
  const getAiClient = () => {
    if (!process.env.GEMINI_API_KEY) {
      throw new Error("GEMINI_API_KEY is not set");
    }
    return new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
  };

  // AI Prompt Optimize API
  app.post("/api/ai/optimize-prompt", async (req, res) => {
    try {
      const { prompt } = req.body;
      if (!prompt) return res.status(400).json({ error: "No prompt" });
      const ai = getAiClient();
      const response = await ai.models.generateContent({
        model: "gemini-2.5-flash",
        contents: `Optimize this simple image prompt into an incredibly detailed, artistic, and cinematic prompt for AI image generators like Stable Diffusion or Midjourney. Focus on visual descriptors, lighting, environment, and style. Keep it under 50 words: "${prompt}"`,
      });
      res.json({ result: response.text?.trim() || prompt });
    } catch (e: any) {
      console.error(e);
      res.status(500).json({ error: e.message || "Failed to optimize prompt" });
    }
  });

  // Since we don't have real integrations for images/video generation in the mock UI, 
  // we just simulate the Gemini response generation or fake video URL as in the original service
  
  app.post("/api/ai/writing", async (req, res) => {
    try {
      const { prompt, type, tone, length } = req.body;
      const ai = getAiClient();
      
      const lengthMap = {
        short: "Keep it brief, around 50-100 words.",
        medium: "Aim for a medium length, around 150-300 words.",
        long: "Provide a comprehensive response, around 400-600 words.",
      };
      
      const promptText = `You are a professional writing assistant. 
Task: Generate a ${type} with a ${tone} tone. 
Topic/Instructions: ${prompt}
Length Constraint: ${lengthMap[length as keyof typeof lengthMap] || ""}
Output ONLY the requested content, properly formatted, without any conversational filler or introductions.`;

      const response = await ai.models.generateContent({
        model: "gemini-2.5-flash",
        contents: promptText,
      });

      res.json({ content: response.text || "" });
    } catch (e: any) {
      console.error(e);
      res.status(500).json({ error: e.message });
    }
  });

  app.post("/api/ai/image", async (req, res) => {
    try {
       // Since the original was a mock returning placeholder images, we'll maintain the mock,
       // but add a small delay to simulate processing. The prompt optimization already uses Gemini above
       setTimeout(() => {
          res.json({ 
             imageUrl: `https://picsum.photos/seed/${Date.now()}/800/800`
          });
       }, 2000);
    } catch (e: any) {
      res.status(500).json({ error: e.message });
    }
  });

  app.post("/api/ai/video", async (req, res) => {
    try {
      // Mock video response simulating the original behavior
      setTimeout(() => {
        res.json({
           videoUrl: "https://www.w3schools.com/html/mov_bbb.mp4",
           thumbnailUrl: `https://picsum.photos/seed/${Date.now()}/400/300`
        });
      }, 3000);
    } catch (e: any) {
      res.status(500).json({ error: e.message });
    }
  });

  // Vite middleware for development
  if (process.env.NODE_ENV !== "production") {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: "spa",
    });
    app.use(vite.middlewares);
  } else {
    const distPath = path.join(process.cwd(), "dist");
    app.use(express.static(distPath));
    app.get("*", (req, res) => {
      res.sendFile(path.join(distPath, "index.html"));
    });
  }

  app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server running on http://localhost:${PORT}`);
  });
}

startServer();

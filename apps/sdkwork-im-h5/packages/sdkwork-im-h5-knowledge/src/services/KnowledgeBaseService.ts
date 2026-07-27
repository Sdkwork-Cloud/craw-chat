export interface KnowledgeBase {
  id: string;
  name: string;
  description: string;
  icon: string;
  color?: string;
  isArchived?: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface KnowledgeDocument {
  id: string;
  kbId: string;
  title: string;
  content: string;
  category: string;
  author: string;
  createdAt: string;
  updatedAt: string;
}

const KB_STORAGE_KEY = "sdkwork_im_h5_knowledge_bases";
const DOC_STORAGE_KEY = "sdkwork_im_h5_knowledge_docs";

export class KnowledgeBaseService {
  // --- Knowledge Base ---
  static async getKnowledgeBases(): Promise<KnowledgeBase[]> {
    const data = localStorage.getItem(KB_STORAGE_KEY);
    if (!data) return [];
    try {
      return JSON.parse(data);
    } catch {
      return [];
    }
  }

  static async getKnowledgeBase(id: string): Promise<KnowledgeBase | null> {
    const kbs = await this.getKnowledgeBases();
    return kbs.find(kb => kb.id === id) || null;
  }

  static async createKnowledgeBase(kb: Omit<KnowledgeBase, "id" | "createdAt" | "updatedAt">): Promise<KnowledgeBase> {
    const kbs = await this.getKnowledgeBases();
    const newKb: KnowledgeBase = {
      ...kb,
      id: crypto.randomUUID(),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    kbs.push(newKb);
    localStorage.setItem(KB_STORAGE_KEY, JSON.stringify(kbs));
    return newKb;
  }

  static async updateKnowledgeBase(id: string, updates: Partial<KnowledgeBase>): Promise<KnowledgeBase | null> {
    const kbs = await this.getKnowledgeBases();
    const index = kbs.findIndex(kb => kb.id === id);
    if (index === -1) return null;
    kbs[index] = { ...kbs[index], ...updates, updatedAt: new Date().toISOString() };
    localStorage.setItem(KB_STORAGE_KEY, JSON.stringify(kbs));
    return kbs[index];
  }

  static async deleteKnowledgeBase(id: string): Promise<void> {
    await this.deleteKnowledgeBases([id]);
  }

  static async deleteKnowledgeBases(ids: string[]): Promise<void> {
    if (!ids || ids.length === 0) return;
    const idSet = new Set(ids);
    let kbs = await this.getKnowledgeBases();
    kbs = kbs.filter(kb => !idSet.has(kb.id));
    localStorage.setItem(KB_STORAGE_KEY, JSON.stringify(kbs));
    
    // Cascading delete for documents
    let docs = await this.getAllDocuments();
    docs = docs.filter(doc => !idSet.has(doc.kbId));
    localStorage.setItem(DOC_STORAGE_KEY, JSON.stringify(docs));
  }

  static async archiveKnowledgeBases(ids: string[]): Promise<void> {
    if (!ids || ids.length === 0) return;
    const idSet = new Set(ids);
    let kbs = await this.getKnowledgeBases();
    const now = new Date().toISOString();
    kbs = kbs.map(kb => idSet.has(kb.id) ? { ...kb, isArchived: true, updatedAt: now } : kb);
    localStorage.setItem(KB_STORAGE_KEY, JSON.stringify(kbs));
  }

  static async unarchiveKnowledgeBases(ids: string[]): Promise<void> {
    if (!ids || ids.length === 0) return;
    const idSet = new Set(ids);
    let kbs = await this.getKnowledgeBases();
    const now = new Date().toISOString();
    kbs = kbs.map(kb => idSet.has(kb.id) ? { ...kb, isArchived: false, updatedAt: now } : kb);
    localStorage.setItem(KB_STORAGE_KEY, JSON.stringify(kbs));
  }

  // --- Documents ---
  static async getAllDocuments(): Promise<KnowledgeDocument[]> {
    const data = localStorage.getItem(DOC_STORAGE_KEY);
    if (!data) return [];
    try {
      return JSON.parse(data);
    } catch {
      return [];
    }
  }

  static async getDocumentsByKbId(kbId: string): Promise<KnowledgeDocument[]> {
    const docs = await this.getAllDocuments();
    return docs.filter(doc => doc.kbId === kbId);
  }

  static async getDocument(id: string): Promise<KnowledgeDocument | null> {
    const docs = await this.getAllDocuments();
    return docs.find(doc => doc.id === id) || null;
  }

  static async createDocument(doc: Omit<KnowledgeDocument, "id" | "createdAt" | "updatedAt">): Promise<KnowledgeDocument> {
    const docs = await this.getAllDocuments();
    const newDoc: KnowledgeDocument = {
      ...doc,
      id: crypto.randomUUID(),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
    docs.push(newDoc);
    localStorage.setItem(DOC_STORAGE_KEY, JSON.stringify(docs));
    return newDoc;
  }

  static async updateDocument(id: string, updates: Partial<KnowledgeDocument>): Promise<KnowledgeDocument | null> {
    const docs = await this.getAllDocuments();
    const index = docs.findIndex(doc => doc.id === id);
    if (index === -1) return null;
    docs[index] = { ...docs[index], ...updates, updatedAt: new Date().toISOString() };
    localStorage.setItem(DOC_STORAGE_KEY, JSON.stringify(docs));
    return docs[index];
  }

  static async deleteDocument(id: string): Promise<void> {
    let docs = await this.getAllDocuments();
    docs = docs.filter(doc => doc.id !== id);
    localStorage.setItem(DOC_STORAGE_KEY, JSON.stringify(docs));
  }
}

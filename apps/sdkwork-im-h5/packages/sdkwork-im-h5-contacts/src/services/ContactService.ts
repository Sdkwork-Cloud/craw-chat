import { useTranslation } from "react-i18next";
import type { User } from "@sdkwork/im-h5-types";

const INITIAL_CONTACTS: User[] = [
  {
    id: "u1",
    name: "Alex Chen",
    avatar: "https://picsum.photos/seed/alex/200/200",
    status: "online",
  },
  {
    id: "u2",
    name: "Sarah Jenkins",
    avatar: "https://picsum.photos/seed/sarah/200/200",
    status: "online",
  },
  {
    id: "u3",
    name: "David Lee",
    avatar: "https://picsum.photos/seed/david/200/200",
  },
  {
    id: "u4",
    name: "Emily Chen",
    avatar: "https://picsum.photos/seed/emily/200/200",
  },
  {
    id: "u5",
    name: "Michael Brown",
    avatar: "https://picsum.photos/seed/michael/200/200",
  },
  {
    id: "u6",
    name: "Alice Wong",
    avatar: "https://picsum.photos/seed/a1/200/200",
  },
  {
    id: "u7",
    name: "Bob Lee",
    avatar: "https://picsum.photos/seed/b1/200/200",
  },
  {
    id: "u8",
    name: "Charlie",
    avatar: "https://picsum.photos/seed/c1/200/200",
  },
  { id: "u9", name: "Cindy", avatar: "https://picsum.photos/seed/c2/200/200" },
  {
    id: "u10",
    name: "David Tao",
    avatar: "https://picsum.photos/seed/d1/200/200",
  },
  { id: "u11", name: "Frank", avatar: "https://picsum.photos/seed/f1/200/200" },
];

const STORAGE_KEY = "sdkwork_im_h5_contacts";

export let MOCK_CONTACTS: User[] = [];

const loadContacts = () => {
  if (MOCK_CONTACTS.length > 0) return MOCK_CONTACTS;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_CONTACTS = JSON.parse(data);
    } else {
      MOCK_CONTACTS = [...INITIAL_CONTACTS];
      saveContacts();
    }
  } catch (e) {
    MOCK_CONTACTS = [...INITIAL_CONTACTS];
  }
  return MOCK_CONTACTS;
};

const saveContacts = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_CONTACTS));
  } catch (e) {
    console.error("Failed to save contacts data", e);
  }
};

export interface Contact {
  id: string;
  name: string;
  avatar: string;
  phone?: string;
  desc?: string;
}

export const ContactService = {
  async getContactsDict(): Promise<Record<string, Contact[]>> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const contacts = loadContacts().filter(
          (u) => u.id !== "u1" && !u.id.startsWith("agent_"),
        );
        const dict: Record<string, Contact[]> = {};

        contacts.forEach((c) => {
          const firstChar = (c.name || "#").charAt(0).toUpperCase();
          const letter = /[A-Z]/.test(firstChar) ? firstChar : "#";
          if (!dict[letter]) dict[letter] = [];
          dict[letter].push({
            id: c.id,
            name: c.name,
            avatar: c.avatar || "",
            status: c.status,
          } as Contact);
        });

        // Sort keys
        const sortedDict: Record<string, Contact[]> = {};
        Object.keys(dict)
          .sort()
          .forEach((key) => {
            sortedDict[key] = dict[key].sort((a, b) =>
              a.name.localeCompare(b.name),
            );
          });

        resolve(sortedDict);
      }, 300);
    });
  },

  async getContacts(): Promise<User[]> {
    return loadContacts().filter(
      (u) => u.id !== "u1" && !u.id.startsWith("agent_"),
    );
  },

  async searchContacts(query: string): Promise<User[]> {
    if (!query.trim()) return [];
    const lowerQuery = query.toLowerCase();
    const contacts = await this.getContacts();
    return contacts.filter((c) => c.name.toLowerCase().includes(lowerQuery));
  },

  async addFriend(query: string): Promise<User> {
    loadContacts();
    const newUser: User = {
      id: `u${Date.now()}`,
      name: query,
      avatar: `https://picsum.photos/seed/${query}/200/200`,
      status: "online",
    };
    MOCK_CONTACTS = [...MOCK_CONTACTS, newUser];
    saveContacts();
    return newUser;
  },

  async createDirectChat(user: User): Promise<any> {
    const { ChatService } = await import("@sdkwork/im-h5-chat");
    return ChatService.createDirectChat(user);
  },

  async searchFriend(query: string): Promise<User | null> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve({
          id: `u_${Date.now()}`,
          name: query,
          avatar: `https://picsum.photos/seed/${query}/200/200`,
          status: "online",
        });
      }, 500);
    });
  },
};

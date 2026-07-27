import type { User } from "@sdkwork/im-h5-types";

const INITIAL_CONTACTS: User[] = [
  {
    id: "u1",
    name: "Alex Chen",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/alex/200x200.png",
    status: "online",
  },
  {
    id: "u2",
    name: "Sarah Jenkins",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/sarah/200x200.png",
    status: "online",
  },
  {
    id: "u3",
    name: "David Lee",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/david/200x200.png",
  },
  {
    id: "u4",
    name: "Emily Chen",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/emily/200x200.png",
  },
  {
    id: "u5",
    name: "Michael Brown",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/michael/200x200.png",
  },
  {
    id: "u6",
    name: "Alice Wong",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/a1/200x200.png",
  },
  {
    id: "u7",
    name: "Bob Lee",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/b1/200x200.png",
  },
  {
    id: "u8",
    name: "Charlie",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c1/200x200.png",
  },
  { id: "u9", name: "Cindy", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c2/200x200.png" },
  {
    id: "u10",
    name: "David Tao",
    avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/d1/200x200.png",
  },
  { id: "u11", name: "Frank", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/f1/200x200.png" },
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
      avatar: `https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/avatars/${query}/200.png`,
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
          avatar: `https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/avatars/${query}/200.png`,
          status: "online",
        });
      }, 500);
    });
  },
};

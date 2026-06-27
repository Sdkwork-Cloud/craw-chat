import type { MailAppSdkClient, MailFolder, MailMessage } from '@sdkwork/mail-app-sdk';
import {
  extractMailEntity,
  extractMailItems,
  mailMessageRecord,
  readOptionalString,
  readString,
} from '@sdkwork/im-pc-core/sdk/mailApiHelpers';
import { getMailAppSdkClientWithSession } from '@sdkwork/im-pc-core/sdk/mailAppSdkClient';

export interface MailItem {
  id: string;
  senderName: string;
  senderEmail: string;
  time: string;
  subject: string;
  previewText: string;
  bodyHtml?: string;
  attachments?: { name: string; size: string; type: string }[];
  isRead: boolean;
  isStarred: boolean;
  folder: 'inbox' | 'sent' | 'trash' | 'drafts';
}

export interface MailService {
  getMails(folder: string): Promise<MailItem[]>;
  getMailById(id: string): Promise<MailItem | null>;
  markAsRead(id: string): Promise<void>;
  markAsUnread(id: string): Promise<void>;
  toggleStar(id: string): Promise<void>;
  deleteMail(id: string): Promise<void>;
  sendMail(mail: Partial<MailItem>): Promise<MailItem>;
}

const PC_MAIL_SEND_UNAVAILABLE =
  'pc mail send contract requires a configured mail account and sent folder';

interface MailServiceOptions {
  client?: MailAppSdkClient;
}

type UiFolder = MailItem['folder'];

const UI_FOLDER_KINDS: Record<string, UiFolder> = {
  inbox: 'inbox',
  sent: 'sent',
  trash: 'trash',
  drafts: 'drafts',
};

interface MailRuntimeContext {
  accountId: string;
  folderIds: Partial<Record<UiFolder, string>>;
}

function normalizeFolderKind(value: string | undefined): UiFolder | undefined {
  const normalized = value.trim().toLowerCase();
  return UI_FOLDER_KINDS[normalized];
}

function resolveUiFolder(folder: string): UiFolder {
  return UI_FOLDER_KINDS[folder.trim().toLowerCase()] ?? 'inbox';
}

function mapMailMessage(message: MailMessage, folder: UiFolder): MailItem {
  const record = mailMessageRecord(message);
  const fromEmail = readString(record, 'fromEmail', 'from_email');
  const subject = readString(record, 'subject');
  const snippet = readString(record, 'snippet', 'bodyText', 'body_text');
  const bodyHtml = readOptionalString(record, 'bodyHtml', 'body_html');
  return {
    id: readString(record, 'id'),
    senderName: fromEmail.split('@')[0] || 'Unknown',
    senderEmail: fromEmail,
    time: new Date().toISOString(),
    subject,
    previewText: snippet,
    bodyHtml,
    isRead: Boolean(record.isRead ?? record.is_read),
    isStarred: Boolean(record.isStarred ?? record.is_starred),
    folder,
  };
}

class SdkworkMailService implements MailService {
  private readonly clientFactory: () => MailAppSdkClient;
  private runtimeContext: MailRuntimeContext | null = null;

  constructor(options: MailServiceOptions = {}) {
    this.clientFactory = () => options.client ?? getMailAppSdkClientWithSession();
  }

  private client(): MailAppSdkClient {
    return this.clientFactory();
  }

  private async resolveRuntimeContext(): Promise<MailRuntimeContext | null> {
    if (this.runtimeContext) {
      return this.runtimeContext;
    }
    const accountsResponse = await this.client().mailAccounts.mail.accounts.list();
    const accounts = extractMailItems<{ id?: string }>(accountsResponse);
    const accountId = accounts.find((account) => typeof account.id === 'string' && account.id.length > 0)?.id;
    if (!accountId) {
      return null;
    }
    const foldersResponse = await this.client().mailFolders.mail.folders.list({ accountId });
    const folders = extractMailItems<MailFolder>(foldersResponse);
    const folderIds: Partial<Record<UiFolder, string>> = {};
    for (const folder of folders) {
      const folderKind = normalizeFolderKind(folder.folderKind);
      if (folderKind && folder.id) {
        folderIds[folderKind] = folder.id;
      }
    }
    this.runtimeContext = { accountId, folderIds };
    return this.runtimeContext;
  }

  private async resolveFolderId(folder: string): Promise<string | null> {
    const context = await this.resolveRuntimeContext();
    if (!context) {
      return null;
    }
    return context.folderIds[resolveUiFolder(folder)] ?? null;
  }

  async getMails(folder: string): Promise<MailItem[]> {
    const folderId = await this.resolveFolderId(folder);
    if (!folderId) {
      return [];
    }
    const response = await this.client().mailMessages.mail.messages.list({ folderId });
    const uiFolder = resolveUiFolder(folder);
    return extractMailItems<MailMessage>(response).map((message) => mapMailMessage(message, uiFolder));
  }

  async getMailById(id: string): Promise<MailItem | null> {
    const response = await this.client().mailMessages.mail.messages.retrieve(id);
    const message = extractMailEntity<MailMessage>(response);
    if (!message?.id) {
      return null;
    }
    const record = mailMessageRecord(message);
    const folderId = readOptionalString(record, 'folderId', 'folder_id');
    const context = await this.resolveRuntimeContext();
    let folder: UiFolder = 'inbox';
    if (context && folderId) {
      for (const [kind, resolvedFolderId] of Object.entries(context.folderIds) as [UiFolder, string][]) {
        if (resolvedFolderId === folderId) {
          folder = kind;
          break;
        }
      }
    }
    return mapMailMessage(message, folder);
  }

  async markAsRead(id: string): Promise<void> {
    await this.client().mailMessages.mail.messages.update(id, { isRead: true });
  }

  async markAsUnread(id: string): Promise<void> {
    await this.client().mailMessages.mail.messages.update(id, { isRead: false });
  }

  async toggleStar(id: string): Promise<void> {
    const current = await this.getMailById(id);
    await this.client().mailMessages.mail.messages.update(id, {
      isStarred: !(current?.isStarred ?? false),
    });
  }

  async deleteMail(id: string): Promise<void> {
    const trashFolderId = await this.resolveFolderId('trash');
    if (trashFolderId) {
      await this.client().mailMessages.mail.messages.update(id, { folderId: trashFolderId });
      return;
    }
    await this.client().mailMessages.mail.messages.delete(id);
  }

  async sendMail(mail: Partial<MailItem>): Promise<MailItem> {
    const context = await this.resolveRuntimeContext();
    const sentFolderId = context?.folderIds.sent;
    if (!context?.accountId || !sentFolderId) {
      throw new Error(PC_MAIL_SEND_UNAVAILABLE);
    }
    const response = await this.client().mailMessages.mail.messages.create({
      accountId: context.accountId,
      folderId: sentFolderId,
      subject: mail.subject ?? 'Untitled',
      bodyHtml: mail.bodyHtml,
      bodyText: mail.previewText,
      isDraft: false,
    });
    const created = extractMailEntity<MailMessage>(response);
    if (!created?.id) {
      throw new Error(PC_MAIL_SEND_UNAVAILABLE);
    }
    return mapMailMessage(created, 'sent');
  }
}

export const mailService = new SdkworkMailService();

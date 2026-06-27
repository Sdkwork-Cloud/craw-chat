export interface Announcement {
  id: number;
  title: string;
  status: 'published' | 'draft';
  date: string;
  views: number;
  sender: string;
}

export interface GetAnnouncementsResponse {
  data: Announcement[];
  total: number;
  publishedCount: number;
  viewsCount: number;
  draftCount: number;
}

const CONSOLE_ANNOUNCEMENT_CONTRACT_UNAVAILABLE = 'console announcement contract is not available';

class AnnouncementService {
  async getAnnouncements(): Promise<GetAnnouncementsResponse> {
    throw new Error(CONSOLE_ANNOUNCEMENT_CONTRACT_UNAVAILABLE);
  }
}

export const announcementService = new AnnouncementService();

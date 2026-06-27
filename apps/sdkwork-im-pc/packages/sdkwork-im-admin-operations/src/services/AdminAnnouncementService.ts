export interface AdminAnnouncement {
  id: string;
  title: string;
  date: string;
  target: string;
  views: string;
  status: 'delivered' | 'scheduled' | 'draft';
  tag: string;
}

const ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE = 'admin announcement contract is not available';

class AdminAnnouncementService {
  async getAnnouncements(): Promise<AdminAnnouncement[]> {
    throw new Error(ADMIN_ANNOUNCEMENT_CONTRACT_UNAVAILABLE);
  }
}

export const adminAnnouncementService = new AdminAnnouncementService();

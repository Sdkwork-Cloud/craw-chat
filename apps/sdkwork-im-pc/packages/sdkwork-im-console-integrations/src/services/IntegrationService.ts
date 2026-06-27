export interface IntegrationApp {
  id: string;
  name: string;
  type: string;
  desc: string;
  color: string;
  iconType: 'Puzzle' | 'Webhook' | 'Bot';
  status: 'active' | 'disabled';
}

export interface GetAppsResponse {
  data: IntegrationApp[];
  total: number;
}

export const CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE =
  'console integration contract is not available';

class IntegrationService {
  async getApps(_params: { search?: string; status?: string }): Promise<GetAppsResponse> {
    throw new Error(CONSOLE_INTEGRATION_CONTRACT_UNAVAILABLE);
  }
}

export const integrationService = new IntegrationService();

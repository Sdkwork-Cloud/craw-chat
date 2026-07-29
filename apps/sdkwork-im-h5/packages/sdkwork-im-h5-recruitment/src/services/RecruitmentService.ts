export interface CandidateRecord {
  id: string;
  name: string;
  jobTitle: string;
  stage: string;
  date: string;
  avatar?: string;
  experience: string;
  education: string;
}

export class RecruitmentCapabilityUnavailableError extends Error {
  constructor() {
    super("Recruitment is unavailable because its owner SDK is not composed.");
    this.name = "RecruitmentCapabilityUnavailableError";
  }
}

export class RecruitmentService {
  static async getCandidates(): Promise<CandidateRecord[]> {
    throw new RecruitmentCapabilityUnavailableError();
  }

  static async updateCandidateStage(_id: string, _stage: string): Promise<void> {
    throw new RecruitmentCapabilityUnavailableError();
  }

  static async deleteCandidate(_id: string): Promise<void> {
    throw new RecruitmentCapabilityUnavailableError();
  }
}

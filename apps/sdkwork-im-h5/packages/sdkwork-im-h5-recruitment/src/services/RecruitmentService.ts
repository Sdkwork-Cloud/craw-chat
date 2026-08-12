/**
 * Recruitment capability — fail-closed (PRD).
 *
 * Audited as a pure client-side mock with no owner backend SDK. The fake
 * candidate data (incl. placeholder images) and `clawchat_*` storage are
 * removed: every method throws a typed
 * `RecruitmentCapabilityUnavailableError` so any page that reaches this
 * surface shows a typed unavailable state instead of fabricated candidates.
 */

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
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "RecruitmentCapabilityUnavailableError";
  }
}

export class RecruitmentService {
  static async getCandidates(): Promise<CandidateRecord[]> {
    throw new RecruitmentCapabilityUnavailableError("Candidate list");
  }
  static async updateCandidateStage(_id: string, _stage: string): Promise<void> {
    throw new RecruitmentCapabilityUnavailableError("Candidate stage update");
  }
  static async deleteCandidate(_id: string): Promise<void> {
    throw new RecruitmentCapabilityUnavailableError("Candidate deletion");
  }
}

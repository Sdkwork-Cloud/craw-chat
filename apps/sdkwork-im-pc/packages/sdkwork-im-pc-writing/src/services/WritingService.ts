export const PC_WRITING_CONTRACT_UNAVAILABLE = 'pc writing contract is not available';

export interface WritingService {
  generate(prompt: string): Promise<void>;
}

class SdkworkWritingService implements WritingService {
  async generate(_prompt: string): Promise<void> {
    throw new Error(PC_WRITING_CONTRACT_UNAVAILABLE);
  }
}

export const writingService = new SdkworkWritingService();

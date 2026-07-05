import { toast as imToast } from '@sdkwork/im-pc-chat';
import { bootstrapImCommunityPcHost } from '@sdkwork/im-pc-community';

export function bootstrapImCommunityPcIntegration(): void {
  bootstrapImCommunityPcHost({
    toast(message, variant = 'info') {
      const type =
        variant === 'error' ? 'error' : variant === 'success' ? 'success' : 'info';
      imToast(message, type);
    },
  });
}

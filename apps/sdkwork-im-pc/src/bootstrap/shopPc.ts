import { toast as imToast } from '@sdkwork/im-pc-chat';
import { chatService } from '@sdkwork/im-pc-chat';
import { bootstrapImShopPcHost } from '@sdkwork/im-pc-shop';

export function bootstrapImShopPcIntegration(): void {
  bootstrapImShopPcHost({
    toast(message, variant = 'info') {
      const type =
        variant === 'error' ? 'error' : variant === 'success' ? 'success' : 'info';
      imToast(message, type);
    },
    async sendAssistantMessage(recipientId, text, messageType = 'text') {
      await chatService.sendMessage(recipientId, text, messageType);
    },
  });
}

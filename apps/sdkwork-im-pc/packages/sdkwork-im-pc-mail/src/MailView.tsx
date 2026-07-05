import React, { useMemo, useState } from 'react';
import { ArrowLeft } from 'lucide-react';
import { getMailAppSdkClient } from '@sdkwork/mail-pc-core';
import {
  createMailAppServices,
  InboxPage,
  MessagePage,
  type MailAppServices,
} from '@sdkwork/mail-pc-mail';
import { resolveAppSdkBaseUrl } from '@sdkwork/im-pc-core/sdk/appSdkClient';

function createImHostedMailServices(): MailAppServices {
  const client = getMailAppSdkClient(resolveAppSdkBaseUrl());
  return createMailAppServices(client);
}

export const MailView: React.FC = () => {
  const [selectedMessageId, setSelectedMessageId] = useState<string | null>(null);
  const services = useMemo(() => createImHostedMailServices(), []);

  if (selectedMessageId) {
    return (
      <div className="flex h-full flex-col overflow-hidden bg-[#1e1e1e] text-gray-100">
        <header className="flex items-center gap-3 border-b border-white/10 px-4 py-3">
          <button
            type="button"
            className="inline-flex items-center gap-2 rounded-md px-2 py-1 text-sm text-gray-300 hover:bg-white/10"
            onClick={() => setSelectedMessageId(null)}
          >
            <ArrowLeft className="h-4 w-4" />
            Back
          </button>
        </header>
        <div className="flex-1 overflow-auto p-4">
          <MessagePage messageId={selectedMessageId} services={services} />
        </div>
      </div>
    );
  }

  return (
    <div className="h-full overflow-auto bg-[#1e1e1e] p-4 text-gray-100">
      <InboxPage onOpenMessage={setSelectedMessageId} services={services} />
    </div>
  );
};

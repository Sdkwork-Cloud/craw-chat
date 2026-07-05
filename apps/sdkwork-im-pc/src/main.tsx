import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import App from './App.tsx';
import { bootstrapHostAppearanceBridge } from '@sdkwork/im-pc-commons';
import { bootstrapImDrivePcIntegration } from './bootstrap/drivePc';
import { bootstrapImKnowledgebasePcIntegration } from './bootstrap/knowledgebasePc';
import { bootstrapImCoursePcIntegration } from './bootstrap/coursePc';
import { bootstrapImNotaryPcIntegration } from './bootstrap/notaryPc';
import { bootstrapImAgentsPcIntegration } from './bootstrap/agentsPc';
import { bootstrapImVoicePcIntegration } from './bootstrap/voicePc';
import { bootstrapImCommunityPcIntegration } from './bootstrap/communityPc';
import { bootstrapImAiotPcIntegration } from './bootstrap/aiotPc';
import { bootstrapImMailPcIntegration } from './bootstrap/mailPc';
import './index.css';

async function bootstrapImPcCapabilityIntegrations(): Promise<void> {
  bootstrapHostAppearanceBridge();
  await Promise.all([
    bootstrapImNotaryPcIntegration(),
    bootstrapImDrivePcIntegration(),
    bootstrapImCoursePcIntegration(),
    bootstrapImAgentsPcIntegration(),
    bootstrapImKnowledgebasePcIntegration(),
    bootstrapImVoicePcIntegration(),
    Promise.resolve(bootstrapImCommunityPcIntegration()),
    Promise.resolve(bootstrapImMailPcIntegration()),
    Promise.resolve(bootstrapImAiotPcIntegration()),
  ]);
}

function renderImPcApp(): void {
  const root = createRoot(document.getElementById('root')!);
  if (import.meta.env.DEV) {
    root.render(
      <StrictMode>
        <App />
      </StrictMode>,
    );
  } else {
    root.render(<App />);
  }
}

void bootstrapImPcCapabilityIntegrations()
  .then(() => {
    renderImPcApp();
  })
  .catch((error: unknown) => {
    console.error('[sdkwork-im-pc] capability bootstrap failed', error);
    renderImPcApp();
  });

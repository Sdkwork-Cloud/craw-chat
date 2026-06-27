import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import App from './App.tsx';
import { bootstrapHostAppearanceBridge } from '@sdkwork/im-pc-commons';
import { bootstrapImDrivePcIntegration } from './bootstrap/drivePc';
import { bootstrapImKnowledgebasePcIntegration } from './bootstrap/knowledgebasePc';
import { bootstrapImCoursePcIntegration } from './bootstrap/coursePc';
import { bootstrapImNotaryPcIntegration } from './bootstrap/notaryPc';
import { bootstrapImAgentsPcIntegration } from './bootstrap/agentsPc';
import './index.css';

bootstrapHostAppearanceBridge();
bootstrapImNotaryPcIntegration();
bootstrapImDrivePcIntegration();
bootstrapImKnowledgebasePcIntegration();
bootstrapImCoursePcIntegration();
bootstrapImAgentsPcIntegration();

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
